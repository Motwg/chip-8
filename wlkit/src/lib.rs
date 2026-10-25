#[macro_use]
mod registry;
mod keyboard;
mod screen;
mod states;
mod vkeyboard;
mod window;

pub use vkeyboard::VKeyboard;
pub use window::Window;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
