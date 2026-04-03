#![allow(clippy::must_use_candidate)]
use log::warn;

use crate::cpu::{Cpu, Flags, RegisterPair};
use crate::interconnect::Interconnect;

/************************************************************************
 * 8-bit Arithmetic instructions
 * *********************************************************************/

/// Increments 8-bit register
///
/// Flags:  Z N H C
///         Z 0 H -      
pub fn inc_8bit(flags: &mut Flags, register: &mut u8) {
    let mut value: u8 = *register;

    flags.update_half_carry_flag_sum_8bit(value, 1);
    value = value.wrapping_add(1);

    flags.update_zero_flag(value);
    flags.clear_sub_flag();

    *register = value;
}

/// Decrements 8-bit register
///
/// Flags:  Z N H C
///         Z 1 H -      
pub fn dec_8bit(flags: &mut Flags, register: &mut u8) {
    let mut value = *register;

    flags.update_half_carry_flag_sub_8bit(value, 1);
    value = value.wrapping_sub(1);

    flags.update_zero_flag(value);
    flags.set_sub_flag();

    *register = value;
}

/// Increments value in memory using HL pointer
///
/// Flags:  Z N H C
///         Z 0 H -      
pub fn inc_mem(cpu: &mut Cpu, interconnect: &mut Interconnect) {
    let hl = cpu.registers.hl();
    let mut value = interconnect.read_mem(hl);

    interconnect.emu_tick(1);

    cpu.registers.f.update_half_carry_flag_sum_8bit(value, 1);
    value = value.wrapping_add(1);
    interconnect.write_mem(cpu.registers.hl(), value);

    interconnect.emu_tick(1);

    cpu.registers.f.update_zero_flag(value);
    cpu.registers.f.clear_sub_flag();
}

/// Decrements value in memory using HL pointer
///
/// Flags:  Z N H C
///         Z 1 H -
pub fn dec_mem(cpu: &mut Cpu, interconnect: &mut Interconnect) {
    let mut value: u8 = interconnect.read_mem(cpu.registers.hl());

    interconnect.emu_tick(1);

    cpu.registers.f.update_half_carry_flag_sub_8bit(value, 1);
    value = value.wrapping_sub(1);
    interconnect.write_mem(cpu.registers.hl(), value);

    interconnect.emu_tick(1);

    cpu.registers.f.update_zero_flag(value);
    cpu.registers.f.set_sub_flag();
}

/// Adds Accumulator(register A) and another register together, storing result in the accumulator
///
/// a = r + a
///
/// Flags:  Z N H C
///         Z 0 H C
pub fn add_a_r(cpu: &mut Cpu, operand: u8) {
    let mut a: u8 = cpu.registers.a;
    cpu.registers.f.update_half_carry_flag_sum_8bit(a, operand);
    cpu.registers.f.update_carry_flag_sum_8bit(a, operand);
    a = a.wrapping_add(operand);

    cpu.registers.f.clear_sub_flag();
    cpu.registers.f.update_zero_flag(a);
    cpu.registers.a = a;
}

/// Adds Accumulator(register A), another register, and carry all together, storing result in the accumulator
///
/// a = a + r + carry
///
/// Flags:  Z N H C
///         Z 0 H C
pub fn adc_a_r(cpu: &mut Cpu, operand: u8) {
    let accumulator: u8 = cpu.registers.a;
    let result: u16 =
        (accumulator as u16) + (operand as u16) + (cpu.registers.f.carry_flag() as u16);
    let half_carry: bool =
        ((accumulator & 0x0F) + (operand & 0x0F) + cpu.registers.f.carry_flag()) > 0x0F;

    cpu.registers.f.clear_sub_flag();
    if half_carry {
        cpu.registers.f.set_half_carry_flag();
    } else {
        cpu.registers.f.clear_half_carry_flag();
    }

    if result > 0xFF {
        cpu.registers.f.set_carry_flag();
    } else {
        cpu.registers.f.clear_carry_flag();
    }

    cpu.registers.f.update_zero_flag(result as u8);
    cpu.registers.a = result as u8;
}

/// Subtracts register from the accumulator, storing the result in the accumulator
///
/// a = a - r
///
/// Flags:  Z N H C
///         Z 1 H C
pub fn sub_a_r(cpu: &mut Cpu, operand: u8) {
    let mut accumulator: u8 = cpu.registers.a;

    cpu.registers
        .f
        .update_half_carry_flag_sub_8bit(accumulator, operand);
    cpu.registers
        .f
        .update_carry_flag_sub_8bit(accumulator, operand);
    accumulator = accumulator.wrapping_sub(operand);

    cpu.registers.f.set_sub_flag();
    cpu.registers.f.update_zero_flag(accumulator);
    cpu.registers.a = accumulator;
}

