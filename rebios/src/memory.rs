use x86_64::{
    PhysAddr,
    VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB
    }
};
use limine::response::MemoryMapResponse;
use limine::memory_map::EntryType;

/// A FrameAllocator that returns usable frames from Limine's memory map response.
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMapResponse,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a FrameAllocator from a Limine MemoryMapResponse.
    ///
    /// Safety: caller must guarantee that the Limine-provided memory map is valid and
    /// the entries marked usable are actually free to allocate.
    pub unsafe fn init(memory_map: &'static MemoryMapResponse) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> + '_ {
        // entries() -> &[&Entry]
        self.memory_map.entries().iter()
            // keep usable and bootloader-reclaimable (common pattern)
            .filter(|entry| {
                (entry.entry_type == EntryType::USABLE)
                || (entry.entry_type == EntryType::BOOTLOADER_RECLAIMABLE)
            })
            .flat_map(|entry| {
                // entry.base and entry.length are u64
                let start = entry.base as usize;
                let end = (entry.base + entry.length) as usize;
                // iterate frame-aligned addresses
                (start..end).step_by(4096)
            })
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr as u64)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

// rest of file unchanged (create_example_mapping, init, active_level_4_table, etc.)
pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>
){
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static>
{
    unsafe {
        let level_4_table = active_level_4_table(physical_memory_offset);
        OffsetPageTable::new(level_4_table, physical_memory_offset)
    }
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable 
{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}
