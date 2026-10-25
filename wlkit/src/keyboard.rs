use crate::{
    registry::{Ignore, Wayland},
    states::WindowState,
};
use rustix::fs::{MemfdFlags, memfd_create};
use std::{default::Default, derive};
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
            wl_keyboard::Event::Keymap { format, fd, size } => {
                println!("Keymap {:?} | {:?} | {:?}", format, fd, size);
            }
            wl_keyboard::Event::Enter {
                serial,
                surface,
                keys,
            } => println!("Enter {:?} vs {:?}", state.keys(), keys),
            wl_keyboard::Event::Leave { serial, surface } => {
                println!("Leave")
            }
            wl_keyboard::Event::Key {
                serial,
                time,
                key,
                state: key_state,
            } => println!("Key {:?} - {:?}", key, key_state),
            wl_keyboard::Event::Modifiers {
                serial,
                mods_depressed,
                mods_latched,
                mods_locked,
                group,
            } => println!("Modifiers"),
            wl_keyboard::Event::RepeatInfo { rate, delay } => {
                println!("RepeatInfo {} | {}", rate, delay)
            }
            _ => println!("Default"),
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
