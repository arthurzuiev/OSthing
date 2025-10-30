#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::{panic::PanicInfo};
mod vga_buffer;
mod serial;

#[cfg(test)]
use os_thing::test_runner;

#[cfg(not(test))]
use crate::vga_buffer::Color;


#[unsafe(no_mangle)] // For Rust: Don't you dare mangle this name!
pub extern "C" fn _start() -> ! {
    // So... we extern it as C because BIOS loves C conventions.
    // And it is _start because BIOS loves to start at start.

    println!("BIOS Wasteland is my new home :_)");
    
    #[cfg(test)]
    test_main();
    loop {}
}

/// manual panic (｡Ó﹏Ò｡)
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    c_println!(Color::Red, Color::Black,"{}", info);
    loop {}
}

/// test panic (｡Ó﹏Ò｡)
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_thing::test_panic_handler(info)
}