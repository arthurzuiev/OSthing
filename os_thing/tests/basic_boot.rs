#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_thing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[allow(dead_code)]
fn test_runner(_tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_thing::test_panic_handler(info)
}

use os_thing::println;

#[test_case]
fn test_println() {
    println!("test_println output | from basic boot");
}