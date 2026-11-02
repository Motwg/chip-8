use crate::{keyboard::Keyboard, registry::Wayland, screen::Screen, states::WindowState};

pub struct Window {
    wl: Wayland<WindowState>,
    kb: Keyboard,
    screen: Screen,
}
impl Window {
    pub fn new(width: i32, height: i32) -> Self {
        let mut wl = Wayland::connect(WindowState::default()).unwrap();
        let kb = Keyboard::new(&mut wl);
        let screen = Screen::new(&mut wl, width, height);
        Window { wl, kb, screen }
    }

    pub fn is_alive(&self) -> bool {
        self.screen.is_alive()
    }

    pub fn keys(&self) -> &Vec<u32> {
        self.wl.state().keys()
    }

    pub fn run(&mut self) {
        self.screen.fill(0x000000ff);
        self.screen.write(0xF1000074, 000, 400, 100, 44);
        self.screen.write(0x0000F364, 000, 600, 200, 84);
        self.screen.write(0x00F200A4, 000, 200, 300, 74);

        self.screen.write(0xF100f3D4, 000, 200, 400, 74);
        self.screen.write(0xf1F20094, 000, 200, 500, 74);
        self.screen.write(0x00F2f364, 000, 200, 600, 74);
        loop {
            let n = self.wl.roundtrip().unwrap();
            if n > 0 {
                println!("Events: {}", n);
            }
            // println!("{:?}", self.wl.state().keys());
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new(1920, 1080)
    }
}
