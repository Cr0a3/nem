use crate::{colors, hcf, ui::window::Window, GraphicsDriver};

use super::IDT;

pub fn init() {
    unsafe {
        IDT.divide_error.set_handler_fn(divide_error_handler);
        IDT.debug.set_handler_fn(debug_error_handler);
        IDT.non_maskable_interrupt.set_handler_fn(non_maskable_interrupt_error_handler);
        IDT.breakpoint.set_handler_fn(breakpoint_handler);
        IDT.overflow.set_handler_fn(overflow_error_handler);
        IDT.bound_range_exceeded.set_handler_fn(bound_range_exceeded_error_handler);
        IDT.invalid_opcode.set_handler_fn(invalid_opcode_error_handler);
        IDT.device_not_available.set_handler_fn(device_not_avialable_error);
        IDT.invalid_tss.set_handler_fn(invalid_tss_handler);
        IDT.segment_not_present.set_handler_fn(segment_not_presented_error_handler);
        IDT.stack_segment_fault.set_handler_fn(stack_segmentation_fault_error_handler);
        IDT.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        IDT.page_fault.set_handler_fn(page_fault_error_handler);
        IDT.x87_floating_point.set_handler_fn(x87_floating_point_error_handler);
        IDT.alignment_check.set_handler_fn(alignment_check_error_handler);
        IDT.machine_check.set_handler_fn(machine_check_error_handler);
        IDT.simd_floating_point.set_handler_fn(simd_floating_point_error_handler);
        IDT.virtualization.set_handler_fn(virtualization_error_handler);
        IDT.cp_protection_exception.set_handler_fn(cp_protection_exception_handler);
        IDT.hv_injection_exception.set_handler_fn(hv_inject_exception_handler);
        IDT.vmm_communication_exception.set_handler_fn(vmm_communication_exception_handler);
        IDT.security_exception.set_handler_fn(security_exception);
        IDT.double_fault
            .set_handler_fn(double_fault_error);
    }
}
extern "x86-interrupt" fn double_fault_error(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    let graphics = unsafe { GraphicsDriver::new().unwrap() };

    let window = Window::new((500, 200), (0, 0));

    window.render_center_text("Double fault", 3, colors::BRIGHT_RED, &graphics);

    hcf();
}

extern "x86-interrupt" fn machine_check_error_handler(
    _stack_frame: InterruptStackFrame,
) -> ! {
    let graphics = unsafe { GraphicsDriver::new().unwrap() };

    let window = Window::new((500, 200), (0, 0));

    window.render_center_text("Machine check error handler", 3, colors::BRIGHT_RED, &graphics);

    hcf();
}

macro_rules! ExceptionHandler {
    ($name:tt, $display_box_name:expr) => {
        extern "x86-interrupt" fn $name(_: InterruptStackFrame) {
            let graphics = unsafe { GraphicsDriver::new().unwrap() };
        
            let window = Window::new((500, 200), (0, 0));
        
            window.render_center_text($display_box_name, 3, colors::BRIGHT_RED, &graphics);
        }
    };
}

macro_rules! ExceptionHandlerWithErrCode {
    ($name:tt, $display_box_name:expr) => {
        extern "x86-interrupt" fn $name(_: InterruptStackFrame, _: u64) {
            let graphics = unsafe { GraphicsDriver::new().unwrap() };
        
            let window = Window::new((500, 200), (0, 0));
        
            window.render_center_text($display_box_name, 3, colors::BRIGHT_RED, &graphics);
        }
    };
}

use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};
macro_rules! ExceptionHandlerPaging {
    ($name:tt, $display_box_name:expr) => {
        extern "x86-interrupt" fn $name(_: InterruptStackFrame, _: PageFaultErrorCode) {
            let graphics = unsafe { GraphicsDriver::new().unwrap() };
        
            let window = Window::new((500, 200), (0, 0));
        
            window.render_center_text($display_box_name, 3, colors::BRIGHT_RED, &graphics);
        }
    };
}

ExceptionHandler!(divide_error_handler, "Division Error");
ExceptionHandler!(debug_error_handler, "Debug");
ExceptionHandler!(non_maskable_interrupt_error_handler, "Non maskable interrupt");
ExceptionHandler!(breakpoint_handler, "Breakpoint");
ExceptionHandler!(overflow_error_handler, "Overflow");
ExceptionHandler!(bound_range_exceeded_error_handler, "Bound range exceeded");
ExceptionHandler!(invalid_opcode_error_handler, "Invalid Opcode");
ExceptionHandler!(device_not_avialable_error, "Device nov aviable");
ExceptionHandler!(x87_floating_point_error_handler, "x87 Floating Point error");
ExceptionHandler!(virtualization_error_handler, "Virtualization Error");
ExceptionHandler!(hv_inject_exception_handler, "HV Inject Exception");
ExceptionHandler!(simd_floating_point_error_handler, "SIMD Floating Point Error");

ExceptionHandlerWithErrCode!(invalid_tss_handler, "Invalid tss");
ExceptionHandlerWithErrCode!(segment_not_presented_error_handler, "Segment not present");
ExceptionHandlerWithErrCode!(stack_segmentation_fault_error_handler, "Stack Segmentation fault");
ExceptionHandlerWithErrCode!(general_protection_fault_handler, "General protection fault");
ExceptionHandlerWithErrCode!(alignment_check_error_handler, "Alignment Check Error");
ExceptionHandlerWithErrCode!(cp_protection_exception_handler, "Cp Protection Exception");
ExceptionHandlerWithErrCode!(vmm_communication_exception_handler, "VMM communication error");
ExceptionHandlerWithErrCode!(security_exception, "Security exception");

ExceptionHandlerPaging!(page_fault_error_handler, "");