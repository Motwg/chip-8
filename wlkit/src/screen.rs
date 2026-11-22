use std::{
    fs::File,
    io::Result,
    os::{fd::AsFd, unix::fs::FileExt},
};

use rustix::fs::{MemfdFlags, memfd_create};
use wayland_client::{
    Connection, Dispatch, Proxy, QueueHandle,
    protocol::{
        wl_buffer::WlBuffer,
        wl_compositor::WlCompositor,
        wl_shm::{Format, WlShm},
        wl_shm_pool::WlShmPool,
        wl_surface::WlSurface,
    },
};
use wayland_protocols::xdg::shell::client::{
    xdg_surface::{self, XdgSurface},
    xdg_wm_base::{self, XdgWmBase},
};

use crate::{
    registry::{Ignore, Wayland},
    states::WindowState,
};

impl Dispatch<XdgSurface, ()> for WindowState {
    fn event(
        _state: &mut Self,
        proxy: &XdgSurface,
        event: <XdgSurface as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        match event {
            xdg_surface::Event::Configure { serial } => proxy.ack_configure(serial),
            _ => {}
        }
    }
}

impl Dispatch<XdgWmBase, ()> for WindowState {
    fn event(
        _state: &mut Self,
        proxy: &XdgWmBase,
        event: <XdgWmBase as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        match event {
            xdg_wm_base::Event::Ping { serial } => {
                proxy.pong(serial);
            }
            _ => {}
        }
    }
}

struct Damager {
    x: u32,
    y: u32,
    dx: u32,
    dy: u32,
}

impl Damager {
    fn full(width: u32, height: u32) -> Self {
        Self {
            x: 0,
            y: 0,
            dx: width,
            dy: height,
        }
    }
}

pub struct Screen {
    surface: WlSurface,
    _buffer: WlBuffer,
    _file: File,
    _width: u32,
    _height: u32,
}

impl Screen {
    pub fn new(wl: &mut Wayland<WindowState>, width: i32, height: i32) -> Self {
        let comp: WlCompositor = wl.get(Ignore).unwrap();
        let surface: WlSurface = comp.create_surface(wl.qh(), Ignore);

        let xdg_base: XdgWmBase = wl.get(Ignore).unwrap();
        let xdg_surface = xdg_base.get_xdg_surface(&surface, wl.qh(), ());
        let _top = xdg_surface.get_toplevel(wl.qh(), Ignore);
        surface.commit();
        let _ = wl.roundtrip().unwrap();

        let shm: WlShm = wl.get(Ignore).unwrap();
        let file = memfd_create("screen", MemfdFlags::CLOEXEC)
            .map(File::from)
            .unwrap();
        file.set_len((width * height * 4) as u64)
            .expect("Could not create buffer of enough size");
        let pool: WlShmPool = shm.create_pool(file.as_fd(), width * height * 4, wl.qh(), Ignore);
        let buffer: WlBuffer = pool.create_buffer(
            0,
            width,
            height,
            width * 4,
            Format::Argb8888,
            wl.qh(),
            Ignore,
        );
        let _ = surface.attach(Some(&buffer), 0, 0);
        surface.commit();
        let _ = wl.roundtrip().unwrap();
        Screen {
            surface,
            _buffer: buffer,
            _file: file,
            _width: width as u32,
            _height: height as u32,
        }
    }

    fn damage_full(&self) -> Damager {
        Damager::full(self._width, self._height)
    }

    fn commit(&self, damager: Damager) {
        self.surface.attach(Some(&self._buffer), 0, 0);
        self.surface.damage_buffer(
            damager.x as i32,
            damager.y as i32,
            damager.dx as i32,
            damager.dy as i32,
        );
        self.surface.commit();
    }

    pub fn write(&mut self, color: u32, x: u32, dx: u32, y: u32, dy: u32) -> Result<()> {
        let clamp = dx.min(self._width.saturating_sub(x));
        let bytes = color.to_le_bytes().repeat(clamp as usize);
        let rows = (dy).min(self._height.saturating_sub(y));
        for y_pos in y..y + rows {
            self._file
                .write_all_at(&bytes, ((y_pos * self._width + x) * 4) as u64)?;
        }
        self.commit(Damager {
            x,
            y,
            dx: clamp,
            dy: rows,
        });
        Ok(())
    }

    pub fn fill(&mut self, color: u32) -> Result<()> {
        let bytes = color
            .to_le_bytes()
            .repeat(self._width as usize * self._height as usize);
        self._file.write_all_at(&bytes, 0)?;
        self.commit(self.damage_full());
        Ok(())
    }

    pub fn update_with_buffer(&mut self, buffer: &[u32]) -> Result<()> {
        let bytes: Vec<u8> = buffer
            .iter()
            .flat_map(|dword| dword.to_le_bytes())
            .collect();
        self._file.write_all_at(&bytes, 0)?;
        self.commit(self.damage_full());
        Ok(())
    }

    pub fn is_alive(&self) -> bool {
        self.surface.is_alive()
    }
}
