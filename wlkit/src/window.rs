use crate::{keyboard::Keyboard, registry::Wayland, screen::Screen, states::WindowState};

pub struct Window {
    wl: Wayland<WindowState>,
    _kb: Keyboard,
    screen: Screen,
}
impl Window {
    pub fn new(width: i32, height: i32) -> Self {
        let mut wl = Wayland::connect(WindowState::default()).unwrap();
        let _kb = Keyboard::new(&mut wl);
        let screen = Screen::new(&mut wl, width, height);
        Window { wl, _kb, screen }
    }

    pub fn is_alive(&self) -> bool {
        self.screen.is_alive()
    }

    pub fn keys(&self) -> &[u32] {
        self.wl.state().keys()
    }

    pub fn screen(&mut self) -> &mut Screen {
        &mut self.screen
    }

    pub fn pool(&mut self) -> usize {
        self.wl.roundtrip().unwrap()
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new(1920, 1080)
    }
}
