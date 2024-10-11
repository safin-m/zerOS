// This attribute tells the Rust compiler that the standard library is not available.
// This is crucial for OS development because the standard library depends on an underlying operating system, which we are trying to build.
#![no_std]
// This attribute indicates that the usual main function is not used.
// Instead, a custom entry point is defined. This is necessary for low-level programming like OS development where we need full control over the entry point.
#![no_main]
// This attribute enables custom test frameworks, which is necessary for writing tests in OS development.
// It allows us to define our own test runner function, which is called when running tests.
#![feature(custom_test_frameworks)]
// This attribute specifies the test framework to use.
// In this case, we are using the built-in test framework provided by Rust.
#![test_runner(zer_os::run_test)]
// This attribute reexports the test harness main function as start_test.
// This is necessary for running tests because the test harness expects the main function to be named start_test.
#![reexport_test_harness_main = "start_test"]

// Importing PanicInfo from the core library, which is a subset of the standard library that can be used in no_std environments.
// PanicInfo provides information about panics, which are critical errors.
use core::panic::PanicInfo;

// Importing the println macro from the vga module.
// This macro is used for printing formatted text to the screen in VGA text mode.
#[allow(unused_imports)]
use modules::uart;
use modules::vga;
use vga::{Color, ColorCode};

// Declaring a module named vga inside the modules module.
// This is used for handling VGA text mode, which is a common way to output text to the screen in early stages of OS development.
mod modules {
    pub mod panic_handler;
    pub mod uart;
    pub mod vga;
}

// This attribute marks the following function as the panic handler, which is called whenever a panic occurs.
// In OS development, we need to define our own panic handler because the default one in the standard library is not available.
#[panic_handler]
// The panic handler function. The ! indicates that this function will never return.
//
// # Arguments
// - `info` : A reference to a PanicInfo struct, which contains information about the panic.
//
// In this case, it enters an infinite loop, effectively halting the system.
#[allow(deprecated)]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    let panic_color = ColorCode::new(Color::LightRed, Color::Black);
    let panic_message_color = ColorCode::new(Color::Red, Color::Black);

    if let Some(location) = info.location() {
        printlnc_f!(
            panic_color,
            " Panic occurred at file '{}' line {}",
            location.file(),
            location.line()
        );
        printlnc_f!(panic_message_color, " {}", info.message());
    } else {
        printlnc_f!(panic_color, " Panic occurred with no message.");
    }

    #[cfg(test)]
    start_test();

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    zer_os::test_panic_handler(info)
}

static TO_PRINT: &[u8] = b"Booting the kernel...\n"; // The message to print to the screen.

// The #[no_mangle] attribute is used to prevent the Rust compiler from renaming this function, which is necessary for the bootloader to find it.
#[no_mangle]
// The entry point of the kernel.
// This function is called by the bootloader when the system starts.
// It initializes the VGA buffer and prints a message to the screen.
// The function then enters an infinite loop, effectively halting the system.
// The extern "C" attribute specifies the calling convention for this function, which is the C calling convention.
pub extern "C" fn _start() -> ! {
    // The following code is used to run tests when the kernel is booted.
    // This is useful for testing the kernel code without having to run it on real hardware.
    #[cfg(test)]
    start_test();

    printlnc!(
        " zerOS x86_64 kernel",
        ColorCode::new(Color::LightGreen, Color::Black)
    );

    printc!(" kernel loaded", ColorCode::new(Color::White, Color::Black));

    let vga_buffer = 0xb8000 as *mut u8; // The address of the VGA buffer in memory.

    for (i, &byte) in TO_PRINT.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // Code Page 437 character
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // Light cyan color
        }
    }

    loop {}
}
