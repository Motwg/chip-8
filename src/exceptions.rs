use std::{fmt, process};

pub fn e_handler<E: fmt::Display>(msg: &str, e: E, code: i8) -> ! {
    eprintln!("[{code}] {msg}: {e}");
    process::exit(code.into());
}
