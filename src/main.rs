#![no_std] // Do not import Rust standard library. 
#![no_main] // Disable rust level entry points
#![feature(abi_x86_interrupt)]

// Import for target
mod kernel;
mod os;

use core::panic::PanicInfo;
use kernel::lib::hlt_loop;
use kernel::vga_driver;
use kernel::vga_driver::Color;

// Put the version and distro name publically available everywhere.
pub const K_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DISTNAME: &str = env!("CARGO_PKG_NAME");



#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_driver::WRITER.lock().set_color(Color::Black, Color::White);
    println!("Welcome to my Bootloader/Kernel!");
    vga_driver::WRITER.lock().set_color(Color::White, Color::Black);
    println!("Distribution: \"{}\" - v{}", DISTNAME, K_VERSION);

    kernel::lib::init();

    hlt_loop();

}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_driver::draw_panic_message(format_args!("Ref: {}" ,info));
    hlt_loop();
}