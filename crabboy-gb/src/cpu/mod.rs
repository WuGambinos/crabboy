#![allow(clippy::must_use_candidate)]
mod execute;
mod instructions;
pub mod interrupts;

use log::debug;
use serde::{Deserialize, Serialize};

use crate::constants::{
    INTERRUPTS, INTERRUPT_ENABLE, INTERRUPT_FLAG, MAX_CYCLES_PER_FRAME, SERIAL_TRANSFER_CONTROL,
    SERIAL_TRASFER_DATA,
};
use crate::cpu::instructions::push_rr;
use crate::cpu::interrupts::{get_interrupt, InterruptType};
use crate::interconnect::Interconnect;

#[derive(Debug, Deserialize, Serialize)]
pub struct Flags {
    pub data: u8,
}

impl Flags {
    fn new() -> Self {
        Flags { data: 0xB0 }
    }

    pub fn clear_all(&mut self) {
        self.data = 0;
    }

    pub fn zero_flag(&self) -> u8 {
        (self.data >> 7) & 1
    }

    pub fn sub_flag(&self) -> u8 {
        (self.data >> 6) & 1
    }

    pub fn half_carry_flag(&self) -> u8 {
        (self.data >> 5) & 1
    }

    pub fn carry_flag(&self) -> u8 {
        (self.data >> 4) & 1
    }

    pub fn set_zero_flag(&mut self) {
        self.data |= 1 << 7;
    }

    pub fn clear_zero_flag(&mut self) {
        self.data &= !(1 << 7);
    }

    pub fn set_sub_flag(&mut self) {
        self.data |= 1 << 6;
    }

    pub fn clear_sub_flag(&mut self) {
        self.data &= !(1 << 6);
    }

    pub fn set_half_carry_flag(&mut self) {
        self.data |= 1 << 5;
    }

    pub fn clear_half_carry_flag(&mut self) {
        self.data &= !(1 << 5);
    }

    pub fn set_carry_flag(&mut self) {
        self.data |= 1 << 4;
    }

    pub fn clear_carry_flag(&mut self) {
        self.data &= !(1 << 4);
    }

