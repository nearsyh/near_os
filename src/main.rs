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

    // map a previously unmapped page
    let page = Page::containing_address(VirtAddr::new(0x1000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

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
