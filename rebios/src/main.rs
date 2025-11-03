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
use rebios::task::{Task};
use rebios::task::keyboard;
use rebios::task::executor::Executor;

// loader
use bootloader::{BootInfo, entry_point};
use x86_64::{VirtAddr};

extern crate alloc;

// set an entry point for out kernel
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // init out stuff
    rebios::init(); //for now mostly exeption stuff

    #[cfg(test)]
    test_main();

    // yummy message
    c_println!(Color::Magenta, Color::Black, "And here we are... Awaiting Async and Await...");
    print!("");

    // initialize memory management stuff
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) }; // memory mapper... hopefully its accurate ^_^
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    
    c_println!(Color::Green, Color::Black, "I did not crash... yet ^_^");

    // space for more stuff :D
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses())); // new
    executor.run();



    // in case scarry error happens that will eat my executor...
    #[allow(unreachable_code)]
    rebios::hlt_loop();
}


async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
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