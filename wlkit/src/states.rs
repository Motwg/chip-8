use wayland_client::{
    WEnum,
    protocol::{
        wl_buffer::WlBuffer, wl_compositor::WlCompositor, wl_keyboard::KeyState, wl_seat::WlSeat,
        wl_shm::WlShm, wl_shm_pool::WlShmPool, wl_surface::WlSurface,
    },
};
use wayland_protocols::xdg::shell::client::{xdg_toplevel::XdgToplevel, xdg_wm_base::XdgWmBase};

#[derive(Default)]
pub(crate) struct WindowState {
    keys: Vec<u32>,
}
impl WindowState {
    pub fn keys(&self) -> &Vec<u32> {
        &self.keys
    }

    pub fn enter(&mut self, key: u32, key_state: WEnum<KeyState>) {
        match key_state {
            WEnum::Value(KeyState::Pressed) => {
                self.keys.push(key);
            }
            WEnum::Value(KeyState::Released) => {
                self.keys.retain(|&x| x != key);
            }
            _ => {}
        }
    }
}

wl_state!(WindowState);
wl_no_events!(WindowState; WlSeat, WlCompositor, WlSurface, WlBuffer, WlShm, WlShmPool);
wl_no_events!(WindowState; XdgWmBase, XdgToplevel);
