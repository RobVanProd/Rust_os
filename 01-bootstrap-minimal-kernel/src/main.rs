#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer; // Module for VGA buffer printing

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_buffer::print_str("PANIC! "); // Added a space for readability
    if let Some(s) = info.payload().downcast_ref::<&str>() {
        vga_buffer::print_str(s);
    } else {
        vga_buffer::print_str("[no message]"); // Made it more distinct
    }
    vga_buffer::print_str("\n"); // Ensure panic message ends with a newline

    loop {} // Corrected: single braces for Rust code
}

/// Entry point of the kernel
#[no_mangle] // Don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    vga_buffer::print_str("Hello, world!\n"); // Added newline
    vga_buffer::print_str("This is the bootstrap kernel.\n"); // Added newline

    // To test panic: uncomment the line below
    // panic!("This is a test panic from _start!");

    loop {} // Corrected: single braces for Rust code
}