/// Subtracts another register and carry from the accumulator, storing the result in the accumulator
///
/// a = a - r - c
///
/// Flags:  Z N H C
///         Z 1 H C
pub fn sbc_a_r(cpu: &mut Cpu, operand: u8) {
    let accumulator = cpu.registers.a as i16;
    let b = operand as i16;
    let result: i16 = accumulator
        .wrapping_sub(b)
        .wrapping_sub(cpu.registers.f.carry_flag() as i16);

    let half_carry: bool =
        ((accumulator & 0x0F) - (b & 0x0F) - (cpu.registers.f.carry_flag() as i16)) < 0;

    cpu.registers.f.set_sub_flag();

    if half_carry {
        cpu.registers.f.set_half_carry_flag();
    } else {
        cpu.registers.f.clear_half_carry_flag();
    }

    if result < 0 {
        cpu.registers.f.set_carry_flag();
    } else {
        cpu.registers.f.clear_carry_flag();
    }

    cpu.registers.f.update_zero_flag(result as u8);
    cpu.registers.a = result as u8;
}

/// Stores the logical "and" of accumulator and register in accumluator
///
/// a = a & r
///
/// Flags:  Z N H C
///         Z 0 1 0
pub fn and_a_r(cpu: &mut Cpu, operand: u8) {
    let mut accumulator: u8 = cpu.registers.a;
    accumulator &= operand;

    cpu.registers.f.update_zero_flag(accumulator);
    cpu.registers.f.clear_sub_flag();
    cpu.registers.f.set_half_carry_flag();
    cpu.registers.f.clear_carry_flag();
    cpu.registers.a = accumulator;
}

/// Stores logical xor of accumulator and register in accumulator
///
/// a = a ^ r
///
/// Flags:  Z N H C
///         Z 0 0 0
pub fn xor_a_r(cpu: &mut Cpu, operand: u8) {
    let mut accumulator: u8 = cpu.registers.a;
    accumulator ^= operand;

    cpu.registers.f.update_zero_flag(accumulator);
    cpu.registers.f.clear_sub_flag();
    cpu.registers.f.clear_half_carry_flag();
    cpu.registers.f.clear_carry_flag();
    cpu.registers.a = accumulator;
}

/// Stores logical or of accumulator and register in accumulator
///
/// a = a | r
///
/// Flags:  Z N H C
///         Z 0 0 0
pub fn or_a_r(cpu: &mut Cpu, operand: u8) {
    let mut accumulator: u8 = cpu.registers.a;
    accumulator |= operand;

    cpu.registers.f.update_zero_flag(accumulator);
    cpu.registers.f.clear_sub_flag();
    cpu.registers.f.clear_half_carry_flag();
    cpu.registers.f.clear_carry_flag();
    cpu.registers.a = accumulator;
}

/// Compares accumulator with register via subtraction
///
/// Flags:  Z N H C
///         Z 1 H C
pub fn cp_a_r(cpu: &mut Cpu, operand: u8) {
    let accumulator: u8 = cpu.registers.a;
    let res = accumulator.wrapping_sub(operand);

    cpu.registers.f.update_zero_flag(res);
    cpu.registers.f.set_sub_flag();
    cpu.registers
        .f
        .update_half_carry_flag_sub_8bit(accumulator, operand);
    cpu.registers
        .f
        .update_carry_flag_sub_8bit(accumulator, operand);
}

/// Adjusts results of binary addition or subtratction to retroactively
/// turn it into a BCD addition or subtraction
///
/// Flags:  Z N H C
///         Z - 0 C
pub fn daa(cpu: &mut Cpu) {
    if cpu.registers.f.sub_flag() == 0 {
        if cpu.registers.f.carry_flag() == 1 || cpu.registers.a > 0x99 {
            cpu.registers.a = cpu.registers.a.wrapping_add(0x60);
            cpu.registers.f.set_carry_flag();
        }

        if cpu.registers.f.half_carry_flag() == 1 || (cpu.registers.a & 0x0F) > 0x09 {
            cpu.registers.a = cpu.registers.a.wrapping_add(0x6);
        }
    } else {
        if cpu.registers.f.carry_flag() == 1 {
            cpu.registers.a = cpu.registers.a.wrapping_sub(0x60);
        }

        if cpu.registers.f.half_carry_flag() == 1 {
            cpu.registers.a = cpu.registers.a.wrapping_sub(0x6);
        }
    }

    cpu.registers.f.clear_half_carry_flag();
    cpu.registers.f.update_zero_flag(cpu.registers.a);
}

/************************************************************************
 * 8-bit Rotate instructions
 * *********************************************************************/

/// Rotates accumulator to the left(Circular)
///
/// 7th bit of Accumulator is copied into carry
/// and into the 0th bit of A
///
/// Flags:  Z N H C
///         0 0 0 C
pub fn rlca(cpu: &mut Cpu) {
    let old_msb: u8 = (cpu.registers.a & 0x80) >> 7;

    cpu.registers.a <<= 1;
    // Store previous 7th bit in 0th position
    cpu.registers.a |= (1 << 0) & old_msb;

    if old_msb == 0 {
        cpu.registers.f.clear_carry_flag();
    } else {
        cpu.registers.f.set_carry_flag();
    }

    cpu.registers.f.clear_zero_flag();
    cpu.registers.f.clear_sub_flag();
    cpu.registers.f.clear_half_carry_flag();
}

