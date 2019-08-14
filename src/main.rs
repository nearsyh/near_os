#![no_std]     // Don't use standard library
#![no_main]    // Don't use the normal entry point in Rust

// Use custom test framework because the default one requires std
#![feature(custom_test_frameworks)]
// Set test runner
#![test_runner(near_os::test_runner)]
// Set the main function for testing
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use near_os::println;

// Conditional Compilation
// Non Test Panic Handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // the ! means never returning
    println!("{}", info);
    loop {}
}

// Test Panic Handler
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    near_os::test_panic_handler(info)
}

#[no_mangle]    // Disable name mangling so that the function name is used
pub extern "C" fn _start() -> ! {
    // extern "C" tells the compiler to use C calling convention
    // _start is just a name convention
    println!("Hello World{}", "!");

    // Only run while testing
    #[cfg(test)]
    test_main(); // This function is auto-generated

    loop {}
}
