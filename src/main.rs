#![no_std]     // Don't use standard library
#![no_main]    // Don't use the normal entry point in Rust

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // the ! means never returning
    loop {}
}

#[no_mangle]    // Disable name mangling so that the function name is used
pub extern "C" fn _start() -> ! {
    // extern "C" tells the compiler to use C calling convention
    // _start is just a name convention
    loop {}
}