use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;



// Color Enums
#[allow(dead_code)] // shut up unused warnings >:(
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // derive useful traits
#[repr(u8)] // VGA color codes are u4... but rust doesnt have any... so u8 it is. ;-;
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

impl From<u8> for Color {
    fn from(val: u8) -> Color {
        match val {
            0 => Color::Black,
            1 => Color::Blue,
            2 => Color::Green,
            3 => Color::Cyan,
            4 => Color::Red,
            5 => Color::Magenta,
            6 => Color::Brown,
            7 => Color::LightGray,
            8 => Color::DarkGray,
            9 => Color::LightBlue,
            10 => Color::LightGreen,
            11 => Color::LightCyan,
            12 => Color::LightRed,
            13 => Color::Pink,
            14 => Color::Yellow,
            15 => Color::White,
            _ => Color::White, // fallback
        }
    }
}


// To represent full color code that specifies foreground and background color we create newtype on top of u8
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // ensure exact same data layout as u8
pub struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }

    #[allow(dead_code)]
    pub fn foreground(&self) -> Color {
        // lower 4 bits
        unsafe { core::mem::transmute(self.0 & 0x0F) }
    }

    #[allow(dead_code)]
    pub fn background(&self) -> Color {
        // upper 4 bits
        unsafe { core::mem::transmute(self.0 >> 4) }
    }
}



// Charcter structures and VGA constants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // guarantees that the struct’s fields are laid out exactly like in a C struct and thus guarantees the correct field ordering
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)] //ensure that it has the same memory layout as its single field
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}



//Finally some logic and functions after all data :D
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        // We iterate over all the screen characters and move each character one row up.
        // We also omit the 0th row (the first range starts at 1) because it’s the row that is shifted off screen.
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    pub fn clear_row(&mut self, row:usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code 
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }

    /// Change foreground color, keep background as-is
    #[allow(dead_code)]
    pub fn set_foreground(&mut self, fg: Color) {
        let bg = Color::from(self.color_code.0 >> 4);
        self.color_code = ColorCode::new(fg, bg);
    }

    #[allow(dead_code)]
    pub fn set_background(&mut self, bg: Color) {
        let fg = Color::from(self.color_code.0 & 0x0F);
        self.color_code = ColorCode::new(fg, bg);
    }

    #[allow(dead_code)]
    /// Change both foreground and background
    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.color_code = ColorCode::new(fg, bg);
    }

    #[allow(dead_code)]
    pub fn get_color(&self) -> ColorCode {
        self.color_code
    }
}



// Implementing support for Rust macros
// just wrapping our write function in the macro lol
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}



// Making a GLOBAL interface (ノ^^)ノ
// using lazy_static dependency for that
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer { 
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black), 
        buffer:  unsafe { &mut *(0xb8000 as *mut Buffer) }
    });
}



// Making pintln and print macros usable
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

// and some custom macros

/// Colorful print. Argument 1: foreground color | Argument 2: background color | Text and other stuff ...
#[macro_export]
macro_rules! c_print {
    ($fg:expr, $bg:expr, $($arg:tt)*) => ({
        use $crate::vga_buffer::{WRITER};
        use core::fmt::Write;

        let mut w = WRITER.lock();

        // Save old color
        let old_color = w.get_color();
        w.set_color($fg, $bg);
        write!(w, $($arg)*).unwrap();
        w.set_color(old_color.foreground(), old_color.background());

    });
}

#[macro_export]
macro_rules! c_println {
    ($fg:expr, $bg:expr, $($arg:tt)*) => ({
        $crate::c_print!($fg, $bg, "{}\n", format_args!($($arg)*));
    });
}

/// Set foreground color globally for all future prints
#[macro_export]
macro_rules! set_foreground_color {
    ($fg:expr) => {{
        use $crate::vga_buffer::WRITER;
        let mut w = WRITER.lock();
        w.set_foreground($fg);
    }};
}

/// Set background color globally for all future prints
#[macro_export]
macro_rules! set_background_color {
    ($bg:expr) => {{
        use $crate::vga_buffer::WRITER;
        let mut w = WRITER.lock();
        w.set_background($bg);
    }};
}

/// Set both foreground and background color globally
#[macro_export]
macro_rules! set_color {
    ($fg:expr, $bg:expr) => {{
        use $crate::vga_buffer::WRITER;
        let mut w = WRITER.lock();
        w.set_color($fg, $bg);
    }};
}

// testing stuff =====================================================================================================================================
#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}