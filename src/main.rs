#![no_std] // Do not import Rust standard library. 
#![no_main] // Disable rust level entry points

// Put the version and distro name publically available everywhere.
pub const K_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DISTNAME: &str = env!("CARGO_PKG_NAME");

use core::panic::PanicInfo;

use vga_buffer::draw_panic_message;
use vga_buffer::Color;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::WRITER.lock().set_color(Color::Black, Color::White);
    println!("Welcome to my Bootloader/Kernel!");
    vga_buffer::WRITER.lock().set_color(Color::White, Color::Black);
    println!("Distribution: \"{}\" - v{}", DISTNAME, K_VERSION);

    baremetal::init();
    println!("Kernel has handled Interrupt.");

    loop{}

}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    draw_panic_message(format_args!("Ref: {}" ,info));
    loop{}
}