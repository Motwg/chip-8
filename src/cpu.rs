use beep::beep;

fn word_nibble(word: u8, idx: u8) -> u8 {
    (word & (0xF << (4 * idx))) >> (4 * idx)
}

fn join_arr(arr: &[u8]) -> u16 {
    arr.iter().fold(0u16, |acc, &v| (acc << 4) + u16::from(v))
}

fn u8_to_bool(byte: u8, pos: u8) -> bool {
    (byte >> (7 - pos)) & 1 == 1
}

const START: usize = 0x200;

pub struct Cpu {
    pc: u16,
    i: u16,
    v: [u8; 16],
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    memory: [u8; 4096],
    framebuffer: [bool; 64 * 32],
}
impl Cpu {
    pub fn new(rom: &[u8]) -> Self {
        let mut memory = [0u8; 4096];
        memory[START..START + rom.len()].copy_from_slice(rom);
        Cpu {
            pc: START as u16,
            i: 0x0,
            v: [0; 16],
            stack: Vec::with_capacity(16),
            delay_timer: 0,
            sound_timer: 0,
            memory,
            framebuffer: [false; 64 * 32],
        }
    }

    pub fn work(&mut self) {
        let first: u8 = self.memory[usize::from(self.pc)];
        let second: u8 = self.memory[usize::from(self.pc + 1)];
        let nibbles: (u8, u8, u8, u8) = (
            word_nibble(first, 1),
            word_nibble(first, 0),
            word_nibble(second, 1),
            word_nibble(second, 0),
        );

        print!("{} - {first:02X}{second:02X}: ", self.pc);
        self.pc += 2;
        match nibbles {
            // (0x0, 0x0, 0x0, 0x0) => println!("subroutine"),
            (0x0, 0x0, 0xE, 0x0) => Self::clear(self),
            (0x0, 0x0, 0xE, 0xE) => Self::return_subroutine(self),
            (0x1, a, b, c) => Self::jump(self, join_arr(&[a, b, c])),
            (0x2, a, b, c) => Self::subroutine(self, join_arr(&[a, b, c])),
            (0x3, x, a, b) => Self::skip_if(self, x as usize, join_arr(&[a, b]) as u8),
            (0x4, x, a, b) => Self::skip_ifn(self, x as usize, join_arr(&[a, b]) as u8),
            (0x5, x, y, 0) => Self::skip_if_eq(self, x as usize, y as usize),
            (0x6, x, a, b) => Self::set_register(self, x as usize, join_arr(&[a, b]) as u8),
            (0x7, x, a, b) => Self::add_register(self, x as usize, join_arr(&[a, b]) as u8),
            (0x8, x, y, 0x0) => Self::set(self, x as usize, y as usize),
            (0x8, x, y, 0x1) => Self::bitwise_or(self, x as usize, y as usize),
            (0x8, x, y, 0x2) => Self::bitwise_and(self, x as usize, y as usize),
            (0x8, x, y, 0x3) => Self::bitwise_xor(self, x as usize, y as usize),
            (0x8, x, y, 0x4) => Self::add(self, x as usize, y as usize),
            (0x8, x, y, 0x5) => Self::subtract(self, x as usize, y as usize),
            (0x8, x, _, 0x6) => Self::right_shift(self, x as usize),
            (0x8, x, y, 0x7) => Self::subtract_inv(self, x as usize, y as usize),
            (0x8, x, _, 0xE) => Self::left_shift(self, x as usize),
            (0x9, x, y, 0x0) => Self::skip_ifn_eq(self, x as usize, y as usize),
            (0xA, a, b, c) => Self::set_index_register(self, join_arr(&[a, b, c])),
            (0xB, a, b, c) => Self::offset_jump(self, join_arr(&[a, b, c])),
            (0xD, x, y, n) => Self::draw(self, x, y, n),
            (0xF, x, 0x0, 0x7) => Self::get_delay(self, x as usize),
            (0xF, x, 0x1, 0x5) => Self::set_delay(self, x as usize),
            (0xF, x, 0x1, 0x8) => Self::set_sound(self, x as usize),
            (0xF, x, 0x1, 0xE) => Self::add_i(self, x as usize),
            (0xF, x, 0x3, 0x3) => Self::store_bcd(self, x as usize),
            (0xF, x, 0x5, 0x5) => Self::store_reg(self, x as usize),
            (0xF, x, 0x6, 0x5) => Self::fill_reg(self, x as usize),
            _ => {
                println!("Invalid instruction");
                panic!()
            }
        };
    }

    pub fn framebuffer(&self) -> &[bool; 64 * 32] {
        &self.framebuffer
    }

