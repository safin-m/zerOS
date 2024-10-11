#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(zer_os::run_test)]
#![reexport_test_harness_main = "start_test"]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    start_test();

    loop {}
}

fn run_test(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    zer_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
