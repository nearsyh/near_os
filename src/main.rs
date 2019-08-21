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

extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

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

use bootloader::{BootInfo, entry_point};

// Make sure the entry point has the write argument type
// It defines the real low-level _start function
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // extern "C" tells the compiler to use C calling convention
    // _start is just a name convention
    println!("Hello World{}", "!");

    // Initialize our OS
    near_os::init();

    use near_os::memory;
    use x86_64::{VirtAddr, structures::paging::Page};

    let mut mapper = unsafe { memory::init(boot_info.physical_memory_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    use near_os::allocator;
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));let x = Box::new(42);

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

    println!("Not Crash");
    near_os::hlt_loop();
}
