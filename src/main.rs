#![no_std] // Do not import Rust standard library. 
#![no_main] // Disable rust level entry points


use core::panic::PanicInfo;
mod vga_buffer;

static HELLO: &[u8] = b"Hello World";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}