#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::run_test)]
#![reexport_test_harness_main = "start_test"]

pub mod modules {
    pub mod panic_handler;
    pub mod uart;
    pub mod vga;
}

use core::panic::PanicInfo;
use modules::uart;
use modules::vga;

// The Testable trait is used to define testable types.
// This trait provides a run method that runs the test.
pub trait Testable {
    fn run(&self) -> ();
}

/// Implement the Testable trait for functions that take no arguments and return nothing.
/// This allows us to define test functions as functions that take no arguments and return nothing.
/// The run method is called to run the test.
/// The serial_print and serial_println macros are used to print formatted text to the serial port.
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

// This function is used to run tests in OS development.
// It takes a slice of test functions as input and runs each test in sequence.
// The tests are defined as functions that take no arguments and return nothing.
//
// # Arguments
// - `tests` : A slice of test functions to run.
//
// The test functions are defined in the test module, which is a convention in Rust for organizing tests.
pub fn run_test(tests: &[&dyn Testable]) {
    use modules::panic_handler::{exit_os, OSExitCode};

    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_os(OSExitCode::Success);
}

/// The panic handler for tests.
/// This function is used to handle panics that occur during testing.
/// It prints an error message and exits the test with a failure code.
/// The function then enters an infinite loop, effectively halting the system.
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    use modules::panic_handler::{exit_os, OSExitCode};
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_os(OSExitCode::Fail);
    loop {}
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    start_test();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

// The test_case attribute is used to define a test function.
// This attribute is used to mark functions as test cases, which are run when testing the code.
#[test_case]
// A trivial test case that asserts that 1 is equal to 1.
// This test is used to verify that the test framework is working correctly.
fn trivial_assertion() {
    assert_eq!(1, 1);
}