    pub fn tick(&mut self) {
        if self.sound_timer > 0 {
            let _ = beep(0);
        }
        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);
    }

    fn jump(&mut self, idx: u16) {
        println!("jump to {idx}");
        self.pc = idx;
    }

    fn offset_jump(&mut self, idx: u16) {
        self.pc = idx + u16::from(self.v[0]);
        println!("jump to {}", self.pc);
    }

    fn subroutine(&mut self, idx: u16) {
        self.stack.push(self.pc);
        self.pc = idx;
    }

    fn return_subroutine(&mut self) {
        self.pc = self.stack.pop().unwrap();
    }

    fn skip_if(&mut self, v: usize, nn: u8) {
        if self.v[v] == nn {
            self.pc += 2
        }
    }

    fn skip_ifn(&mut self, v: usize, nn: u8) {
        if self.v[v] != nn {
            self.pc += 2
        }
    }

    fn skip_if_eq(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += 2
        }
    }

    fn skip_ifn_eq(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += 2
        }
    }

    fn set_register(&mut self, v: usize, x: u8) {
        println!("set register {v} to {x}");
        self.v[v] = x;
    }

    fn add_register(&mut self, v: usize, x: u8) {
        println!("add {x} to register {v}");
        self.v[v] = self.v[v].wrapping_add(x);
    }

    fn set(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    fn bitwise_or(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
    }

    fn bitwise_and(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
    }

    fn bitwise_xor(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
    }

    fn add(&mut self, x: usize, y: usize) {
        let (sum, overflow) = self.v[x].overflowing_add(self.v[y]);
        self.v[x] = sum;
        self.v[0xF] = u8::from(overflow);
    }

    fn subtract(&mut self, x: usize, y: usize) {
        let (diff, overflow) = self.v[x].overflowing_sub(self.v[y]);
        self.v[x] = diff;
        self.v[0xF] = u8::from(overflow) ^ 1;
    }

    fn subtract_inv(&mut self, x: usize, y: usize) {
        let (diff, overflow) = self.v[y].overflowing_sub(self.v[x]);
        self.v[x] = diff;
        self.v[0xF] = u8::from(overflow) ^ 1;
    }

    fn right_shift(&mut self, x: usize) {
        self.v[0xF] = self.v[x] & 1;
        self.v[x] >>= 1;
    }

    fn left_shift(&mut self, x: usize) {
        self.v[0xF] = self.v[x] & 128;
        self.v[x] <<= 1;
    }

    fn set_index_register(&mut self, x: u16) {
        println!("set index register to {x}");
        self.i = x;
    }

    fn clear(&mut self) {
        self.framebuffer = [false; 64 * 32];
    }

    fn draw(&mut self, x: u8, y: u8, n: u8) {
        println!("draw {x} {y} {n}");
        let vx = self.v[usize::from(x)] % 64;
        let vy = self.v[usize::from(y)] % 32;
        self.v[0xF] = 0;
        for h in 0..n {
            let py = usize::from(vy) + usize::from(h);
            if py >= 32 {
                break;
            }

            let byte = self.memory[self.i as usize + h as usize];
            for w in 0..8 {
                let px = usize::from(vx) + usize::from(w);
                if px >= 64 {
                    break;
                }

                let idx = py * 64 + px;
                if self.framebuffer[idx] & u8_to_bool(byte, w) {
                    self.v[0xF] = 1;
                }
                self.framebuffer[idx] ^= u8_to_bool(byte, w);
            }
        }
    }

    fn add_i(&mut self, x: usize) {
        if x != 0xF {
            self.i += self.v[x] as u16;
        }
    }

    fn skip_if_key(&mut self, x: usize) {}

    fn skip_ifn_key(&mut self, x: usize) {}

    fn get_delay(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
    }

    fn set_delay(&mut self, x: usize) {
        self.delay_timer = self.v[x];
    }

    fn set_sound(&mut self, x: usize) {
        self.sound_timer = self.v[x];
    }

    fn store_bcd(&mut self, x: usize) {
        let decimal = self.v[x];
        self.memory[self.i as usize] = decimal.div_euclid(100);
        self.memory[self.i as usize + 1] = decimal.div_euclid(10) % 10;
        self.memory[self.i as usize + 2] = decimal.div_euclid(100);
        println!(
            "Storing {} into {:?}",
            decimal,
            &self.memory[self.i as usize..=self.i as usize + 2]
        )
    }

    fn store_reg(&mut self, x: usize) {
        for offset in 0..=x {
            self.memory[self.i as usize + offset] = self.v[offset];
        }
    }

    fn fill_reg(&mut self, x: usize) {
        for offset in 0..=x {
            self.v[offset] = self.memory[self.i as usize + offset];
        }
    }
}
