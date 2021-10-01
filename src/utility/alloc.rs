use core::convert::TryInto;
use core::alloc::{GlobalAlloc, Layout};

use crate::include::interop::allocate::*;

#[global_allocator]
static GLOBAL: AllocatePool = AllocatePool { name: "Tag1" };

/// Turns a string into a u32.
/// 
/// Uses u32 function `from_le_bytes` to turn a 4 byte array of u8 values
/// into a single u32 value; to accomplish this we turn the string into a
/// slice of bytes and try_into to convert it according to the argument type.
fn tagify(name: &'static str) -> u32 {
    u32::from_le_bytes(name.as_bytes().try_into().unwrap())
}

struct AllocatePool {
    name: &'static str,
}

unsafe impl GlobalAlloc for AllocatePool {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // IMPORTANT: So it is possible that ExAllocatePool2 will return
        // NULL and in the future it might be important to handle this
        // case maybe using alloc_error_handler, logging, or otherwise,
        // however, as of right now it will panic regardless of if we 
        // explicitly call panic or not.
        ExAllocatePool2(POOL_FLAGS::POOL_FLAG_PAGED as u64, layout.size(), 
            tagify(self.name)) as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        ExFreePoolWithTag(ptr as _, tagify(self.name))
    }
} 