/// Rotates accumulator to the right(Circular)
///
/// 0th Bit of Accumulator is copied into the carry and into 7th bit of Accumulator
///
/// Flags:  Z N H C
///         0 0 0 C
pub fn rrca(cpu: &mut Cpu) {
    let old_lsb: u8 = cpu.registers.a & 0x01;
    cpu.registers.a >>= 1;
    // Store previous 0th bit in 7th bit of A
    cpu.registers.a |= (1 << 7) & (old_lsb << 7);

    if old_lsb == 0 {
        cpu.registers.f.clear_carry_flag();
    } else {
        cpu.registers.f.set_carry_flag();
    }

    cpu.registers.f.clear_zero_flag();
    cpu.registers.f.clear_sub_flag();
    cpu.registers.f.clear_half_carry_flag();
}

/// Rotates accumulator to the left
///
/// 7th bit is moved into carry, and the carry is moved into the 0th bit
///
/// Flags:  Z N H C
///         0 0 0 C
pub fn rla(cpu: &mut Cpu) {
    let old_msb: u8 = cpu.registers.a & 0x80;
    cpu.registers.a <<= 1;

    // Store carry in 0th bit of A
    cpu.registers.a |= (1 << 0) & (cpu.registers.f.carry_flag());

    if old_msb == 0 {
        cpu.registers.f.clear_carry_flag();
    } else {
        cpu.registers.f.set_carry_flag();
    }

    cpu.registers.f.clear_zero_flag();
    cpu.registers.f.clear_sub_flag();
    cpu.registers.f.clear_half_carry_flag();
}

/// Rotates accumluator to the right
///
/// 0th bit of A is moved into the carry, and the carry is moved into the 7th bit of A
///
/// Flags:  Z N H C
///         0 0 0 C
pub fn rra(cpu: &mut Cpu) {
    let old_lsb: u8 = cpu.registers.a & 0x01;
    cpu.registers.a >>= 1;

    // Store carry in 7th bit of A
    cpu.registers.a |= (1 << 7) & (cpu.registers.f.carry_flag() << 7);

    if old_lsb == 0 {
        cpu.registers.f.clear_carry_flag();
    } else {
        cpu.registers.f.set_carry_flag();
    }

    cpu.registers.f.clear_zero_flag();
    cpu.registers.f.clear_sub_flag();
    cpu.registers.f.clear_half_carry_flag();
}

/// Rotates register to the left(Circular)
///
/// Contents of registert R are rotated to left 1 bit position.
///
/// Contents of 7th bit are into carry and also to 0th bit
///
/// Flags:  Z N H C
///         Z 0 0 C
pub fn rlc(f: &mut Flags, r: &mut u8) {
    let mut value: u8 = *r;

    let old_msb: u8 = (value & 0x80) >> 7;
    value <<= 1;

    // Store 7th bit into 0th bit of register
    value |= (1 << 0) & (old_msb);

    if old_msb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    f.update_zero_flag(value);
    f.clear_sub_flag();
    f.clear_half_carry_flag();
    *r = value;
}

/// Rotates (mem[HL]) to the left(Circular)
///
/// Contents of mem[HL] are rotated to left 1 bit position.
///
/// Contents of 7th bit are into carry and also to 0th bit
///
/// Flags:  Z N H C
///         Z 0 0 C
pub fn rlc_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16) {
    let mut value = interconnect.read_mem(addr);

    interconnect.emu_tick(1);

    let old_msb: u8 = (value & 0x80) >> 7;
    value <<= 1;

    // Store 7th bit in 0th bit of mem[HL]
    value |= (1 << 0) & (old_msb);

    if old_msb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    f.update_zero_flag(value);
    f.clear_sub_flag();
    f.clear_half_carry_flag();

    interconnect.write_mem(addr, value);
    interconnect.emu_tick(1);
}

/// Rotates register to the right (Circular)
///
/// Register is rotated to the right 1 bit position.
///
/// Contents of Bit 0 are copied to Carry Flag and also to bit 7.
///
/// Flags:  Z N H C
///         Z 0 0 C
pub fn rrc(f: &mut Flags, r: &mut u8) {
    let mut value: u8 = *r;
    let old_lsb: u8 = value & 0x01;
    value >>= 1;

    // Store 0th bit into 7th bit of register
    value |= (1 << 7) & (old_lsb << 7);

    if old_lsb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    f.update_zero_flag(value);
    f.clear_sub_flag();
    f.clear_half_carry_flag();
    *r = value;
}

/// Rotates (mem[HL]) to the right(Circular)
///
/// mem[HL] is rotated to the right 1 bit position.
///
/// Contents of Bit 0 are copied to Carry Flag and also to bit 7.
///
/// Flags:  Z N H C
///         Z 0 0 C
pub fn rrc_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16) {
    let mut value: u8 = interconnect.read_mem(addr);
    interconnect.emu_tick(1);

    let old_lsb: u8 = value & 0x01;
    value >>= 1;

    // Store 0th bit in 7th bit
    value |= (1 << 7) & (old_lsb << 7);

    if old_lsb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    f.update_zero_flag(value);
    f.clear_sub_flag();
    f.clear_half_carry_flag();

    interconnect.write_mem(addr, value);
    interconnect.emu_tick(1);
}

