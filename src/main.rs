// This attribute tells the Rust compiler that the standard library is not available.
// This is crucial for OS development because the standard library depends on an underlying operating system, which we are trying to build.
#![no_std]
// This attribute indicates that the usual main function is not used.
// Instead, a custom entry point is defined. This is necessary for low-level programming like OS development where we need full control over the entry point.
#![no_main]

// Importing PanicInfo from the core library, which is a subset of the standard library that can be used in no_std environments.
// PanicInfo provides information about panics, which are critical errors.
use core::panic::PanicInfo;

// Declaring a module named vga inside the modules module.
// This is used for handling VGA text mode, which is a common way to output text to the screen in early stages of OS development.
mod modules {
    pub mod vga;
}

// This attribute marks the following function as the panic handler, which is called whenever a panic occurs.
// In OS development, we need to define our own panic handler because the default one in the standard library is not available.
#[panic_handler]
// The panic handler function. The ! indicates that this function will never return.
// In this case, it enters an infinite loop, effectively halting the system.
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static TO_PRINT: &[u8] = b"Booting the kernel...\n";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in TO_PRINT.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
