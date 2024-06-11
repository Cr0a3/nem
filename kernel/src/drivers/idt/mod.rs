use x86_64::instructions::interrupts;
use x86_64::set_general_handler;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use pic8259::ChainedPics;
use crate::{debug, error, hcf, info, println};
pub mod exceptions;

pub const PIC_1_OFFSET: u8 = 32;

pub static mut PICS: ChainedPics = unsafe {ChainedPics::new(PIC_1_OFFSET, PIC_1_OFFSET + 8)};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init() {
    unsafe {
        set_general_handler!(&mut IDT, idt_handler, 0..=255);

        IDT[IRQS::Timer.as_u8()].set_handler_fn(timer_handler);

        IDT.load();
        interrupts::enable();

        
        PICS.initialize();
    }
}

pub fn idt_handler(stack: InterruptStackFrame, int: u8, err: Option<u64>) {
    info!("int {}", int);
    info!("stack: {:#?}", stack);

    if let Some(err) = err {
        error!("Error {}", err);
    }
}

enum  IRQS {
    Timer
}

impl IRQS {
    pub fn as_u8(&self) -> u8 {
        match &self {
            IRQS::Timer => 0 + PIC_1_OFFSET,
        }
    }
}

extern "x86-interrupt" fn timer_handler(_stack: InterruptStackFrame) {

    info!("sucess");

    unsafe {
        PICS.notify_end_of_interrupt(PIC_1_OFFSET);
    }
}