#![no_std]
#![no_main]
#![deny(warnings)]

use blog_os::{exit_qemu, ExitCode, serial_println};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    blog_os::interrupts::init_idt();

    x86_64::instructions::interrupts::int3();

    unsafe {
        exit_qemu(ExitCode::Success);
    }
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);

    unsafe {
        exit_qemu(ExitCode::Failure);
    }
    loop {}
}
