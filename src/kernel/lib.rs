use crate::{kernel::interrupts, println};


static mut snapShotMemory

pub struct memorySnapShot{
    timeStamp: u64, // Ticks recorded from timer

}


pub fn init(){
    println!("Kernel: configuring memory structure...");



    let (high_page, _ ) = x86_64::registers::control::Cr3::read();
    println!("Kernel Memory: {:?}", high_page);

    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize()};
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

