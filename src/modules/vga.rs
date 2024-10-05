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
struct ColorCode(u8);

impl ColorCode {
    /// Creates a new `ColorCode` by combining the foreground and background colors.
    ///
    /// # Arguments
    /// - `foreground`: The foreground color as a `Color`.
    /// - `background`: The background color as a `Color`.
    ///
    /// # Returns
    /// A `ColorCode` instance with the combined color code.
    fn new(foreground: Color, background: Color) -> ColorCode {
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
/// The `Buffer` struct holds a 2D array of `ScreenChar` representing the entire VGA text buffer.
///
/// # Fields
/// - `chars`: A 2D array of `ScreenChar` with dimensions `BUFFER_HEIGHT` x `BUFFER_WIDTH`.
///
/// This struct is used to directly manipulate the VGA text buffer for displaying text on the screen.
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
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
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }

    /// Moves to a new line in the VGA text buffer.
    ///
    /// This method is currently a placeholder and does not perform any actions.
    fn new_line(&mut self) {}
}

impl Writer {
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
}
