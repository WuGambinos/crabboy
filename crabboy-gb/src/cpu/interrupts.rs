use crate::constants::{INTERRUPTS, INTERRUPT_ENABLE, INTERRUPT_FLAG};
use crate::interconnect::Interconnect;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InterruptType {
    VBlank,
    LcdStat,
    Timer,
    Serial,
    Joypad,
}

pub fn is_interrupt_enabled(interconnect: &mut Interconnect, index: usize) -> bool {
    let interrupt_enable = interconnect.read_mem(INTERRUPT_ENABLE);
    (interrupt_enable & (1 << index)) > 0
}

pub fn is_interrupt_requested(interconnect: &mut Interconnect, index: usize) -> bool {
    let interrupt_flag = interconnect.read_mem(INTERRUPT_FLAG);
    (interrupt_flag & (1 << index)) > 0
}

pub fn request_interrupt(interconnect: &mut Interconnect, interrupt: InterruptType) {
    let mut interrupt_request = interconnect.read_mem(INTERRUPT_FLAG);
    let nth_bit = INTERRUPTS.iter().position(|&i| i == interrupt).unwrap();
    interrupt_request |= 1 << nth_bit;
    interconnect.write_mem(INTERRUPT_FLAG, interrupt_request);
}

pub fn get_interrupt(interconnect: &mut Interconnect) -> Option<InterruptType> {
    for (i, interrupt) in INTERRUPTS.iter().enumerate() {
        if is_interrupt_enabled(interconnect, i) && is_interrupt_requested(interconnect, i) {
            return Some(*interrupt);
        }
    }
    None
}
