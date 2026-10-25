use crate::registry::{Ignore, Wayland};
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

struct State;
wl_state!(State);
wl_no_events!(
    State;
    WlSeat,
    ZwpVirtualKeyboardManagerV1,
    ZwpVirtualKeyboardV1
);

const KEYMAP: &str = "xkb_keymap {
    xkb_keycodes { include \"evdev+aliases(qwerty)\" };
    xkb_types    { include \"complete\" };
    xkb_compat   { include \"complete\" };
    xkb_symbols  { include \"pc+pl+inet(evdev)\" };
};\0";

pub struct VKeyboard {
    wl: Wayland<State>,
    keyboard: ZwpVirtualKeyboardV1,
    start: Instant,
}
impl VKeyboard {
    pub fn new() -> Self {
        let mut wl = Wayland::connect(State).unwrap();
        let seat: WlSeat = wl.get(Ignore).unwrap();
        let manager: ZwpVirtualKeyboardManagerV1 = wl.get(Ignore).unwrap();
        let keyboard = manager.create_virtual_keyboard(&seat, wl.qh(), Ignore);

        let bytes = KEYMAP.as_bytes().to_vec();
        let mut file = memfd_create("keymap", MemfdFlags::CLOEXEC)
            .map(File::from)
            .unwrap();
        file.write_all(&bytes).unwrap();
        keyboard.keymap(1, file.as_fd(), bytes.len() as u32);
        wl.roundtrip().unwrap();
        VKeyboard {
            wl,
            keyboard,
            start: Instant::now(),
        }
    }

    pub fn write(&mut self, text: Vec<u32>) {
        self.wl.roundtrip().unwrap();
        let now = || self.start.elapsed().as_millis() as u32;
        for code in text {
            self.keyboard.key(now(), code, 1);
            self.wl.flush().unwrap();
            sleep(Duration::from_millis(5));
            self.keyboard.key(now(), code, 0);
            self.wl.flush().unwrap();
            sleep(Duration::from_millis(5));
        }
        self.wl.roundtrip().unwrap();
    }
}
