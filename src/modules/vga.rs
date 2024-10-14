use core::fmt;
use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
/// The VGA (Video Graphics Array) buffer is a specific area of memory used to control the display output on a screen.
/// In VGA text mode, this buffer is typically located at memory address `0xB8000` in the physical address space.
///
/// The VGA buffer is organized as a 2D array of characters, where each character is represented by two bytes:
/// - The first byte contains the ASCII value of the character.
/// - The second byte contains the color code, which combines the foreground and background colors.
///
/// The CPU accesses the VGA buffer through memory-mapped I/O. This means that the CPU can read from and write to the VGA buffer
/// just like it would with regular RAM, but the writes and reads are directed to the video hardware instead of the main memory.
///
/// In OS development, the VGA buffer is used to display text on the screen. This is particularly useful in early stages of OS development
/// when a graphical user interface is not yet available. By writing to the VGA buffer, the OS can provide feedback, display logs, and interact
/// with the user through a simple text interface.
///
/// Here is a step-by-step explanation of how the VGA buffer is used in OS development:
///
/// 1. **Memory Mapping**: The VGA buffer is mapped to a specific physical address (`0xB8000`). The OS needs to ensure that this address is accessible
///    and not used for other purposes.
///
/// 2. **Character Representation**: Each character on the screen is represented by two bytes in the VGA buffer. The first byte is the ASCII value
///    of the character, and the second byte is the color code.
///
/// 3. **Writing to the Buffer**: To display a character on the screen, the OS writes the appropriate ASCII value and color code to the correct
///    position in the VGA buffer. For example, to display the character 'A' with a white foreground and black background at the top-left corner
///    of the screen, the OS would write the values `0x41` (ASCII for 'A') and `0x0F` (color code for white on black) to the first two bytes of
///    the VGA buffer.
///
/// 4. **Screen Dimensions**: The VGA text mode typically supports 80 columns and 25 rows. This means the VGA buffer needs to accommodate
///    80 * 25 * 2 = 4000 bytes.
///
/// 5. **Scrolling**: When the screen is full, the OS may need to scroll the contents up to make room for new text. This involves copying
///    the contents of the VGA buffer up by one row and clearing the last row.
///
/// 6. **Cursor Position**: The OS can control the position of the text cursor by writing to specific I/O ports. This allows the OS to
///    move the cursor to the desired position before writing new characters.
///
/// By understanding and utilizing the VGA buffer, OS developers can create a basic text interface for interacting with the user,
/// displaying logs, and debugging the OS during its early stages of development.
///
///
///
///
///
///
///
///
///
/// Represents the available colors for the VGA text mode.
///
/// The `Color` enum defines the standard 16 colors used in VGA text mode. Each color is represented by a unique `u8` value.
///
/// This enum is crucial for setting text colors in VGA text mode, which is commonly used in OS development for displaying text on the screen.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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

/// Represents a color code combining foreground and background colors.
///
/// The `ColorCode` struct is a wrapper around a `u8` value that encodes both the foreground and background colors for a character in VGA text mode.
///
/// # Fields
/// - `0`: The combined color code as a `u8`.
///
/// # Methods
/// - `new(foreground: Color, background: Color) -> ColorCode`: Creates a new `ColorCode` by combining the foreground and background colors.
///
/// This struct is essential for setting the color attributes of characters displayed on the screen in VGA text mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    /// Creates a new `ColorCode` by combining the foreground and background colors.
    ///
    /// # Arguments
    /// - `foreground`: The foreground color as a `Color`.
    /// - `background`: The background color as a `Color`.
    ///
    /// # Returns
    /// A `ColorCode` instance with the combined color code.
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// Represents a character on the screen with its ASCII value and color code.
///
/// The `ScreenChar` struct holds the ASCII character and its associated color code for display in VGA text mode.
///
/// # Fields
/// - `ascii_character`: The ASCII value of the character as a `u8`.
/// - `color_code`: The color code as a `ColorCode`.
///
/// This struct is used to represent each character cell in the VGA text buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

/// The height of the VGA text buffer (25 rows).
const BUFFER_HEIGHT: usize = 25;

/// The width of the VGA text buffer (80 columns).
const BUFFER_WIDTH: usize = 80;