/// Rotates register to the left
///
/// Contents of operand are rotated left 1 bit position
///
/// Contents of bit 7 are copied to Carry Flag. Previous contents of Carry Flag are copied to bit 0
///
/// Flags:  Z N H C
///         Z 0 0 C
pub fn rl(f: &mut Flags, r: &mut u8) {
    let mut value: u8 = *r;

    let old_msb: u8 = (value & 0x80) >> 7;
    value <<= 1;

    // Copy carry to 0th bit
    value |= (1 << 0) & f.carry_flag();

    if old_msb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    f.update_zero_flag(value);
    f.clear_sub_flag();
    f.clear_half_carry_flag();
    *r = value;
}

/// Rotates mem[HL] to the left
///
/// Contents of mem[HL] are roateted left 1 bit position
///
/// Contents of bit 7 are copied to Carry Flag. Previous contents of Carry Flag are copied to bit 0
///
/// Flags:  Z N H C
///         Z 0 0 C
pub fn rl_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16) {
    let mut value: u8 = interconnect.read_mem(addr);
    interconnect.emu_tick(1);

    let old_msb: u8 = (value & 0x80) >> 7;
    value <<= 1;

    // Copy carry to 0th bit
    value |= (1 << 0) & f.carry_flag();

    if old_msb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    f.update_zero_flag(value);
    f.clear_sub_flag();
    f.clear_half_carry_flag();

    interconnect.write_mem(addr, value);
    interconnect.emu_tick(1);
}

/// Rotates register to the right
///
/// Contents of operand are rotated right 1 bit position through Carry Flag
///
/// Conents of bit 0 are copied to carry flag and previoius contents of carry flag
/// are copied to bit 7
///
/// Flags:  Z N H C
///         Z 0 0 C
pub fn rr(f: &mut Flags, r: &mut u8) {
    let mut value: u8 = *r;
    let old_lsb: u8 = value & 0x01;
    value >>= 1;

    // Copy carry to 0th bit
    value |= (1 << 7) & (f.carry_flag() << 7);

    if old_lsb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    f.update_zero_flag(value);
    f.clear_sub_flag();
    f.clear_half_carry_flag();
    *r = value;
}

/// Rotates mem[HL] to the right
///
/// Contents of mem[HL] are rotated right 1 bit position through Carry Flag
///
/// Conents of bit 0 are copied to carry flag and previoius contents of carry flag
/// are copied to bit 7
///
/// Flags:  Z N H C
///         Z 0 0 C
pub fn rr_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16) {
    let mut value: u8 = interconnect.read_mem(addr);
    interconnect.emu_tick(1);

    let old_lsb: u8 = value & 0x01;
    value >>= 1;

    // Copy carry to 0th bit
    value |= (1 << 7) & (f.carry_flag() << 7);

    if old_lsb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    f.update_zero_flag(value);
    f.clear_sub_flag();
    f.clear_half_carry_flag();

    interconnect.write_mem(addr, value);
    interconnect.emu_tick(1);
}

/// Shifts register to the left arithmetically
///
/// An arithmetic shift left 1 bit position is performed on contents of register
///
/// Contents of bit 7 are copied to carry flag
///
/// Flags:  Z N H C
///         Z 0 0 C
pub fn sla(f: &mut Flags, r: &mut u8) {
    let mut value: u8 = *r;
    let old_msb: u8 = (value & 0x80) >> 7;
    value <<= 1;

    if old_msb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    f.update_zero_flag(value);
    f.clear_sub_flag();
    f.clear_half_carry_flag();
    *r = value;
}

/// Shifts mem[HL] to the left arithmetically
///
/// An arithmetic shift left 1 bit position is performed on contents of mem[HL]
///
/// Contents of bit 7 are copied to carry flag
///
/// Flags:  Z N H C
///         Z 0 0 C
pub fn sla_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16) {
    let mut value: u8 = interconnect.read_mem(addr);
    interconnect.emu_tick(1);

    let old_msb: u8 = (value & 0x80) >> 7;
    value <<= 1;

    if old_msb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    f.update_zero_flag(value);
    f.clear_sub_flag();
    f.clear_half_carry_flag();

    interconnect.write_mem(addr, value);
    interconnect.emu_tick(1);
}

/// Shifts register to the right arithmetically
///    
/// Flags:  Z N H C
///         Z 0 0 C
pub fn sra(f: &mut Flags, r: &mut u8) {
    let mut value: u8 = *r;

    let old_msb: u8 = value & 0x80;
    let old_lsb: u8 = value & 0x01;
    value >>= 1;

    // put 7th bit back in
    value |= (1 << 7) & (old_msb);

    // Update Carry
    if old_lsb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    f.update_zero_flag(value);
    f.clear_sub_flag();
    f.clear_half_carry_flag();
    *r = value;
}

