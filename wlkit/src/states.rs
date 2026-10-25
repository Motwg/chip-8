use wayland_client::protocol::{
    wl_buffer::WlBuffer, wl_compositor::WlCompositor, wl_seat::WlSeat, wl_shm::WlShm,
    wl_shm_pool::WlShmPool, wl_surface::WlSurface,
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
