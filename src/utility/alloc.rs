use core::convert::TryInto;
use core::alloc::{GlobalAlloc, Layout};

use lazy_static::lazy_static;
use crate::include::interop::Allocate::*;

struct AllocatePool {
    tag: u32
}

impl AllocatePool {
    const fn new(tag: &'static str) -> Self {
        Self {
            /// Turns string into u32.
            /// 
            /// Uses u32 function `from_le_bytes` to turn a 4 byte array of u8 values
            /// into a single u32 value; to accomplish this we turn the string into a
            /// slice of bytes and try_into to convert it according to the argument type.
            #[feature(const_panic)]
            tag: u32::from_le_bytes(tag.as_bytes().try_into().unwrap())
        }
    }
}

unsafe impl GlobalAlloc for AllocatePool {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let allocation = ExAllocatePool2(POOL_FLAGS::POOL_FLAG_PAGED as u64, 
            layout.size(), self.tag);
        
        if allocation.is_null() { panic!() }
        
        allocation as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        
    }
}

lazy_static! {
    //#[global_allocator]
    static ref GLOBAL: AllocatePool = AllocatePool::new("Test");
}