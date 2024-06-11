pub mod graphics;
pub mod print;
pub mod idt;
pub mod e9;
pub mod info;

pub fn init() {
    idt::init();
}