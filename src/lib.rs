// lib.rs is a separate compilation unit. we need no_std again.
#![no_std]

// Conditinoally enable no_main while testing
#![cfg_attr(test, no_main)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// Enable x86_interrupt calling convention
#![feature(abi_x86_interrupt)]

#![feature(alloc_error_handler)]

extern crate alloc;

// Make print and serial_print available
// pub makes the modules available from outside
pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub mod allocator;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
//static ALLOCATOR: allocator::Dummy = allocator::Dummy;

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub fn init() {
    // Initialize the GDT
    gdt::init();

    // Initialize the IDT
    interrupts::init_idt();

    // Initialize the PIC
    unsafe {
        interrupts::PICS.lock().initialize()
    };
    x86_64::instructions::interrupts::enable();
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

// Avoid the loop {} consumes CPU
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
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
    hlt_loop();
}

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry point for `cargo xtest`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    // Initialize the IDT before running tests.
    init();

    test_main();
    hlt_loop();
}

/// Panic Handler for `cargo xtest`
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}