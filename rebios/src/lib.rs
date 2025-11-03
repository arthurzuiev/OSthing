#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

// module use
use core::panic::PanicInfo;
pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub mod allocator;
pub mod task;

extern crate alloc;

//
// === Limine requests (used by the kernel on boot) ===
//
use limine::request::{MemoryMapRequest, HhdmRequest};
use limine::BaseRevision;
/// Tell Limine the protocol revision we require
pub static BASE_REVISION: BaseRevision = BaseRevision::new();

/// Ask Limine for a memory map (populated before entry)
#[used]
pub static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

/// Ask Limine for the Higher-Half Direct Map (HHDM) offset
#[used]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();




pub fn init(){
    gdt::init();
    interrupts::ini_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

// LIB TEST ===============================================================================================================================================

/// Entry point for `cargo test`
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init(); // so scary exceptions not eat our OS
    test_main();
    hlt_loop();
}

// TEST LOGIC ===============================================================================================================================================
// this thing is made so we associate any T with Testable... and we print stuff automatically instead of using print in every test function.
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok] :D");
    }
}

// a test runner
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("============================ Running {} yummy tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("WHAT THE FUCK DID YOU JUST GIVE ME!? [FAILED]\n");
    serial_println!(">:( Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