/// Shift mem[HL] to the right arithmetically
///
/// Flags:  Z N H C
///         Z 0 0 C
pub fn sra_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16) {
    let mut value: u8 = interconnect.read_mem(addr);
    interconnect.emu_tick(1);

    let old_msb: u8 = value & 0x80;
    let old_lsb: u8 = value & 0x01;
    value >>= 1;

    // put 7th bit back in
    value |= (1 << 7) & (old_msb);

    if old_lsb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    f.update_zero_flag(value);
    f.clear_sub_flag();
    f.clear_half_carry_flag();

    interconnect.write_mem(addr, value);
    interconnect.emu_tick(1);
}

/// Swaps r
///
/// Exchange lower and higher nibbles
///
/// Flags:  Z N H C
///         Z 0 0 0
pub fn swap(f: &mut Flags, r: &mut u8) {
    let mut value: u8 = *r;
    let lower_nibble: u8 = value & 0x0F;
    let upper_nibble: u8 = value & 0xF0;

    // Swap
    value = (((lower_nibble as u16) << 4) | ((upper_nibble as u16) >> 4)) as u8;

    f.update_zero_flag(value);
    f.clear_carry_flag();
    f.clear_sub_flag();
    f.clear_half_carry_flag();
    *r = value;
}

/// Swaps mem[HL]
///
/// Exchange lower and higher nibbles
///
/// Flags:  Z N H C
///         Z 0 0 0
pub fn swap_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16) {
    let mut value = interconnect.read_mem(addr);
    interconnect.emu_tick(1);

    let lower_nibble: u8 = value & 0x0F;
    let upper_nibble: u8 = value & 0xF0;

    // Swap
    value = (((lower_nibble as u16) << 4) | ((upper_nibble as u16) >> 4)) as u8;

    f.update_zero_flag(value);
    f.clear_carry_flag();
    f.clear_sub_flag();
    f.clear_half_carry_flag();

    interconnect.write_mem(addr, value);
    interconnect.emu_tick(1);
}

/// Shifts register to the right logically
///
/// Performs right shift on operand. 0th bit is copied to carry
///
/// 7th bit is cleared
///
/// Flags:  Z N H C
///         Z 0 0 C
pub fn srl(f: &mut Flags, r: &mut u8) {
    let mut value: u8 = *r;

    let old_lsb: u8 = value & 0x01;
    value >>= 1;

    // Clear 7th bit
    value &= !(1 << 7);

    if old_lsb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    f.update_zero_flag(value);
    f.clear_sub_flag();
    f.clear_half_carry_flag();
    *r = value;
}

///  Shifts mem[HL] to the right logically
///
/// Performs right shift on operand. 0th bit is copied to carry
///
/// 7th bit is cleared
///
/// Flags:  Z N H C
///         Z 0 0 C
pub fn srl_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16) {
    let mut value: u8 = interconnect.read_mem(addr);
    interconnect.emu_tick(1);

    let old_lsb: u8 = value & 0x01;
    value >>= 1;

    // Clear 7th bit
    value &= !(1 << 7);

    if old_lsb == 0 {
        f.clear_carry_flag();
    } else {
        f.set_carry_flag();
    }

    f.update_zero_flag(value);
    f.clear_sub_flag();
    f.clear_half_carry_flag();

    interconnect.write_mem(addr, value);
    interconnect.emu_tick(1);
}

/************************************************************************
 * 16-bit Arithmetic instructions
 * *********************************************************************/

/// Increments register pair
///
/// Flags: None
pub fn inc_16bit(cpu: &mut Cpu, register_pair: RegisterPair) {
    match register_pair {
        RegisterPair::BC => {
            cpu.registers.set_bc(cpu.registers.bc().wrapping_add(1));
        }

        RegisterPair::DE => {
            cpu.registers.set_de(cpu.registers.de().wrapping_add(1));
        }

        RegisterPair::HL => {
            cpu.registers.set_hl(cpu.registers.hl().wrapping_add(1));
        }

        RegisterPair::SP => {
            cpu.sp = cpu.sp.wrapping_add(1);
        }
        _ => warn!("{:?}, Not a register PAIR", register_pair),
    }
}

/// Decrements Register Pair
///
/// Flags: None
pub fn dec_16bit(cpu: &mut Cpu, register_pair: RegisterPair) {
    match register_pair {
        RegisterPair::BC => {
            cpu.registers.set_bc(cpu.registers.bc().wrapping_sub(1));
        }
        RegisterPair::DE => {
            cpu.registers.set_de(cpu.registers.de().wrapping_sub(1));
        }
        RegisterPair::HL => {
            cpu.registers.set_hl(cpu.registers.hl().wrapping_sub(1));
        }
        RegisterPair::SP => {
            cpu.sp = cpu.sp.wrapping_sub(1);
        }
        _ => warn!("{:?}, Not a register PAIR", register_pair),
    }
}