/// Represents the VGA text buffer.
///
/// The `Buffer` struct holds a 2D array of `ScreenChar` wrapped in `Volatile` representing the entire VGA text buffer.
///
/// # Fields
/// - `chars`: A 2D array of `Volatile<ScreenChar>` with dimensions `BUFFER_HEIGHT` x `BUFFER_WIDTH`.
///
/// This struct is used to directly manipulate the VGA text buffer for displaying text on the screen.
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Provides methods to write text to the VGA text buffer.
///
/// The `Writer` struct manages the current position in the buffer and the color code for text. It provides methods to write individual bytes and strings to the buffer.
///
/// # Fields
/// - `column_position`: The current column position in the buffer as a `usize`.
/// - `color_code`: The current color code as a `ColorCode`.
/// - `buffer`: A mutable reference to the `Buffer`.
///
/// This struct is crucial for writing text to the screen in VGA text mode, which is a common requirement in OS development for displaying messages and debugging information.
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// Writes a single byte to the VGA text buffer.
    ///
    /// # Arguments
    /// - `byte`: The byte to write as a `u8`.
    ///
    /// If the byte is a newline (`\n`), it moves to a new line. Otherwise, it writes the byte at the current position and advances the column position.
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

    /// Writes a string to the VGA text buffer.
    ///
    /// # Arguments
    /// - `s`: The string to write as a `&str`.
    ///
    /// This method iterates over each byte in the string and writes it to the buffer. Non-printable characters are replaced with `0xfe`.
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// Moves the cursor to a new line in the VGA text buffer.
    ///
    /// This method shifts all rows up by one, effectively removing the top row, and clears the last row. It then resets the column position to 0.
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Clears a row in the VGA text buffer.
    ///
    /// # Arguments
    /// - `row`: The index of the row to clear as a `usize`.
    ///
    /// This method writes a blank character to each column in the specified row.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    /// Temporarily sets the color code to the provided value
    /// This is useful for printing text in a different color without changing the default color.
    ///
    /// # Arguments
    /// - `color_code`: The color code to set as a `ColorCode`.
    ///
    /// This method sets the color code to the provided value for the next write operation.
    #[allow(dead_code)]
    pub fn set_color(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }

    /// Resets the color to a default value (or previous value if needed)
    ///
    /// # Arguments
    /// - `color_code`: The color code to set as a `ColorCode`.
    ///
    /// This method resets the color code to the provided value for the next write operation.
    #[allow(dead_code)]
    pub fn reset_color(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }
}

/// Implements the `fmt::Write` trait for the `Writer` struct.
///
/// This allows the `Writer` to be used with the `write!` and `writeln!` macros.
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    /// A global `Writer` instance wrapped in a `Mutex`.
    ///
    /// This `Writer` instance is used to write text to the VGA text buffer in a thread-safe manner.
    /// The `Writer` struct manages the current position in the buffer and the color code for text.
    /// It provides methods to write individual bytes and strings to the buffer.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightCyan, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/// Prints to the VGA text buffer.
/// This macro uses the `format_args!` macro to format the given arguments and then calls the internal `_print` function to write the formatted string to the VGA buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

/// Prints to the VGA text buffer with a newline.
/// This macro is similar to `print!`, but it appends a newline character (`\n`) at the end.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Prints to the VGA text buffer with a specified color.
/// This function is used internally by the `print!` and `println!` macros to write formatted text to the VGA buffer with a specific color.
///
/// # Arguments
/// - `args`: The formatted arguments to print.
/// - `color`: The color code to use for the text.
///
/// This function is not intended to be used directly. Instead, use the `print!` and `println!` macros.
pub fn _print_with_color(args: fmt::Arguments, color: ColorCode) {
    let mut writer = WRITER.lock();
    let original_color = writer.color_code;
    writer.set_color(color);
    writer.write_fmt(args).unwrap();
    writer.reset_color(original_color);
}

/// Prints to the VGA text buffer.
///
/// This macro uses the `format_args!` macro to format the given arguments
/// and then calls the internal `_print` function to write the formatted
/// string to the VGA text buffer.
/// Can also be used to print text in a specific color by providing a color code.
#[macro_export]
macro_rules! printc {
    ($string:expr, $color:expr) => {
        $crate::vga::_print_with_color(format_args!($string), $color);
    };
    ($($arg:tt)*) => {
        $crate::vga::_print(format_args!($($arg)*));
    };
}

