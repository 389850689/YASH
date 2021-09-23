#![no_std]

/// causes linker errors when using floating point instructions 
/// workaround: https://github.com/rust-lang/rust/issues/62785/
#[used]
#[no_mangle]
pub static _fltused: i32 = 0;

mod log;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "system" fn driver_entry() -> u32 {

    let x = ["wow", "very", "nice"];
    dbg_print!("{:#?}", x);

    0 /* STATUS_SUCCESS */
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}