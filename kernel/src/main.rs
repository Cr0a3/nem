#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![allow(static_mut_refs)]

pub mod panic;
pub mod ui;
pub mod drivers;
use core::arch::asm;

use drivers::graphics::*;
use panic::*;

use limine::BaseRevision;

#[used]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());

    drivers::init();

    asm!("int 80");

    info!("after interrupt");

    hcf();
}