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
        idt
    };
}

pub fn init_idt(){
    IDT.load();
}

extern "x86-interrupt" fn system_timer(
    stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: Breakpoint\n{:#?}", stack_frame);
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