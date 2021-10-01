use core::convert::TryInto;
use core::alloc::{GlobalAlloc, Layout};

use lazy_static::lazy_static;
use crate::include::interop::Allocate::*;

#[global_allocator]
static GLOBAL: AllocatePool = AllocatePool { name: "Test" };

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
        let allocation = ExAllocatePool2(POOL_FLAGS::POOL_FLAG_PAGED as u64, 
            layout.size(), tagify(self.name));

        return if !allocation.is_null() { allocation as _ } else { panic!() } 
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        
    }
} 