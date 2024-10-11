// The lazy_static crate is used to create a lazily evaluated static variable.
use lazy_static::lazy_static;
// The spin crate is used to provide a spinlock, which is a type of lock that spins in a loop until it can acquire the lock.
use spin::Mutex;
// The uart_16550 crate is used to provide an interface for interacting with UART serial ports.
use uart_16550::SerialPort;

// The SERIAL1 static variable is a lazily evaluated static Mutex<SerialPort> instance.
// This variable is used to provide a global interface to the first serial port (COM1) on x86 systems.
// The Mutex type is used to provide mutual exclusion for the serial port, ensuring that only one thread can access it at a time.
// The SerialPort type is used to provide an interface for interacting with UART serial ports.
// The lazy_static macro is used to create a lazily evaluated static variable.
// This macro is used to create a static variable that is initialized at runtime when it is first accessed.
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

// The serial_print macro is used to print formatted text to the serial port.
// This macro is used to print text to the serial port, which is a common way to output text in early stages of OS development.
// The macro takes a format string and arguments, formats them, and prints the resulting text to the serial port.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::uart::_print(format_args!($($arg)*));
    };
}

// The serial_println macro is used to print formatted text to the serial port followed by a newline character.
// This macro is used to print text to the serial port followed by a newline character, which is a common way to output text in early stages of OS development.
// The macro takes a format string and arguments, formats them, and prints the resulting text to the serial port followed by a newline character.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

// The _print function is used to print formatted text to the serial port.
// This function is used internally by the serial_print macro to print formatted text to the serial port.
// The function takes a format string and arguments, formats them, and prints the resulting text to the serial port.
// This function is not intended to be called directly, but is used internally by the serial_print macro.
#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}
