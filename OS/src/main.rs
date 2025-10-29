#![no_std]
#![no_main]

use core::{panic::PanicInfo};
mod vga_buffer;

#[unsafe(no_mangle)] // For Rust: Don't you dare mangle this name!
pub extern "C" fn _start() -> ! {
    // So... we extern it as C because BIOS loves C conventions.
    // And it is _start because BIOS loves to start at start.
   
    println!("Hello from {macroname} :D", macroname="println");

    loop {}
}

/// When panic... do nothing. Classic behavious of human in real life... beautiful.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}