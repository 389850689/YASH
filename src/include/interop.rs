// NOTE: add some type aliases here later, I guess.

pub mod Allocate {
    #[allow(non_camel_case_types)]
    pub enum POOL_FLAGS {
       POOL_FLAG_REQUIRED_START     =  0x0000000000000001,
       /* POOL_FLAG_USE_QUOTA       =  0x0000000000000001, // Charge quota */
       POOL_FLAG_UNINITIALIZED      =  0x0000000000000002, // Don't zero-initialize allocation
       POOL_FLAG_SESSION            =  0x0000000000000004, // Use session specific pool
       POOL_FLAG_CACHE_ALIGNED      =  0x0000000000000008, // Cache aligned allocation
       POOL_FLAG_RESERVED1          =  0x0000000000000010, // Reserved for system use
       POOL_FLAG_RAISE_ON_FAILURE   =  0x0000000000000020, // Raise exception on failure
       POOL_FLAG_NON_PAGED          =  0x0000000000000040, // Non paged pool NX
       POOL_FLAG_NON_PAGED_EXECUTE  =  0x0000000000000080, // Non paged pool executable
       POOL_FLAG_PAGED              =  0x0000000000000100, // Paged pool
       POOL_FLAG_RESERVED2          =  0x0000000000000200, // Reserved for system use
       POOL_FLAG_RESERVED3          =  0x0000000000000400, // Reserved for system use
       POOL_FLAG_REQUIRED_END       =  0x0000000080000000,
       POOL_FLAG_OPTIONAL_START     =  0x0000000100000000,
       /* POOL_FLAG_SPECIAL_POOL    =  0x0000000100000000, // Make special pool allocation */
       POOL_FLAG_OPTIONAL_END       =  0x8000000000000000,
    }

    // NOTE: using system as the calling convention here is alright, however, it might be more clear to change it to C.
    extern "system" {
        pub fn ExAllocatePool2(Flags: u64, NumberOfBytes: usize, Tag: u32) -> *mut core::ffi::c_void;
    }
}
