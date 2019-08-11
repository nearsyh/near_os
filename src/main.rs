#![no_std]     // Don't use standard library
#![no_main]    // Don't use the normal entry point in Rust

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // the ! means never returning
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]    // Disable name mangling so that the function name is used
pub extern "C" fn _start() -> ! {
    // extern "C" tells the compiler to use C calling convention
    // _start is just a name convention

    // Cast 0xb8000 as a raw pointer.
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        // Use unsafe because the raw pointer can't be proved safe.
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}