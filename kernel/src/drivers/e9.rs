use x86_64::instructions::port::Port;

pub fn debug(msg: &str) {
    let mut e9 = Port::<u8>::new(0xE9);

    for chr in msg.chars() {
        unsafe {
            e9.write(chr as u8);
        }
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ($crate::drivers::e9::_debug(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! debugln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::debug!("{}\n", format_args!($($arg)*)));
}

pub struct FMT {}
impl FMT { pub fn new() -> Self { Self {} }}

use core::fmt::{self, Write};

impl<'a> fmt::Write for FMT {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        debug(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _debug(args: fmt::Arguments) {
    FMT::new().write_fmt(args);
}

pub struct Logger {}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => (
        $crate::debugln!("{} [ DEBUG ] {}{}", "\x1b[38;2;90m", "\x1b[0m", format_args!($($arg)*))
    );
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => (
        $crate::debugln!("{} [ INFO  ] {}{}", "\x1b[34m", "\x1b[0m", format_args!($($arg)*))
    );
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (
        $crate::debugln!("{} [ WARN  ] {}{}", "\x1b[33m", "\x1b[0m", format_args!($($arg)*));
    );
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        $crate::debugln!("{} [ ERROR ] {}{}", "\x1b[31m", "\x1b[0m", format_args!($($arg)*))
    );
}