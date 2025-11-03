#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
use rebios::test_runner;

use core::{panic::PanicInfo};
mod serial;

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

// remove bootloader usage for normal boot; tests still use bootloader path
// use bootloader::{BootInfo, entry_point};
use x86_64::{VirtAddr};

extern crate alloc;

// NOTE: when running tests (cfg(test)) your test entry point in lib.rs still applies

pub extern "C" fn _start() -> ! {
    // forward to your kernel_main (keeps your current name and tests)
    kernel_main()
}

fn kernel_main() -> ! {
    // init stuff
    rebios::init();

    #[cfg(test)]
    test_main();

    // Announcement
    c_println!(Color::Magenta, Color::Black, "And here we are... Awaiting Async and Await...");
    print!("");

    // --- Get the HHDM offset from Limine and create a mapper using that offset ---
    // Use the HHDM request we declared in lib.rs
    {
        use limine::request::HhdmRequest;
        // Since the request is static in lib.rs, grab it:
        let hhdm_resp = rebios::HHDM_REQUEST
            .get_response()
            .expect("Limine did not provide HHDM response");
        let phys_mem_offset = VirtAddr::new(hhdm_resp.offset());
        let mut mapper = unsafe { memory::init(phys_mem_offset) };

        // --- Get the memory map response from Limine and create your FrameAllocator ---
        let memmap_resp = rebios::MEMORY_MAP_REQUEST
            .get_response()
            .expect("Limine did not provide a memory map");

        let mut frame_allocator = unsafe {
            BootInfoFrameAllocator::init(memmap_resp)
        };

        allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    }

    c_println!(Color::Green, Color::Black, "I did not crash... yet ^_^");

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

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

/// panic handlers left as-is
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    c_println!(Color::Red, Color::Black,"{}", info);
    rebios::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rebios::test_panic_handler(info)
}
