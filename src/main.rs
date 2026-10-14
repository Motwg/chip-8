mod cpu;

use cpu::Cpu;
use minifb::{Window, WindowOptions};
use std::{env, fmt, fs, process};

const W: usize = 64;
const H: usize = 32;
const SCALE: usize = 25;

fn main() {
    let path: String = env::args()
        .nth(1)
        .expect("Usage: cargo run -- <path-to-rom>");
    let rom: Vec<u8> = fs::read(&path)
        .unwrap_or_else(|e| err(&format!("Could not read {path}"), e, 1))
        .into_iter()
        .collect();
    let mut cpu = Cpu::new(&rom);
    let mut window = Window::new("Chip-8", W * SCALE, H * SCALE, WindowOptions::default()).unwrap();
    let mut buffer = vec![0u32; W * SCALE * H * SCALE];

    window.set_target_fps(60);
    while window.is_open() {
        for _ in 0..10 {
            cpu.work();
        }
        cpu.tick();
        let fb = cpu.framebuffer();
        for y in 0..H * SCALE {
            for x in 0..W * SCALE {
                let on = fb[(y / SCALE) * W + (x / SCALE)];
                buffer[y * W * SCALE + x] = if on { 0xFFA0FF } else { 0 };
            }
        }

        window
            .update_with_buffer(&buffer, W * SCALE, H * SCALE)
            .unwrap();
    }
}

fn err<E: fmt::Display>(msg: &str, e: E, code: i8) -> ! {
    eprintln!("[{code}] {msg}: {e}");
    process::exit(code.into());
}
