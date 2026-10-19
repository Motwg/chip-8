use crate::{
    exceptions::e_handler,
    wayland::registry::{Ignore, State, Wayland},
    wl_no_events,
};
use rustix::fs::{MemfdFlags, memfd_create};
use std::{
    fs::File,
    io::Write,
    os::fd::AsFd,
    thread::sleep,
    time::{Duration, Instant},
};
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::{
    zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1,
    zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
};

wl_no_events!(WlSeat, ZwpVirtualKeyboardManagerV1, ZwpVirtualKeyboardV1);

const KEYMAP: &str = "xkb_keymap {
    xkb_keycodes { include \"evdev+aliases(qwerty)\" };
    xkb_types    { include \"complete\" };
    xkb_compat   { include \"complete\" };
    xkb_symbols  { include \"pc+pl+inet(evdev)\" };
};\0";

pub struct VKeyboard {
    wl: Wayland<State>,
    keyboard: ZwpVirtualKeyboardV1,
}
impl VKeyboard {
    pub fn new() -> Self {
        let wl = Wayland::connect(State).unwrap();

        let seat: WlSeat = wl
            .get(Ignore)
            .unwrap_or_else(|e| e_handler("Wayland seat not available", e, 3));
        let manager: ZwpVirtualKeyboardManagerV1 = wl
            .get(Ignore)
            .unwrap_or_else(|e| e_handler("Wayland keyboard manager not available", e, 4));
        let keyboard = manager.create_virtual_keyboard(&seat, wl.qh(), Ignore);

        let mut bytes = KEYMAP.as_bytes().to_vec();
        let mut file = memfd_create("keymap", MemfdFlags::CLOEXEC)
            .map(File::from)
            .unwrap();
        file.write_all(&bytes).unwrap();
        keyboard.keymap(1, file.as_fd(), bytes.len() as u32);
        VKeyboard { wl, keyboard }
    }

    pub fn write(&mut self, text: Vec<u32>) {
        self.wl.roundtrip().unwrap();
        let start = Instant::now();
        let now = || start.elapsed().as_millis() as u32;
        for code in text {
            self.keyboard.key(now(), code, 1); // pressed
            self.wl.flush().unwrap();
            sleep(Duration::from_millis(5));
            self.keyboard.key(now(), code, 0); // released
            self.wl.flush().unwrap();
            sleep(Duration::from_millis(5));
        }
        self.wl.roundtrip().unwrap();
    }
}
