#![no_std]
#![no_main]
#![deny(warnings)]

use blog_os::{exit_qemu, ExitCode};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        exit_qemu(ExitCode::Success);
    }
    loop {}
}
