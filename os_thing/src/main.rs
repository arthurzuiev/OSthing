#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::{panic::PanicInfo};
mod serial;

use os_thing::print;
#[allow(dead_code)]
use os_thing::vga_buffer::Color;
#[allow(dead_code)]
use os_thing::{println, set_foreground_color, c_println};

#[cfg(test)]
use os_thing::test_runner;

#[unsafe(no_mangle)] // For Rust: Don't you dare mangle this name!
pub extern "C" fn _start() -> ! {
    // So... we extern it as C because BIOS loves C conventions.
    // And it is _start because BIOS loves to start at start.

    // init out stuff
    os_thing::init(); //for now mostly exeption stuff

    c_println!(Color::Magenta, Color::Black, "BIOS Wasteland started growing grass (modules) :D");
    print!("");

    //invoke exception
    os_thing::init();
    
    // trigger a page fault to trigger triple fault :D

    c_println!(Color::Green, Color::Black, "I did not crash... yet ^_^");

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