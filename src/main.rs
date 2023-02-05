// main.rs

#![no_std] // dont link std library
#![no_main] // disable rust entry point

use core::panic::PanicInfo;

#[no_mangle] // what's mangling?
pub extern "C" fn _start() -> ! {
    loop {}
}

// panic handler 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}