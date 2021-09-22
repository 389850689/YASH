pub use winapi::km::wdm::DbgPrint;

/// Sends a message to the kernel debugger.
/// 
/// This uses rust's default string formatting style.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::log::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
fn _print(args: core::fmt::Arguments) {
    
}