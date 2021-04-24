#![no_std]
#![no_main]
use core::panic::PanicInfo;

/// Disabling mangling ensures the compiler returns a function named `_start`
///
/// We tell the compiler to use `C calling convention`
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
/// Function called on panic. It is a diverging function with a `never` return type.
///
/// # Arguments
/// - `info`- contains the file and line where the panic happened and the optional panic message
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
