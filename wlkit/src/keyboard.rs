use crate::{
    registry::{Ignore, Wayland},
    states::WindowState,
};
use wayland_client::{
    Connection, Dispatch, QueueHandle,
    protocol::{
        wl_keyboard::{self, WlKeyboard},
        wl_seat::WlSeat,
    },
};

impl Dispatch<WlKeyboard, ()> for WindowState {
    fn event(
        state: &mut Self,
        _proxy: &WlKeyboard,
        event: wl_keyboard::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        match event {
            wl_keyboard::Event::Keymap {
                format: _,
                fd: _,
                size: _,
            } => {}
            wl_keyboard::Event::Enter {
                serial: _,
                surface: _,
                keys: _,
            } => {}
            wl_keyboard::Event::Leave { serial: _, surface } => {
                println!("Leave");
                surface.destroy();
            }
            wl_keyboard::Event::Key {
                serial: _,
                time: _,
                key,
                state: key_state,
            } => {
                println!("Key {:?} - {:?}", key, key_state);
                state.enter(key, key_state);
                println!("{:?}", state.keys())
            }
            wl_keyboard::Event::Modifiers {
                serial: _serial,
                mods_depressed: _mods_depressed,
                mods_latched: _mods_latched,
                mods_locked: _mods_locker,
                group: _group,
            } => println!("Modifiers"),
            wl_keyboard::Event::RepeatInfo { rate, delay } => {
                println!("RepeatInfo {} | {}", rate, delay)
            }
            _ => {}
        }
    }
}

pub struct Keyboard {
    _kb: WlKeyboard,
}
impl Keyboard {
    pub fn new(wl: &mut Wayland<WindowState>) -> Self {
        let seat: WlSeat = wl.get(Ignore).unwrap();
        let _kb: WlKeyboard = seat.get_keyboard(wl.qh(), ());
        let _ = wl.roundtrip().unwrap();
        Keyboard { _kb }
    }
}
