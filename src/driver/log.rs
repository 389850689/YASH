///!
///! NOTE: I might completely remove this implementation of printing (implementing fmt::Write)
///! and instead create a wrapper for kernel heap allocations for collections
///! 

use core::fmt::{self, Write};
pub use winapi::km::wdm::DbgPrint;

/// Sends a message to the kernel debugger.
/// 
/// This uses rust's default string formatting style.
#[macro_export]
macro_rules! dbg_print {
    ($($arg:tt)*) => ($crate::log::_print(format_args!($($arg)*)));
}

/// Wrapper struct to provide an API for writing a string to an array.
/// More importantly, after we implement the fmt::Write trait we can 
/// use rust's format strings on byte arrays.
#[doc(hidden)]
struct Wrapper<'a> {
    buf: &'a mut [u8],
}

impl<'a> Wrapper<'a> {
    fn new(buf: &'a mut [u8]) -> Self { Self { buf } }
}

impl fmt::Write for Wrapper<'_>
{
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        // TODO: fix bug that makes it so that format (newlines) actually work
        // TODO: fix bug that causes undefined behavior when copying exactly 0x1000 non-zero values to the buffer
        self.buf[..s.len()].copy_from_slice(s.as_bytes());
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    // buffer to place string in, minus 1, for the null term
    let mut buffer: [u8; 0x1000] = [0; 0x1000];
    
    // write format arguments string to buffer
    Wrapper::new(&mut buffer).write_fmt(args).unwrap();

    // NOTE: I believe this is considered bad practice, even though it really isn't unsafe.
    unsafe { DbgPrint(&buffer as *const u8); }
}