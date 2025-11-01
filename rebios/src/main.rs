#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

// other
#[cfg(test)]
use rebios::test_runner;

use core::{panic::PanicInfo};
mod serial;

// lib imports
use rebios::print;
#[allow(dead_code)]
use rebios::vga_buffer::Color;
use rebios::{c_println};
use rebios::interrupts;
use rebios::memory;
use rebios::memory::BootInfoFrameAllocator;

// loader
use bootloader::{BootInfo, entry_point};
use x86_64::{structures::paging::Page, VirtAddr};


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // init out stuff
    rebios::init(); //for now mostly exeption stuff

    c_println!(Color::Magenta, Color::Black, "Memory paging is hell compared to BIOS Wasteland >_<");
    print!("");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) }; // memory mapper... hopefully its accurate ^_^
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeef000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    #[cfg(test)]
    test_main();

    c_println!(Color::Green, Color::Black, "I did not crash... yet ^_^");

    rebios::hlt_loop();
}

/// manual panic (｡Ó﹏Ò｡)
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    c_println!(Color::Red, Color::Black,"{}", info);
    rebios::hlt_loop();
}

/// test panic (｡Ó﹏Ò｡)
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rebios::test_panic_handler(info)
}