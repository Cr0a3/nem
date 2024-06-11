use core::arch::asm;
use crate::println;

#[panic_handler]
pub fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    println!("{}", _info);
    hcf();
}

pub fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
