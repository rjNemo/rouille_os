#![no_std]

use core::panic::PanicInfo;

fn main() {}
/// Function called on panic. It is a diverging function with a `never` return type.
///
/// # Arguments
/// - `info`- contains the file and line where the panic happened and the optional panic message
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