/// Sum of HL and register pair RR stored in HL
///
/// Flags:  Z N H C
///         - 0 H C
pub fn add_rr_hl(cpu: &mut Cpu, register_pair: RegisterPair) {
    match register_pair {
        RegisterPair::BC => {
            let a = cpu.registers.hl() as u32;
            let b = cpu.registers.bc() as u32;

            let result = a + b;

            cpu.registers.f.clear_sub_flag();
            cpu.registers.f.update_half_carry_flag_sum_16bit(a, b);
            cpu.registers
                .f
                .update_carry_flag_sum_16bit(cpu.registers.hl(), cpu.registers.bc());
            cpu.registers.set_hl(result as u16);
        }
        RegisterPair::DE => {
            let a = cpu.registers.hl() as u32;
            let b = cpu.registers.de() as u32;

            let result = a + b;

            cpu.registers.f.clear_sub_flag();
            cpu.registers.f.update_half_carry_flag_sum_16bit(a, b);
            cpu.registers
                .f
                .update_carry_flag_sum_16bit(cpu.registers.hl(), cpu.registers.de());
            cpu.registers.set_hl(result as u16);
        }
        RegisterPair::HL => {
            let a = cpu.registers.hl() as u32;
            let b = cpu.registers.hl() as u32;

            let result = a + b;

            cpu.registers.f.clear_sub_flag();
            cpu.registers.f.update_half_carry_flag_sum_16bit(a, b);
            cpu.registers
                .f
                .update_carry_flag_sum_16bit(cpu.registers.hl(), cpu.registers.hl());

            cpu.registers.set_hl(result as u16);
        }

        RegisterPair::SP => {
            let a = cpu.registers.hl() as u32;
            let b = cpu.sp as u32;

            let result = a + b;

            cpu.registers.f.clear_sub_flag();
            cpu.registers.f.update_half_carry_flag_sum_16bit(a, b);
            cpu.registers
                .f
                .update_carry_flag_sum_16bit(cpu.registers.hl(), cpu.sp);
            cpu.registers.set_hl(result as u16);
        }
        _ => warn!("{:?}, Not a register PAIR", register_pair),
    }
}

/************************************************************************
 * Jump Instructions
 * *********************************************************************/

/// Relative Jumps
///
/// PC = PC + 8bit signed
///
/// Flags: None
pub fn jr(cpu: &mut Cpu, dd: u8) {
    let offset = dd as i8;
    cpu.pc = cpu.pc.wrapping_add(offset as u16).wrapping_add(2);
}

/// Relative Jumps if Zero flag is set
///
/// Flags: None
pub fn jr_z(cpu: &mut Cpu, interconnect: &mut Interconnect, dd: u8) {
    if cpu.registers.f.zero_flag() == 1 {
        jr(cpu, dd);
        interconnect.emu_tick(3);
    } else {
        cpu.pc += 2;
        interconnect.emu_tick(2);
    }
}

/// Relative Jumps if Zero flag is clear
///
/// Flags: None
pub fn jr_nz(cpu: &mut Cpu, interconnect: &mut Interconnect, dd: u8) {
    if cpu.registers.f.zero_flag() == 0 {
        jr(cpu, dd);
        interconnect.emu_tick(3);
    } else {
        cpu.pc += 2;
        interconnect.emu_tick(2);
    }
}

/// Relative Jumps if Carry flag is Set
///
/// Flags: None
pub fn jr_c(cpu: &mut Cpu, interconnect: &mut Interconnect, dd: u8) {
    if cpu.registers.f.carry_flag() == 1 {
        jr(cpu, dd);
        interconnect.emu_tick(3);
    } else {
        cpu.pc += 2;
        interconnect.emu_tick(2);
    }
}

/// Relative Jumps if Carry flag is clear
///
/// Flags: None
pub fn jr_nc(cpu: &mut Cpu, interconnect: &mut Interconnect, dd: u8) {
    if cpu.registers.f.carry_flag() == 0 {
        jr(cpu, dd);
        interconnect.emu_tick(3);
    } else {
        cpu.pc += 2;
        interconnect.emu_tick(2);
    }
}

/// Jumps to nn
///
/// Flags: None
pub fn jp(cpu: &mut Cpu, nn: u16) {
    cpu.pc = nn;
}

/// Jumps to nn if zero flag is set
///
/// Flags: None
pub fn jp_z(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    if cpu.registers.f.zero_flag() == 1 {
        jp(cpu, nn);
        interconnect.emu_tick(4);
    } else {
        cpu.pc += 3;
        interconnect.emu_tick(3);
    }
}

/// Jumps to nn if zero flag is clear
///
/// Flags: None
pub fn jp_nz(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    if cpu.registers.f.zero_flag() == 0 {
        jp(cpu, nn);
        interconnect.emu_tick(4);
    } else {
        cpu.pc += 3;
        interconnect.emu_tick(3);
    }
}

/// Jumps to nn if carry flag is set
///
/// Flags: None
pub fn jp_c(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    if cpu.registers.f.carry_flag() == 1 {
        jp(cpu, nn);
        interconnect.emu_tick(4);
    } else {
        cpu.pc += 3;
        interconnect.emu_tick(3);
    }
}

/// Jumps to nn if carry flag is clear
///
/// Flags: None
pub fn jp_nc(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    if cpu.registers.f.carry_flag() == 0 {
        jp(cpu, nn);
        interconnect.emu_tick(4);
    } else {
        cpu.pc += 3;
        interconnect.emu_tick(3);
    }
}

