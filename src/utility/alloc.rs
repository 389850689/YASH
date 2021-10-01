use core::convert::TryInto;
use core::alloc::{GlobalAlloc, Layout};

use lazy_static::lazy_static;
use crate::include::interop::Allocate::*;

#[global_allocator]
static GLOBAL: AllocatePool = AllocatePool { tag: None, tag_created: false };

struct AllocatePool {
    tag: Option<u32>,
    tag_created: bool
}

impl AllocatePool {
    fn create_tag(&mut self, tag: &'static str) {
            /// Turns string into u32.
            /// 
            /// Uses u32 function `from_le_bytes` to turn a 4 byte array of u8 values
            /// into a single u32 value; to accomplish this we turn the string into a
            /// slice of bytes and try_into to convert it according to the argument type.
            self.tag = Some(u32::from_le_bytes(tag.as_bytes().try_into().unwrap()));
            self.tag_created = true;
        }
    }
}

unsafe impl GlobalAlloc for AllocatePool {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // NOTE: this is very questionable, and is subject to change.


        let allocation = ExAllocatePool2(POOL_FLAGS::POOL_FLAG_PAGED as u64, 
            layout.size(), self.tag);
        
        if allocation.is_null() { panic!() }
        
    
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        
    }
}