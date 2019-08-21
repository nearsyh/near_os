use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

pub const HEAP_START: usize = 0x_4444_4444_0000;  // Virtual Address
pub const HEAP_SIZE: usize = 100 * 1024;          // 100 KiB

use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper,
        Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

// Map Heap Region to Physical Memory
pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError> {
    // Get all pages
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    // Map each page
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        // flush() refreshs TLB
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    unsafe {
        super::ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _prt: *mut u8, _layout: Layout) {
        panic!("dealloc should never be called")
    }
}