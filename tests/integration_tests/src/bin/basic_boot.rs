#![no_std]
#![no_main]

use blog_os::{exit_qemu, ExitCode, serial_println};
use core::panic::PanicInfo;

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    unsafe {
        exit_qemu(ExitCode::Success);
    }
    loop {}
}

/// This function is called on panic.

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);

    unsafe {
        exit_qemu(ExitCode::Failure);
    }
    loop {}
}
