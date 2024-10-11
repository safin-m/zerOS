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
#![test_runner(crate::run_test)]
// This attribute reexports the test harness main function as start_test.
// This is necessary for running tests because the test harness expects the main function to be named start_test.
#![reexport_test_harness_main = "start_test"]

// Importing PanicInfo from the core library, which is a subset of the standard library that can be used in no_std environments.
// PanicInfo provides information about panics, which are critical errors.
use core::panic::PanicInfo;

// Importing the println macro from the vga module.
// This macro is used for printing formatted text to the screen in VGA text mode.
use modules::vga;
use vga::{Color, ColorCode};

// Declaring a module named vga inside the modules module.
// This is used for handling VGA text mode, which is a common way to output text to the screen in early stages of OS development.
mod modules {
    pub mod panic_handler;
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
fn panic(info: &PanicInfo) -> ! {
    let panic_color = ColorCode::new(Color::Black, Color::LightRed);
    if let Some(panic_info) = info.payload().downcast_ref::<&str>() {
        println_with_color!(panic_color, "Panic occurred: {}", panic_info);
    } else {
        println_with_color!(panic_color, "Panic occurred with no message.");
    }
    loop {}
}

// The test_case attribute is used to define a test function.
// This attribute is used to mark functions as test cases, which are run when testing the code.
#[test_case]
// A trivial test case that asserts that 1 is equal to 1.
// This test is used to verify that the test framework is working correctly.
fn trivial_assertion() {
    print!(" trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

// The #[cfg(test)] attribute is used to conditionally compile the test code only when running tests.
// This is necessary because we don't want to include test code in the final kernel binary.
#[cfg(test)]
// This function is used to run tests in OS development.
// It takes a slice of test functions as input and runs each test in sequence.
// The tests are defined as functions that take no arguments and return nothing.
//
// # Arguments
// - `tests` : A slice of test functions to run.
//
// The test functions are defined in the test module, which is a convention in Rust for organizing tests.
pub fn run_test(tests: &[&dyn Fn()]) {
    use modules::panic_handler::{exit_os, OSExitCode};

    println!(" Running tests");
    for test in tests {
        test();
    }
    println!("", ColorCode::new(Color::Black, Color::Black));
    exit_os(OSExitCode::Success);
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

    println!(
        " zerOS x86_64 kernel",
        ColorCode::new(Color::LightGreen, Color::Black)
    );

    print!(" kernel loaded", ColorCode::new(Color::White, Color::Black));

    let vga_buffer = 0xb8000 as *mut u8; // The address of the VGA buffer in memory.

    for (i, &byte) in TO_PRINT.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // Code Page 437 character
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // Light cyan color
        }
    }

    loop {}
}