    pub fn update_carry_flag_sum_8bit(&mut self, register: u8, operand: u8) {
        let res: u16 = (u16::from(register)) + (u16::from(operand));

        if res > 0xFF {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
    }

    pub fn update_carry_flag_sum_16bit(&mut self, register: u16, operand: u16) {
        let res: u32 = (u32::from(register)) + (u32::from(operand));

        if res > 0xFFFF {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
    }

    pub fn update_carry_flag_sub_8bit(&mut self, register: u8, operand: u8) {
        if register < operand {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
    }

    pub fn update_carry_flag_sub_16bit(&mut self, register: u16, operand: u16) {
        if register < operand {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
    }

    fn update_half_carry_flag_sum_8bit(&mut self, register: u8, operand: u8) {
        if ((register & 0xF) + (operand & 0xF)) > 0xF {
            self.set_half_carry_flag();
        } else {
            self.clear_half_carry_flag();
        }
    }

    fn update_half_carry_flag_sum_16bit(&mut self, register: u32, operand: u32) {
        let half_carry: bool = ((register & 0x0FFF) + (operand & 0x0FFF)) > 0x0FFF;

        if half_carry {
            self.set_half_carry_flag();
        } else {
            self.clear_half_carry_flag();
        }
    }

    fn update_half_carry_flag_sub_8bit(&mut self, register: u8, operand: u8) {
        if (register & 0xF) < (operand & 0xF) {
            self.set_half_carry_flag();
        } else {
            self.clear_half_carry_flag();
        }
    }

    pub fn update_half_carry_flag_sub_16bit(&mut self, register: u16, operand: u16) {
        if (register & 0xFFF) < (operand & 0xFFF) {
            self.set_half_carry_flag();
        } else {
            self.clear_half_carry_flag();
        }
    }

    fn update_zero_flag(&mut self, v: u8) {
        if v == 0 {
            self.set_zero_flag();
        } else {
            self.clear_zero_flag();
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: Flags,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            f: Flags::new(),
        }
    }

    pub fn bc(&self) -> u16 {
        u16::from_be_bytes([self.b, self.c])
    }

    pub fn set_bc(&mut self, data: u16) {
        let [b, c] = data.to_be_bytes();
        self.b = b;
        self.c = c;
    }

    pub fn de(&self) -> u16 {
        u16::from_be_bytes([self.d, self.e])
    }

    pub fn set_de(&mut self, data: u16) {
        let [d, e] = data.to_be_bytes();
        self.d = d;
        self.e = e;
    }

    pub fn hl(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l])
    }

    pub fn set_hl(&mut self, data: u16) {
        let [h, l] = data.to_be_bytes();
        self.h = h;
        self.l = l;
    }

    pub fn af(&self) -> u16 {
        u16::from_be_bytes([self.a, self.f.data])
    }

    pub fn set_af(&mut self, data: u16) {
        let [a, f] = data.to_be_bytes();
        self.a = a;
        self.f.data = f;
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum RegisterPair {
    BC,
    DE,
    HL,
    AF,
    SP,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cpu {
    pub registers: Registers,
    pub sp: u16,
    pub pc: u16,
    pub ime: bool,
    pub ime_to_be_enabled: bool,
    pub halted: bool,
    pub opcode: u8,
    pub last_cycle: u64,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
            sp: 0xFFFE,
            pc: 0,
            ime: false,
            ime_to_be_enabled: false,
            halted: false,
            opcode: 0,
            last_cycle: 0,
        }
    }

    pub fn run(&mut self, interconnect: &mut Interconnect) {
        let mut cycles_this_update = 0;

        while cycles_this_update < MAX_CYCLES_PER_FRAME {
            let cycles = self.last_cycle as usize;
            cycles_this_update += cycles;

            if self.pc == 0x100 {
                interconnect.write_enabled = false;
                interconnect.boot_active = false;
            }
            let running = !self.halted;

            if running {
                self.execute_instruction(interconnect);

                if interconnect.read_mem(SERIAL_TRANSFER_CONTROL) == 0x81 {
                    let c: char = interconnect.read_mem(SERIAL_TRASFER_DATA) as char;
                    interconnect.serial.write_byte(c as u8);
                    interconnect.serial.output();
                    interconnect.write_mem(SERIAL_TRANSFER_CONTROL, 0x00);
                }
            } else {
                interconnect.emu_tick(1);

                let interrupt_flag = interconnect.read_mem(INTERRUPT_FLAG);
                let interrupt_resquestd = interrupt_flag != 0;

                if interrupt_resquestd {
                    self.halted = false;
                }
            }
        }
    }

    pub fn handle_interrupt(&mut self, interconnect: &mut Interconnect) {
        let interrupts_enabled = self.ime || self.halted;
        if interrupts_enabled {
            let triggered = get_interrupt(interconnect);
            if let Some(triggered_interrupt) = triggered {
                self.halted = false;
                if !self.ime {
                    return;
                }

                // Disable Interrupts
                self.ime = false;

                let n = INTERRUPTS
                    .iter()
                    .position(|&i| i == triggered_interrupt)
                    .unwrap();

                // Push Current PC onto stack
                let lower_pc = self.pc as u8;
                let upper_pc = (self.pc >> 8) as u8;
                push_rr(interconnect, upper_pc, lower_pc, &mut self.sp);

                // Set PC equal to address of handler
                self.pc = match triggered_interrupt {
                    InterruptType::VBlank => 0x40,
                    InterruptType::LcdStat => 0x48,
                    InterruptType::Timer => 0x50,
                    InterruptType::Serial => 0x58,
                    InterruptType::Joypad => 0x58,
                };

                // Clean up the interrupt
                let mut interrupt_flags = interconnect.read_mem(INTERRUPT_FLAG);
                interrupt_flags &= !(1 << n);
                interconnect.write_mem(INTERRUPT_FLAG, interrupt_flags);

                self.ime_to_be_enabled = false;
                interconnect.emu_tick(4);
            }
        }
    }

    pub fn fetch_opcode(&mut self, interconnect: &Interconnect) {
        self.opcode = interconnect.read_mem(self.pc);
    }

    pub fn get_u16(&mut self, interconnect: &Interconnect) -> u16 {
        u16::from_be_bytes([
            interconnect.read_mem(self.pc + 2),
            interconnect.read_mem(self.pc + 1),
        ])
    }

    pub fn log_registers(&self) {
        debug!(
            "A: {} B: {} C: {} D: {} E: {} H: {} L: {}",
            self.registers.a,
            self.registers.b,
            self.registers.c,
            self.registers.d,
            self.registers.e,
            self.registers.h,
            self.registers.l
        );
    }

    pub fn log_state(&self, interconnect: &Interconnect) {
        debug!("PC: {:#X}", self.pc);
        debug!("SP: {:#X}", self.sp);

        debug!(
            "MEM[SP+1]: {:#X}",
            interconnect.read_mem(self.sp.wrapping_add(1))
        );
        debug!("MEM[SP]: {:#X}", interconnect.read_mem(self.sp));

        debug!(
            "MEM[{:#X}]: {:#X}",
            self.sp.wrapping_sub(1),
            interconnect.read_mem(self.sp.wrapping_sub(1))
        );
        debug!(
            "MEM[{:#X}]: {:#X}",
            self.sp.wrapping_sub(2),
            interconnect.read_mem(self.sp.wrapping_sub(2))
        );

        debug!("MEM[0xDFEA]: {:#X}", interconnect.read_mem(0xDFEA));
        debug!("MEM[0xDFE9]: {:#X}", interconnect.read_mem(0xDFE9));

        let reg = format!(
            "AF: {:#X}, BC: {:#X}, DE:{:#X}, HL: {:#X}",
            self.registers.af(),
            self.registers.bc(),
            self.registers.de(),
            self.registers.hl()
        );

        debug!("{}", reg);

        debug!("IF: {:#X}", interconnect.read_mem(INTERRUPT_FLAG));
        debug!("IE: {:#X}", interconnect.read_mem(INTERRUPT_ENABLE));
        debug!("mem[FF0F]: {:#X}", interconnect.read_mem(INTERRUPT_FLAG));

        debug!(
            "DIV: {:#X} TIMA: {:#X} TMA: {:#X} TAC: {:#X}",
            interconnect.timer.div(),
            interconnect.timer.tima(),
            interconnect.timer.tma(),
            interconnect.timer.tac()
        );

        debug!("FLAG: {:#X}", self.registers.f.data);

        debug!("OPCODE: {:#X}", self.opcode);
    }
}

#[cfg(test)]
mod tests;
pub mod timer;
