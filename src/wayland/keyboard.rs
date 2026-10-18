use wayland_client::protocol::wl_seat::WlSeat;
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::{
    zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1,
    zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
};

use crate::{
    exceptions::e_handler,
    wayland::registry::{Ignore, State, Wayland},
    wl_no_events,
};

wl_no_events!(WlSeat, ZwpVirtualKeyboardManagerV1);

pub struct Keyboard;
impl Keyboard {
    pub fn new() {}
}

pub fn run() {
    let mut state = State;
    let wl = Wayland::<State>::connect().unwrap();

    let seat: WlSeat = wl
        .get(Ignore)
        .unwrap_or_else(|e| e_handler("Wayland seat not available", e, 3));
    let manager: ZwpVirtualKeyboardManagerV1 = wl
        .get(Ignore)
        .unwrap_or_else(|e| e_handler("Wayland keyboard manager not available", e, 4));
}