/// Calls to nn
///
/// Flags: None
pub fn call(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    let mut stack_pointer: u16 = cpu.sp;
    stack_pointer -= 2;
    cpu.pc += 3;

    interconnect.write_mem(stack_pointer, (cpu.pc & 0x00FF) as u8);
    interconnect.write_mem(stack_pointer + 1, ((cpu.pc & 0xFF00) >> 8) as u8);

    cpu.pc = nn;
    cpu.sp = stack_pointer;
}

/// Calls to nn if zero flag is set
///
/// Flags: None
pub fn call_z(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    if cpu.registers.f.zero_flag() == 1 {
        call(cpu, interconnect, nn);
        interconnect.emu_tick(6);
    } else {
        cpu.pc += 3;
        interconnect.emu_tick(3);
    }
}

/// Calls to nn if zero flag is clear
///
/// Flags: None
pub fn call_nz(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    if cpu.registers.f.zero_flag() == 0 {
        call(cpu, interconnect, nn);
        interconnect.emu_tick(6);
    } else {
        cpu.pc += 3;
        interconnect.emu_tick(3);
    }
}

/// Calls to nn if carry flag is set
///
/// Flags: None
pub fn call_c(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    if cpu.registers.f.carry_flag() == 1 {
        call(cpu, interconnect, nn);
        interconnect.emu_tick(6);
    } else {
        cpu.pc += 3;
        interconnect.emu_tick(3);
    }
}

/// Calls to nn if carry flag is clear
///
/// Flags: None
pub fn call_nc(cpu: &mut Cpu, interconnect: &mut Interconnect, nn: u16) {
    if cpu.registers.f.carry_flag() == 0 {
        call(cpu, interconnect, nn);
        interconnect.emu_tick(6);
    } else {
        cpu.pc += 3;
        interconnect.emu_tick(3);
    }
}

/// Calls to 00, 08, 10, 18, 20, 28, 30, 38(hex)
///
/// Flags: None
pub fn rst(cpu: &mut Cpu, interconnect: &mut Interconnect, n: u8) {
    let mut stack_pointer: u16 = cpu.sp;

    stack_pointer = stack_pointer.wrapping_sub(2);
    cpu.pc += 1;

    // mem[SP] = lower byte of program counter
    interconnect.write_mem(stack_pointer, (cpu.pc & 0x00FF) as u8);

    // mem[SP+1] = upper byte of program counter (its + 1 below because we already moved the stack pointer)
    interconnect.write_mem(stack_pointer + 1, ((cpu.pc & 0xFF00) >> 8) as u8);

    cpu.pc = n as u16;
    cpu.sp = stack_pointer;
}

/// Returns
///
/// Flags: None
pub fn ret(cpu: &mut Cpu, interconnect: &Interconnect) {
    let mut sp = cpu.sp;

    // PC = (SP)
    let pc = u16::from_be_bytes([
        interconnect.read_mem(sp.wrapping_add(1)),
        interconnect.read_mem(sp),
    ]);

    cpu.pc = pc;
    sp = sp.wrapping_add(2);
    cpu.sp = sp;
}

/// Returns if zero flag is set
///
/// Flags: None
pub fn ret_z(cpu: &mut Cpu, interconnect: &mut Interconnect) {
    if cpu.registers.f.zero_flag() == 1 {
        ret(cpu, interconnect);
        interconnect.emu_tick(5);
    } else {
        cpu.pc += 1;
        interconnect.emu_tick(2);
    }
}

/// Returns if zero flag is clear
///
/// Flags: None
pub fn ret_nz(cpu: &mut Cpu, interconnect: &mut Interconnect) {
    if cpu.registers.f.zero_flag() == 0 {
        ret(cpu, interconnect);
        interconnect.emu_tick(5);
    } else {
        cpu.pc += 1;
        interconnect.emu_tick(2);
    }
}

/// Returns if carry flag is set
///
/// Flags: None
pub fn ret_c(cpu: &mut Cpu, interconnect: &mut Interconnect) {
    if cpu.registers.f.carry_flag() == 1 {
        ret(cpu, interconnect);
        interconnect.emu_tick(5);
    } else {
        cpu.pc += 1;
        interconnect.emu_tick(2);
    }
}

/// Returns if carry flag is  clear
///
/// Flags: None
pub fn ret_nc(cpu: &mut Cpu, interconnect: &mut Interconnect) {
    if cpu.registers.f.carry_flag() == 0 {
        ret(cpu, interconnect);
        interconnect.emu_tick(5);
    } else {
        cpu.pc += 1;

        interconnect.emu_tick(2);
    }
}

/************************************************************************
 * 8-bit LOAD instructions
 * *********************************************************************/

/// Loads 8 bit value into specific register
///
/// Flags: None
pub fn ld_8bit(r: &mut u8, data: u8) {
    *r = data;
}

/// Loads data from io-port 'n' into A register
///
/// Flags: None
pub fn ld_a_from_io(cpu: &mut Cpu, interconnect: &Interconnect, n: u8) {
    let addr: u16 = 0xFF00 + (n as u16);
    cpu.registers.a = interconnect.read_mem(addr);
}

