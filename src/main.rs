mod cpu;
mod exceptions;
mod wayland;

use cpu::Cpu;
use exceptions::e_handler;
use minifb::{Key, Window, WindowOptions};
use std::{env, fs};

const W: usize = 64;
const H: usize = 32;
const SCALE: usize = 5;
const KEYMAP: [Key; 16] = [
    Key::X,
    Key::Key1,
    Key::Key2,
    Key::Key3, // 0 1 2 3
    Key::Q,
    Key::W,
    Key::E,
    Key::A, // 4 5 6 7
    Key::S,
    Key::D,
    Key::Z,
    Key::C, // 8 9 A B
    Key::Key4,
    Key::R,
    Key::F,
    Key::V, // C D E F
];

fn main() {
    let path: String = env::args()
        .nth(1)
        .expect("Usage: cargo run -- <path-to-rom>");
    wayland::keyboard::run();
    let rom: Vec<u8> = fs::read(&path)
        .unwrap_or_else(|e| e_handler(&format!("Could not read {path}"), e, 1))
        .into_iter()
        .collect();
    let mut cpu = Cpu::new(&rom);
    let mut window = Window::new("Chip-8", W * SCALE, H * SCALE, WindowOptions::default()).unwrap();
    let mut keys = [false; 16];
    let mut buffer = vec![0u32; W * SCALE * H * SCALE];
    let mut pressed = window.get_keys();

    window.set_target_fps(1);
    while window.is_open() {
        // for (i, &k) in KEYMAP.iter().enumerate() {
        //     keys[i] = window.is_key_down(k);
        // }
        // println!("{keys:?}");
        pressed = window.get_keys();
        if !pressed.is_empty() {
            println!("{pressed:?}");
        }
        // for _ in 0..10 {
        //     cpu.work();
        // }
        // cpu.tick(keys);
        // let fb = cpu.framebuffer();
        // for y in 0..H * SCALE {
        //     for x in 0..W * SCALE {
        //         let on = fb[(y / SCALE) * W + (x / SCALE)];
        //         buffer[y * W * SCALE + x] = if on { 0xFFA0A0 } else { 0 };
        //     }
        // }

        window
            .update_with_buffer(&buffer, W * SCALE, H * SCALE)
            .unwrap();
    }
}

fn key_mapper(keys: Vec<Key>) -> Vec<u8> {
    // minifb::Window::get_keys(&self)
    keys.iter()
        .filter_map(|key| match key {
            Key::Key1 => Some(0x1),
            Key::Key2 => Some(0x2),
            Key::Key3 => Some(0x3),
            Key::Q => Some(0x4),
            Key::W => Some(0x5),
            Key::E => Some(0x6),
            Key::A => Some(0x7),
            Key::S => Some(0x8),
            Key::D => Some(0x9),
            Key::Z => Some(0xA),
            Key::C => Some(0xB),
            Key::Key4 => Some(0xC),
            Key::R => Some(0xD),
            Key::F => Some(0xE),
            Key::V => Some(0xF),
            Key::X => Some(0x0),
            _ => None,
        })
        .collect()
}
