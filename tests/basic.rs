#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(zer_os::run_test)]
#![reexport_test_harness_main = "start_test"]
extern crate zer_os;
use zer_os::println;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    zer_os::test_panic_handler(info)
}

#[test_case]
fn test_printing_functionality() {
    println!("it is printing");
}
