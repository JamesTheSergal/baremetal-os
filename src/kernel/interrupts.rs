use crate::print;
use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use pic8259::ChainedPics;
use spin;


// Establish offsets for our PIC because the default lines overlap with our 
// Inturrupts
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PICInterruptIndex{
    Timer = PIC_1_OFFSET, // Timer is on Line 0
    Keyboard = PIC_1_OFFSET + 1
}

impl PICInterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)});



lazy_static!{
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt[PICInterruptIndex::Timer.as_usize()].set_handler_fn(system_timer);
        idt[PICInterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_handler);
        idt
    };
}

pub fn init_idt(){
    println!("Kernel: Loading InterruptDescriptorTable (IDT)...");
    IDT.load();
    println!("Kernel: Setup IDT and Legacy APCI Mode.")
}

extern "x86-interrupt" fn system_timer(
    _stack_frame: InterruptStackFrame)
{
    print!(".");
    unsafe {
        PICS.lock().notify_end_of_interrupt(PICInterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_handler(
    _stack_frame: InterruptStackFrame)
{
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(ScancodeSet1::new(),
                layouts::Us104Key, HandleControl::Ignore)
            );
    }

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe{port.read()};
    let mut keyboard = KEYBOARD.lock();


    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }
    
    unsafe {
        PICS.lock().notify_end_of_interrupt(PICInterruptIndex::Keyboard.as_u8());
    }
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: Breakpoint\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    panic!("DOUBLE FAULT\n{:#?}", stack_frame)
}