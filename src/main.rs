#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[cfg(target_os = "linux")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[cfg(target_os = "windows")]
#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    main();
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
