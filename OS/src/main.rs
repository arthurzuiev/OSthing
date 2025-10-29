#![no_std]
#![no_main]

use core::{panic::PanicInfo};

static MESSAGE: &[u8] = b"Hello, BIOS Wasteland!";

#[unsafe(no_mangle)] // For Rust: Don't you dare mangle this name!
pub extern "C" fn _start() -> ! {
    // So... we extern it as C because BIOS loves C conventions.
    // And it is _start because BIOS loves to start at start.

    // VGA buffer with some text inside. Its located at 0xb8000 adress in memory.
    let vga_buffer_addr: *mut u8 = 0xb8000 as *mut u8; // RustAnalyzer forced me to ": *mut u8" ;-;

    // lets loop through each byte in our message
    for (i, &byte) in MESSAGE.iter().enumerate() {
        // unsafe :D
        unsafe {
            // some offset magic
            *vga_buffer_addr.offset(i as isize * 2) = byte;
            *vga_buffer_addr.offset(i as isize * 2 + 1) = 0xb; // white on black
        }
    }

    loop {}
}

/// When panic... do nothing. Classic behavious of human in real life... beautiful.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}