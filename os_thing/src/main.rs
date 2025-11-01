#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

// other
#[cfg(test)]
use os_thing::test_runner;

use core::{panic::PanicInfo};
mod serial;

// lib imports
use os_thing::print;
#[allow(dead_code)]
use os_thing::vga_buffer::Color;
use os_thing::{c_println, println};
use os_thing::interrupts;
use os_thing::memory;

// loader
use bootloader::{BootInfo, entry_point};
use x86_64::{structures::paging::Translate, VirtAddr};


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // So... we extern it as C because BIOS loves C conventions.
    // And it is _start because BIOS loves to start at start.

    // init out stuff
    os_thing::init(); //for now mostly exeption stuff

    c_println!(Color::Magenta, Color::Black, "Memory paging is hell compared to BIOS Wasteland >_<");
    print!("");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) }; // memory mapper... hopefully its accurate ^_^
    
    let addresses = [
        0x8000, // identity mapped vga buffer page :D
        0x201008, // some code page ? wut
        0x0100_0020_1a10, // some stack page
        boot_info.physical_memory_offset, // Virtual adress mapped to physical adress 0
    ];


    c_println!(Color::Cyan, Color::Black, "TRanslating Virtual Adresses to physical");
    for &adress in &addresses {
        let virt = VirtAddr::new(adress);
        let phys = mapper.translate_addr(virt);
        c_println!(Color::Cyan, Color::Black, "Probably a valid translation: {:?} -> {:?}", virt, phys);
    }
    c_println!(Color::Green, Color::Black, "Sorry if made mistaked ^_^");

    #[cfg(test)]
    test_main();

    c_println!(Color::Green, Color::Black, "I did not crash... yet ^_^");

    os_thing::hlt_loop();
}

/// manual panic (｡Ó﹏Ò｡)
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    c_println!(Color::Red, Color::Black,"{}", info);
    os_thing::hlt_loop();
}

/// test panic (｡Ó﹏Ò｡)
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_thing::test_panic_handler(info)
}