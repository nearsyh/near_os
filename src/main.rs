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

    near_os::hlt_loop();
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

    // Initialize our OS
    near_os::init();

    // ===============================================
    // Trigger a breakpoint exception
    // x86_64::instructions::interrupts::int3();
    // ===============================================

    // ===============================================
    // Trigger a page fault
    // unsafe {
        // *(0xdeadbeef as *mut u64) = 42;
    // }
    // ===============================================

    // ===============================================
    // Trigger a stack overflow
    // fn stack_overflow() {
        // stack_overflow();
    // }
    // stack_overflow();
    // ===============================================

    println!("Not Crash");

    // Only run while testing
    #[cfg(test)]
    test_main(); // This function is auto-generated

    // loop {
        // ==============================================
        // Used to trigger a dead lock
        // use near_os::print;
        // print!("-");
        // ==============================================
    // }

    near_os::hlt_loop();
}
