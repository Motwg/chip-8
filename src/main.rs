mod cpu;
mod exceptions;

use cpu::Cpu;
use exceptions::e_handler;
use std::{
    env, fs,
    thread::sleep,
    time::{Duration, Instant},
};
use wlkit::Window;

const W: usize = 64;
const H: usize = 32;
const SCALE: usize = 10;

fn main() {
    let path: String = env::args()
        .nth(1)
        .expect("Usage: cargo run -- <path-to-rom>");
    let mut window = Window::new((W * SCALE) as i32, (H * SCALE) as i32);
    let rom: Vec<u8> = fs::read(&path)
        .unwrap_or_else(|e| e_handler(&format!("Could not read {path}"), e, 1))
        .into_iter()
        .collect();
    let mut cpu = Cpu::new(&rom);
    let mut buffer = vec![0u32; W * SCALE * H * SCALE];
    let frame = Duration::from_secs_f32(1.0 / 60.0);
    window.screen().fill(0x000000ff);

    while window.is_alive() {
        let start = Instant::now();
        window.pool();
        cpu.tick(window.keys());
        for _ in 0..10 {
            cpu.work();
        }
        let fb = cpu.framebuffer();
        for y in 0..H * SCALE {
            for x in 0..W * SCALE {
                let on = fb[(y / SCALE) * W + (x / SCALE)];
                buffer[y * W * SCALE + x] = if on { 0x00FFA0A0 } else { 0 };
            }
        }
        window
            .screen()
            .update_with_buffer(&buffer)
            .expect("Buffer couldn't be updated");
        sleep(frame.saturating_sub(start.elapsed()));
    }
}
