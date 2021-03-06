#![no_std]

/// causes linker errors when using floating point instructions 
/// workaround: https://github.com/rust-lang/rust/issues/62785/
#[used]
#[no_mangle]
pub static _fltused: i32 = 0;

pub mod utility;
pub mod include;

/// Program Entry Point.
#[no_mangle]
pub extern "system" fn driver_entry() -> u32 {


    0 /* STATUS_SUCCESS */
}

/// Called when requested to unload the driver.
pub unsafe extern "C" fn driver_unload() { }