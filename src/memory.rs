use x86_64::structures::paging::PageTable;
use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::{PhysFrame, MapperAllSizes, MappedPageTable};

/// Initialize a new MappedPageTable
pub unsafe fn init(physical_memory_offset: u64) -> impl MapperAllSizes {
    let level_4_table = active_level_4_table(physical_memory_offset);
    let phys_to_virt = move |frame: PhysFrame| -> *mut PageTable {
        let phys = frame.start_address().as_u64();
        let virt = VirtAddr::new(phys + physical_memory_offset);
        virt.as_mut_ptr()
    };
    MappedPageTable::new(level_4_table, phys_to_virt)
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_level_4_table(physical_memory_offset: u64)
    -> &'static mut PageTable {

    use x86_64::registers::control::Cr3;

    // Get Physical Address of Level 4 Page Table
    let (level_4_table_frame, _) = Cr3::read();
    let phys = level_4_table_frame.start_address();

    // Translate to Virtual Address
    let virt = VirtAddr::new(phys.as_u64() + physical_memory_offset);
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

use x86_64::structures::paging::{Page, Size4KiB, Mapper, FrameAllocator};

pub fn create_example_mapping(
    page: Page,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    
    // flush() updates the TLB
    map_to_result.expect("map_to failed").flush();
}

use bootloader::bootinfo::MemoryMap;

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    // Create a FrameAllocator
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }
}

use bootloader::bootinfo::MemoryRegionType;

impl BootInfoFrameAllocator {
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // Filter
        let regions = self.memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        // Map region to address
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
        // transform to frame iterator
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // create physframe
        frame_addresses
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

// Dummy Allocator
pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

// ===============================================================
// ===============================================================

/// Translates the virtual address to physical address or None if not mapped.
/// Our own implementation
unsafe fn _translate_addr(addr: VirtAddr, physical_memory_offset: u64)
    -> Option<PhysAddr> {
    _translate_addr_inner(addr, physical_memory_offset)
}

// Private implemetation.
fn _translate_addr_inner(addr: VirtAddr, physical_memory_offset: u64)
    -> Option<PhysAddr> {
    
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;

    // Read the active level 4 frame from the CR3 register
    let (level_4_table_frame, _) = Cr3::read();

    // Index of each level of page table
    let table_indexes = [
        addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
    ];

    let mut frame = level_4_table_frame;

    // traverse the multi-level page table
    for &index in &table_indexes {
        // convert the frame into a page table reference
        let virt = frame.start_address().as_u64() + physical_memory_offset;
        let table_ptr: *const PageTable = VirtAddr::new(virt).as_ptr();
        let table = unsafe {&*table_ptr};

        // read the page table entry and update frame
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("hube pages not supported"),
        };
    }

    // calculate the physical address by adding the page offset
    Some(frame.start_address() + u64::from(addr.page_offset()))
}