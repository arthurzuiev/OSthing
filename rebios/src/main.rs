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
use rebios::{print, println};
#[allow(dead_code)]
use rebios::vga_buffer::Color;
use rebios::{c_println};
use rebios::interrupts;
use rebios::memory;
use rebios::memory::BootInfoFrameAllocator;
use rebios::allocator;

// loader
use bootloader::{BootInfo, entry_point};
use x86_64::{VirtAddr};

extern crate alloc;
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};


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

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));


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