// lib.rs is a separate compilation unit. we need no_std again.
#![no_std]

// Conditinoally enable no_main while testing
#![cfg_attr(test, no_main)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// Enable x86_interrupt calling convention
#![feature(abi_x86_interrupt)]

// Make print and serial_print available
// pub makes the modules available from outside
pub mod serial;
pub mod vga_buffer;
pub mod interrupts;

pub fn init() {
    // Initialize the IDT
    interrupts::init_idt();
}

// Qemu Helper Functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

use core::panic::PanicInfo;

// Test runner
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

// Panic Handler
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Entry point for `cargo xtest`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize the IDT before running tests.
    init();

    test_main();
    loop {}
}

/// Panic Handler for `cargo xtest`
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}