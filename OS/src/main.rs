#![no_std]
#![no_main]

use core::{panic::PanicInfo};
mod vga_buffer;
use crate::vga_buffer::Color;

#[unsafe(no_mangle)] // For Rust: Don't you dare mangle this name!
pub extern "C" fn _start() -> ! {
    // So... we extern it as C because BIOS loves C conventions.
    // And it is _start because BIOS loves to start at start.

    println!("VGA Color Palete:");
    let colors = [
        Color::Black, Color::Blue, Color::Green, Color::Cyan,
        Color::Red, Color::Magenta, Color::Brown, Color::LightGray,
        Color::DarkGray, Color::LightBlue, Color::LightGreen, Color::LightCyan,
        Color::LightRed, Color::Pink, Color::Yellow, Color::White,
    ];

    for &bg in &colors {
        for &fg in &colors {
            c_print!(fg, bg, "{:>3} ", fg as u8);
        }
        c_println!(Color::White, Color::Black, ""); // reset line
    }

    println!("");
    println!("VGA Rainbow :D");
    let rainbow = [
        Color::Red,
        Color::LightRed,
        Color::Yellow,
        Color::Green,
        Color::Cyan,
        Color::Blue,
        Color::Magenta,
    ];

    for (i, &color) in rainbow.iter().enumerate() {
        set_foreground_color!(color);
        println!("Color {} shining bright!", i + 1);
    }

    // Reset to normal at the end
    set_color!(Color::White, Color::Black);
    println!("Back to default colors!");

    loop {}
}

/// When panic... do nothing. Classic behavious of human in real life... beautiful.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    c_println!(Color::Red, Color::Black,"{}", info);
    loop {}
}