/// Loads data from A register into io-port 'n'
///
/// Flags: None
pub fn ld_io_from_a(cpu: &Cpu, interconnect: &mut Interconnect, n: u8) {
    let addr: u16 = 0xFF00 + (n as u16);
    interconnect.write_mem(addr, cpu.registers.a);
}

/// Loads data from [$FF00 + register C] into A register
///
/// Flags: None
pub fn ld_a_from_io_c(cpu: &mut Cpu, interconnect: &Interconnect) {
    let addr: u16 = 0xFF00 + (cpu.registers.c as u16);
    cpu.registers.a = interconnect.read_mem(addr);
}

/// Loads data from register A into mem[$FF00 + register C]
///
/// Flags: None
pub fn ld_io_c_from_a(cpu: &Cpu, interconnect: &mut Interconnect) {
    let addr: u16 = 0xFF00 + (cpu.registers.c as u16);
    interconnect.write_mem(addr, cpu.registers.a);
}

/************************************************************************
 * 16-bit LOAD instructions
 * *********************************************************************/

/// Contents of Register Pair are popped off stack
///
/// Flags: None
///
/// Unless it is POP AF then
///
/// Flags:  Z N H C
///         Z N H C
pub fn pop_rr(interconnect: &Interconnect, upper: &mut u8, lower: &mut u8, sp: &mut u16) {
    let mut stack_pointer = *sp;

    // Value in memory (mem[sp])
    let lower_byte: u8 = interconnect.read_mem(stack_pointer);
    let upper_byte: u8 = interconnect.read_mem(stack_pointer.wrapping_add(1));

    // rr = mem[sp]
    *lower = lower_byte;
    *upper = upper_byte;

    stack_pointer = stack_pointer.wrapping_add(2);
    *sp = stack_pointer;
}

/// Contents of Register Pair are pushed onto stack
///
/// Flags: None
pub fn push_rr(interconnect: &mut Interconnect, upper: u8, lower: u8, sp: &mut u16) {
    let mut stack_pointer = *sp;
    stack_pointer = stack_pointer.wrapping_sub(2);

    // mem[sp] = rr
    interconnect.write_mem(stack_pointer, lower);
    interconnect.write_mem(stack_pointer + 1, upper);

    *sp = stack_pointer;
}

/************************************************************************
 * Single-bit Operation instructions
 * *********************************************************************/

/// Checks the nth bit of r and stores the inverse in the zero flag.
///
/// Flags:  Z N H C
///         Z 0 1 -
pub fn bit_n_r(f: &mut Flags, r: &mut u8, n: u8) {
    let value: u8 = *r;
    let nth_bit = (value >> n) & 0x01;

    if nth_bit == 0 {
        f.set_zero_flag();
    } else {
        f.clear_zero_flag();
    }
    f.clear_sub_flag();
    f.set_half_carry_flag();
}

/// Checks the nth bit of mem[hl] and stores the inverse in the zero flag.
///
/// Flags:  Z N H C
///         Z 0 1 -
pub fn bit_n_hl(f: &mut Flags, interconnect: &mut Interconnect, addr: u16, n: u8) {
    let value: u8 = interconnect.read_mem(addr);

    let nth_bit = (value >> n) & 0x01;

    // Update Zero Flag
    if nth_bit == 0 {
        f.set_zero_flag();
    } else {
        f.clear_zero_flag();
    }

    f.clear_sub_flag();
    f.set_half_carry_flag();
}

/// Sets the nth bit of r.
///
/// Flags: None
pub fn set_n_r(r: &mut u8, n: u8) {
    let mut value: u8 = *r;

    // Set the nth bit
    value |= 1 << n;
    *r = value;
}

/// Sets the nth bit of mem[HL].
///
/// Flags: None
pub fn set_n_hl(interconnect: &mut Interconnect, addr: u16, n: u8) {
    let mut value: u8 = interconnect.read_mem(addr);
    interconnect.emu_tick(1);

    // Set the nth bit
    value |= 1 << n;

    interconnect.write_mem(addr, value);
    interconnect.emu_tick(1);
}

/// Clears the nth bit of r
///
/// Flags: None
pub fn res_n_r(r: &mut u8, n: u8) {
    let mut value: u8 = *r;

    // Clear the nth bit
    value &= !(1 << n);

    *r = value;
}

/// Clears the nth bit of mem[HL]
///
/// Flags: None
pub fn res_n_hl(interconnect: &mut Interconnect, addr: u16, n: u8) {
    let mut value: u8 = interconnect.read_mem(addr);
    interconnect.emu_tick(1);

    // Clear the nth bit
    value &= !(1 << n);

    interconnect.write_mem(addr, value);
    interconnect.emu_tick(1);
}

/************************************************************************
 * Interrupt Instructions
 * *********************************************************************/

// Enable Interrupt
pub fn ei(cpu: &mut Cpu) {
    cpu.ime_to_be_enabled = true;
}

// Disable Interrupt
pub fn di(cpu: &mut Cpu) {
    cpu.ime = false;
}