/// Prints to the VGA text buffer with a newline.
///
/// This macro is similar to `print!`, but it appends a newline character (`\n`) at the end.
/// It uses the `format_args!` macro to format the given arguments and then calls the internal `_print` function to write the formatted string to the VGA text buffer.
/// Can also be used to print text in a specific color by providing a color code.
#[macro_export]
macro_rules! printlnc {
    ($fmt:expr, $color:expr) => {
        $crate::vga::_print_with_color(format_args!(concat!($fmt, "\n")), $color);
    };
    ($fmt:expr) => {
        $crate::vga::_print(format_args!(concat!($fmt, "\n")));
    };
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Prints to the VGA text buffer with a specified color.
///
/// This macro is similar to `println!`, but it allows specifying a color code for the text.
/// It uses the `format_args!` macro to format the given arguments and then calls the internal `_print_with_color` function to write the formatted string to the VGA text buffer with the specified color.
/// The color code is used for the entire line of text.
/// Can also be used to print text in a specific color by providing a color code.
#[macro_export]
macro_rules! printlnc_f {
    ($color:expr, $fmt:expr) => {{
        $crate::vga::_print_with_color(format_args!($fmt), $color);
        $crate::vga::_print_with_color(format_args!("\n"), $color);
    }};
    ($color:expr, $fmt:expr, $($arg:tt)*) => {{
        $crate::vga::_print_with_color(format_args!($fmt, $($arg)*), $color);
        $crate::vga::_print_with_color(format_args!("\n"), $color);
    }};
}

/// Internal function to print formatted arguments to the VGA text buffer.
///
/// This function is not intended to be used directly. Instead, use the
/// `print!` and `println!` macros.
///
/// # Arguments
///
/// * `args` - The formatted arguments to print.
///
/// # Panics
///
/// This function will panic if it fails to acquire the lock on the VGA writer
/// or if writing to the VGA buffer fails.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

/// Tests for the VGA module.
/// These tests validate the functionality of the VGA text buffer and the Writer struct.
/// The tests cover writing to the buffer, scrolling, wrapping, and color output.
/// The tests use the `print!` and `println!` macros to write to the VGA buffer and compare the output with the expected values.
/// The tests are run using the built-in test framework provided by Rust.

#[test_case]
fn test_print_simple() {
    print!("test_println_simple output");
}

#[test_case]
fn test_print_many() {
    for _ in 0..200 {
        print!("test_print_many output");
    }
}

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

#[test_case]
fn test_println_wrapping() {
    for _ in 0..BUFFER_HEIGHT + 1 {
        println!("Some test string that should wrap to the next line");
    }

    for row in 0..BUFFER_HEIGHT - 1 {
        let screen_char = WRITER.lock().buffer.chars[row][0].read();
        assert_eq!(char::from(screen_char.ascii_character), 'S');
    }
}

#[test_case]
fn test_printc_simple() {
    printc!(
        "test_printc_simple output",
        ColorCode::new(Color::LightRed, Color::Black)
    );
}

#[test_case]
fn test_printc_many() {
    for _ in 0..200 {
        printc!(
            "test_printc_many output",
            ColorCode::new(Color::LightRed, Color::Black)
        );
    }
}

#[test_case]
fn test_printlnc_simple() {
    printlnc!(
        "test_printlnc_simple output",
        ColorCode::new(Color::LightRed, Color::Black)
    );
}

#[test_case]
fn test_printlnc_many() {
    for _ in 0..200 {
        printlnc!(
            "test_printlnc_many output",
            ColorCode::new(Color::LightRed, Color::Black)
        );
    }
}

#[test_case]
fn test_printlnc_wrapping() {
    for _ in 0..BUFFER_HEIGHT + 1 {
        printlnc!(
            "Some test string that should wrap to the next line",
            ColorCode::new(Color::LightRed, Color::Black)
        );
    }

    for row in 0..BUFFER_HEIGHT - 1 {
        let screen_char = WRITER.lock().buffer.chars[row][0].read();
        assert_eq!(char::from(screen_char.ascii_character), 'S');
    }
}

#[test_case]
fn test_printlnc_f_simple() {
    printlnc_f!(
        ColorCode::new(Color::LightRed, Color::Black),
        "test_printlnc_f_simple output"
    );
}

#[test_case]
fn test_printlnc_f_many() {
    for _ in 0..200 {
        printlnc_f!(
            ColorCode::new(Color::LightRed, Color::Black),
            "test_printlnc_f_many output"
        );
    }
}

#[test_case]
fn test_printlnc_f_output() {
    let s = "Some test string that fits on a single line";
    printlnc_f!(ColorCode::new(Color::LightRed, Color::Black), "{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}

#[test_case]
fn test_printlnc_f_wrapping() {
    for _ in 0..BUFFER_HEIGHT + 1 {
        printlnc_f!(
            ColorCode::new(Color::LightRed, Color::Black),
            "Some test string that should wrap to the next line"
        );
    }

    for row in 0..BUFFER_HEIGHT - 1 {
        let screen_char = WRITER.lock().buffer.chars[row][0].read();
        assert_eq!(char::from(screen_char.ascii_character), 'S');
    }
}
