use wayland_client::protocol::{
    wl_buffer::WlBuffer, wl_compositor::WlCompositor, wl_seat::WlSeat, wl_shm::WlShm,
    wl_shm_pool::WlShmPool, wl_surface::WlSurface,
};
use wayland_protocols::xdg::shell::client::{
    xdg_surface::XdgSurface, xdg_toplevel::XdgToplevel, xdg_wm_base::XdgWmBase,
};

#[derive(Default)]
pub(crate) struct WindowState {
    keys: [bool; 16],
}
impl WindowState {
    pub fn keys(&self) -> [bool; 16] {
        self.keys
    }
}

wl_state!(WindowState);
wl_no_events!(WindowState; WlSeat, WlCompositor, WlSurface, WlBuffer, WlShm, WlShmPool);
wl_no_events!(WindowState; XdgWmBase, XdgToplevel);
