#![no_std] // Do not import Rust standard library. 
#![no_main] // Disable rust level entry points
#![feature(abi_x86_interrupt)]

// Important Bootloader specific
use bootloader_api::{entry_point, BootInfo};
use bootloader_api::config::BootloaderConfig;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let config = BootloaderConfig::new_default();
    //config.mappings.
    config
};


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

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    
    println!("Kernel size: {}K PhyAddress: 0x{:X}", boot_info.kernel_len, boot_info.kernel_addr);
    println!("Kernel: Basic frame buffer established at: {:#?}", boot_info.framebuffer);
    println!("Kernel: UEFI RSDP scan found: {:?}", boot_info.rsdp_addr);
    println!("Memory regions initialized: {:#?}", boot_info.memory_regions);

    kernel::lib::init();


    vga_driver::WRITER.lock().set_color(Color::Black, Color::White);
    println!("Welcome to my Bootloader/Kernel!");
    vga_driver::WRITER.lock().set_color(Color::White, Color::Black);
    println!("Distribution: \"{}\" - v{}", DISTNAME, K_VERSION);
    hlt_loop();

}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_driver::draw_panic_message(format_args!("Ref: {}" ,info));
    hlt_loop();
}