use core::convert::TryInto;
use core::alloc::{GlobalAlloc, Layout};

use crate::include::interop::Allocate::*;

struct AllocatePool {
    tag: &'static str
}

unsafe impl GlobalAlloc for AllocatePool {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // The last argument is extremely questionable, and at some point, should be changed.
        let allocation = ExAllocatePool2(POOL_FLAGS::POOL_FLAG_PAGED as u64, 
            layout.size(), u32::from_be_bytes(self.tag.as_bytes().try_into().unwrap()));
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        
    }
}