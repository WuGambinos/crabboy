use crate::cpu::instructions::*;
use crate::cpu::{Cpu, RegisterPair};
use crate::interconnect::Interconnect;

#[test]
fn internal() {
    assert_eq!(4, 4);
}

#[test]
fn ld_bc_u16() {
    let mut cpu = Cpu::new();

    let mut interconnect = Interconnect::new();
    cpu.pc = 0xFF80;

    interconnect.write_mem(cpu.pc, 0x01);
    interconnect.write_mem(cpu.pc + 1, 0xFA);
    interconnect.write_mem(cpu.pc + 2, 0xDC);

    //FADC

    cpu.execute_instruction(&mut interconnect);

    assert_eq!(cpu.registers.bc(), 0xDCFA);
}

/*************************************************************************
 * 8-bit Arithmetic Tests
 *************************************************************************/

#[test]
fn inc_b() {
    let mut cpu = Cpu::new();
    cpu.registers.f.clear_all();

    cpu.registers.b = 0x01;

    inc_8bit(&mut cpu.registers.f, &mut cpu.registers.b);

    //Check resulting register value and flags
    let check = vec![cpu.registers.b, cpu.registers.f.data];

    assert_eq!(check, [0x02, 0x00]);
}

#[test]
fn inc_c() {
    let mut cpu = Cpu::new();
    cpu.registers.f.clear_all();

    cpu.registers.c = 0x01;

    inc_8bit(&mut cpu.registers.f, &mut cpu.registers.c);

    //Check resulting register value and flags
    let check = vec![cpu.registers.c, cpu.registers.f.data];

    assert_eq!(check, [0x02, 0x00]);
}

#[test]
fn inc_d() {
    let mut cpu = Cpu::new();
    cpu.registers.f.clear_all();

    cpu.registers.d = 0x05;
    inc_8bit(&mut cpu.registers.f, &mut cpu.registers.d);

    //Check resulting register value and flags
    let check = vec![cpu.registers.d, cpu.registers.f.data];

    assert_eq!(check, [0x06, 0x00]);
}

#[test]
fn inc_e() {
    let mut cpu = Cpu::new();
    cpu.registers.f.clear_all();

    cpu.registers.e = 0x05;
    inc_8bit(&mut cpu.registers.f, &mut cpu.registers.e);

    //Check resulting register value and flags
    let check = vec![cpu.registers.e, cpu.registers.f.data];

    assert_eq!(check, [0x06, 0x00]);
}

#[test]
fn inc_h() {
    let mut cpu = Cpu::new();
    cpu.registers.f.clear_all();

    cpu.registers.h = 0x05;
    inc_8bit(&mut cpu.registers.f, &mut cpu.registers.h);

    //Check resulting register value and flags
    let check = vec![cpu.registers.h, cpu.registers.f.data];

    assert_eq!(check, [0x06, 0x00]);
}

#[test]
fn inc_l() {
    let mut cpu = Cpu::new();
    cpu.registers.f.clear_all();

    cpu.registers.l = 0x05;
    inc_8bit(&mut cpu.registers.f, &mut cpu.registers.l);

    //Check resulting register value and flags
    let check = vec![cpu.registers.l, cpu.registers.f.data];

    assert_eq!(check, [0x06, 0x00]);
}

#[test]
fn inc_8bit_overflow() {
    let mut cpu = Cpu::new();
    cpu.registers.f.clear_all();

    cpu.registers.b = 0xFF;
    inc_8bit(&mut cpu.registers.f, &mut cpu.registers.b);

    //Check resulting register value and flags
    let check = vec![cpu.registers.b, cpu.registers.f.data];

    assert_eq!(check, [0x00, 0xA0]);
}

#[test]
fn dec_b() {
    let mut cpu = Cpu::new();
    cpu.registers.f.clear_all();

    cpu.registers.b = 0x02;
    dec_8bit(&mut cpu.registers.f, &mut cpu.registers.b);

    //Check resulting register value and flags
    let check = vec![cpu.registers.b, cpu.registers.f.data];

    assert_eq!(check, [0x01, 0x40]);
}

#[test]
fn dec_c() {
    let mut cpu = Cpu::new();
    cpu.registers.f.clear_all();

    cpu.registers.c = 0x05;

    dec_8bit(&mut cpu.registers.f, &mut cpu.registers.c);

    //Check resulting register value and flags
    let check = vec![cpu.registers.c, cpu.registers.f.data];

    //0x04 0x40
    assert_eq!(check, [0x04, 0x40]);
}

#[test]
fn dec_d() {
    let mut cpu = Cpu::new();
    cpu.registers.f.clear_all();

    cpu.registers.d = 0x03;
    dec_8bit(&mut cpu.registers.f, &mut cpu.registers.d);

    //Check resulting register value and flags
    let check = vec![cpu.registers.d, cpu.registers.f.data];

    assert_eq!(check, [0x02, 0x40]);
}

#[test]
fn dec_e() {
    let mut cpu = Cpu::new();
    cpu.registers.f.clear_all();

    cpu.registers.e = 0x01;
    dec_8bit(&mut cpu.registers.f, &mut cpu.registers.e);

    //Check resulting register value and flags
    let check = vec![cpu.registers.e, cpu.registers.f.data];

    assert_eq!(check, [0x00, 0xC0]);
}

#[test]
fn dec_h() {
    let mut cpu = Cpu::new();
    cpu.registers.f.clear_all();

    cpu.registers.h = 0x00;
    dec_8bit(&mut cpu.registers.f, &mut cpu.registers.h);

    //Check resulting register value and flags
    let check = vec![cpu.registers.h, cpu.registers.f.data];

    assert_eq!(check, [0xFF, 0x60]);
}

#[test]
fn dec_l() {
    let mut cpu = Cpu::new();
    cpu.registers.f.clear_all();

    cpu.registers.l = 0x05;
    dec_8bit(&mut cpu.registers.f, &mut cpu.registers.l);

    //Check resulting register value and flags
    let check = vec![cpu.registers.l, cpu.registers.f.data];

    assert_eq!(check, [0x04, 0x40]);
}

///Basic test for ADD r r instruction
#[test]
fn add_r() {
    let mut cpu = Cpu::new();

    cpu.registers.a = 0xFF;
    cpu.registers.b = 0x05;
    let b = cpu.registers.b;

    add_a_r(&mut cpu, b);

    //Check resulting register value and flags
    let check = vec![cpu.registers.a, cpu.registers.f.data];

    assert_eq!(check, [0x04, 0x30]);
}

//Test For Overflow with ADD r r instruction
#[test]
fn add_r_overflow() {
    let mut cpu = Cpu::new();

    cpu.registers.a = 0xFF;
    cpu.registers.b = 0x80;

    let b: u8 = cpu.registers.b;

    add_a_r(&mut cpu, b);

    //Check resulting register value and flags
    let check = vec![cpu.registers.a, cpu.registers.f.data];

    assert_eq!(check, [127, 0x10]);
}

///Basic test for ADC r r instruction
#[test]
fn adc_r() {
    let mut cpu = Cpu::new();

    cpu.registers.a = 0x02;
    cpu.registers.b = 0x25;
    cpu.registers.f.set_carry_flag();

    let b: u8 = cpu.registers.b;

    adc_a_r(&mut cpu, b);

    //Check resulting register value and flags
    let check = vec![cpu.registers.a, cpu.registers.f.data];

    assert_eq!(check, [0x28, 0x00]);
}

///Basic test for SUB r r instruction
#[test]
fn sub_r() {
    let mut cpu = Cpu::new();

    cpu.registers.a = 0x05;
    cpu.registers.b = 0x03;

    let b: u8 = cpu.registers.b;

    sub_a_r(&mut cpu, b);

    //Check resulting register value and flags
    let check = vec![cpu.registers.a, cpu.registers.f.data];

    assert_eq!(check, [0x02, 0x40]);
}

///Basic test for SBC r r instruction
#[test]
fn sbc_r() {
    let mut cpu = Cpu::new();

    cpu.registers.a = 0x08;
    cpu.registers.b = 0x02;
    cpu.registers.f.set_carry_flag();

    let b: u8 = cpu.registers.b;

    sbc_a_r(&mut cpu, b);

    //Check resulting register value and flags
    let check = vec![cpu.registers.a, cpu.registers.f.data];

    assert_eq!(check, [0x05, 0x40]);
}

//Testing for correct borrow detection
#[test]
fn sub_r_borrow() {
    let mut cpu = Cpu::new();

    cpu.registers.a = 0x07;
    cpu.registers.b = 0x10;

    let b: u8 = cpu.registers.b;

    sub_a_r(&mut cpu, b);

    //Check resulting register value and flags
    let check = vec![cpu.registers.a, cpu.registers.f.data];

    assert_eq!(check, [0xF7, 0x50])
}

///Testing for correct result when borrow(carry) is set
#[test]
fn sbc_r_borrow_set() {
    let mut cpu = Cpu::new();

    cpu.registers.a = 0x08;
    cpu.registers.b = 0x03;
    cpu.registers.f.set_carry_flag();

    let b: u8 = cpu.registers.b;

    sbc_a_r(&mut cpu, b);

    //Check resulting register value and flags
    let check = vec![cpu.registers.a, cpu.registers.f.data];

    assert_eq!(check, [0x04, 0x40]);
}

#[test]
fn and_a_hl_test() {
    let mut cpu = Cpu::new();

    let mut interconnect = Interconnect::new();

    cpu.registers.a = 0xFF;

    let addr = cpu.registers.hl();

    interconnect.write_mem(addr, 0xA0);

    and_a_r(&mut cpu, interconnect.read_mem(addr));

    assert_eq!(cpu.registers.a, 0xA0);
}

/*************************************************************************
 * 16-bit Arithmetic Tests
 *************************************************************************/

#[test]
fn inc_bc() {
    let mut cpu = Cpu::new();
    cpu.registers.set_bc(0x00FF);
    inc_16bit(&mut cpu, RegisterPair::BC);
    assert_eq!(cpu.registers.bc(), 256);
}

#[test]
fn inc_de() {
    let mut cpu = Cpu::new();
    cpu.registers.set_de(0xFFFF);
    inc_16bit(&mut cpu, RegisterPair::DE);
    assert_eq!(cpu.registers.de(), 0);
}

#[test]
fn inc_hl() {
    let mut cpu = Cpu::new();
    cpu.registers.set_hl(0x0008);
    inc_16bit(&mut cpu, RegisterPair::HL);
    assert_eq!(cpu.registers.hl(), 0x09);
}

#[test]
fn add_bc_hl() {
    let mut cpu = Cpu::new();

    cpu.registers.set_hl(0xF0);

    cpu.registers.set_bc(0xFF);

    add_rr_hl(&mut cpu, RegisterPair::BC);

    assert_eq!(cpu.registers.hl(), 0x01EF);
}

#[test]
fn add_de_hl() {
    let mut cpu = Cpu::new();

    cpu.registers.set_hl(0x0002);
    cpu.registers.set_de(0x0005);

    add_rr_hl(&mut cpu, RegisterPair::DE);
    assert_eq!(cpu.registers.hl(), 0x0007);
}

#[test]
fn add_hl_hl() {
    let mut cpu = Cpu::new();

    cpu.registers.set_hl(0xFF);

    add_rr_hl(&mut cpu, RegisterPair::HL);
    assert_eq!(cpu.registers.hl(), 0x01FE);
}

#[test]
fn half_carry() {
    let mut cpu = Cpu::new();
    cpu.registers.b = 0x09;

    let operand: u8 = 0x0A;

    cpu.registers
        .f
        .update_half_carry_flag_sum_8bit(cpu.registers.b, operand);

    assert_eq!(cpu.registers.f.half_carry_flag(), 1);
}

#[test]
fn carry() {
    let mut cpu = Cpu::new();

    cpu.registers.set_bc(0xFFFF);
    cpu.registers.set_hl(0x0001);

    cpu.registers
        .f
        .update_carry_flag_sum_16bit(cpu.registers.bc(), cpu.registers.hl());

    //assert_eq!(cpu.f.carry_flag, 1);
    assert_eq!(cpu.registers.bc(), 0xFFFF);
    assert_eq!(cpu.registers.hl(), 0x0001);
}

/*************************************************************************
 * 8-bit Load Tests
 *************************************************************************/

#[test]
fn load_8bit_test() {
    let mut cpu = Cpu::new();

    cpu.registers.c = 5;
    ld_8bit(&mut (cpu.registers.b), cpu.registers.c);
}

/*************************************************************************
 * 16-bit Load Tests
 *************************************************************************/
#[test]
fn pop_rr_test() {
    let mut cpu = Cpu::new();
    let mut mmu = Interconnect::new();

    cpu.sp = 0x1000;

    mmu.write_mem(cpu.sp, 0x55);
    mmu.write_mem(cpu.sp + 1, 0x33);

    pop_rr(
        &mmu,
        &mut cpu.registers.b,
        &mut cpu.registers.c,
        &mut cpu.sp,
    );

    assert_eq!(cpu.registers.bc(), 0x3355);
}

#[test]
fn push_rr_test() {
    let mut cpu = Cpu::new();
    let mut mmu = Interconnect::new();

    cpu.registers.b = 0x22;
    cpu.registers.c = 0x33;

    cpu.sp = 0x1007;

    push_rr(&mut mmu, cpu.registers.b, cpu.registers.c, &mut cpu.sp);

    let check: Vec<u16> = vec![0x0022, 0x0033, 0x1005];

    assert_eq!(
        check,
        [
            mmu.read_mem(0x1006) as u16,
            mmu.read_mem(0x1005) as u16,
            cpu.sp
        ]
    );
}

#[test]
fn push_af_test() {
    let mut cpu = Cpu::new();
    let mut mmu = Interconnect::new();

    cpu.registers.a = 0x22;
    cpu.registers.f.data = 0x33;

    cpu.sp = 0x1007;

    push_rr(&mut mmu, cpu.registers.a, cpu.registers.f.data, &mut cpu.sp);

    let check: Vec<u16> = vec![0x0022, 0x0033, 0x1005];

    assert_eq!(
        check,
        [
            mmu.read_mem(0x1006) as u16,
            mmu.read_mem(0x1005) as u16,
            cpu.sp
        ]
    )
}

/*************************************************************************
 * Jump Instructions Tests
 *************************************************************************/
#[test]
fn jp_test() {
    let mut cpu: Cpu = Cpu::new();

    let nn: u16 = 0xFF00;

    jp(&mut cpu, nn);

    assert_eq!(cpu.pc, nn);
}

#[test]
fn jp_z_test() {
    let mut cpu: Cpu = Cpu::new();
    let mut interconnect: Interconnect = Interconnect::new();

    let nn: u16 = 0xFF00;

    cpu.registers.f.set_zero_flag();

    jp_z(&mut cpu, &mut interconnect, nn);

    assert_eq!(cpu.pc, nn);
}

#[test]
fn jp_nz_test() {
    let mut cpu: Cpu = Cpu::new();
    let mut interconnect: Interconnect = Interconnect::new();

    let nn: u16 = 0xFF00;

    cpu.registers.f.clear_zero_flag();

    jp_nz(&mut cpu, &mut interconnect, nn);

    assert_eq!(cpu.pc, nn);
}

#[test]
fn jp_c_test() {
    let mut cpu: Cpu = Cpu::new();
    let mut interconnect: Interconnect = Interconnect::new();
    let nn: u16 = 0xFF00;

    cpu.registers.f.set_carry_flag();

    jp_c(&mut cpu, &mut interconnect, nn);

    assert_eq!(cpu.pc, nn);
}

#[test]
fn jp_nc_test() {
    let mut cpu: Cpu = Cpu::new();

    let mut interconnect: Interconnect = Interconnect::new();
    let nn: u16 = 0xFF00;

    cpu.registers.f.clear_carry_flag();

    jp_nc(&mut cpu, &mut interconnect, nn);

    assert_eq!(cpu.pc, nn);
}

#[test]
fn jp_hl_test() {
    let mut cpu: Cpu = Cpu::new();

    cpu.registers.set_hl(0xAB00);

    let nn: u16 = cpu.registers.hl();

    jp(&mut cpu, nn);

    assert_eq!(cpu.pc, cpu.registers.hl());
}

#[test]
fn call_test() {
    let mut cpu: Cpu = Cpu::new();
    let mut mmu: Interconnect = Interconnect::new();

    cpu.pc = 0x1A47;
    cpu.sp = 0x3002;

    mmu.write_mem(cpu.pc, 0xCD);
    mmu.write_mem(cpu.pc + 1, 0x35);
    mmu.write_mem(cpu.pc + 2, 0x21);

    let nn: u16 = u16::from_be_bytes([mmu.read_mem(cpu.pc + 2), mmu.read_mem(cpu.pc + 1)]);
    call(&mut cpu, &mut mmu, nn);

    let check: Vec<u16> = vec![0x001A, 0x004A, 0x3000, 0x2135];

    assert_eq!(
        check,
        [
            mmu.read_mem(0x3001) as u16,
            mmu.read_mem(0x3000) as u16,
            cpu.sp,
            cpu.pc
        ]
    );
}

#[test]
fn call_z_test() {
    let mut cpu: Cpu = Cpu::new();
    let mut mmu: Interconnect = Interconnect::new();

    cpu.pc = 0x1A47;
    cpu.sp = 0x3002;

    cpu.registers.f.set_zero_flag();

    mmu.write_mem(cpu.pc, 0xCD);
    mmu.write_mem(cpu.pc + 1, 0x35);
    mmu.write_mem(cpu.pc + 2, 0x21);

    let nn: u16 = u16::from_be_bytes([mmu.read_mem(cpu.pc + 2), mmu.read_mem(cpu.pc + 1)]);
    call_z(&mut cpu, &mut mmu, nn);

    let check: Vec<u16> = vec![0x001A, 0x004A, 0x3000, 0x2135];

    assert_eq!(
        check,
        [
            mmu.read_mem(0x3001) as u16,
            mmu.read_mem(0x3000) as u16,
            cpu.sp,
            cpu.pc
        ]
    );
}

#[test]
fn call_nz_test() {
    let mut cpu: Cpu = Cpu::new();
    let mut mmu: Interconnect = Interconnect::new();

    cpu.pc = 0x1A47;
    cpu.sp = 0x3002;

    cpu.registers.f.clear_zero_flag();

    mmu.write_mem(cpu.pc, 0xCD);
    mmu.write_mem(cpu.pc + 1, 0x35);
    mmu.write_mem(cpu.pc + 2, 0x21);

    let nn: u16 = u16::from_be_bytes([mmu.read_mem(cpu.pc + 2), mmu.read_mem(cpu.pc + 1)]);
    call_nz(&mut cpu, &mut mmu, nn);

    let check: Vec<u16> = vec![0x001A, 0x004A, 0x3000, 0x2135];

    assert_eq!(
        check,
        [
            mmu.read_mem(0x3001) as u16,
            mmu.read_mem(0x3000) as u16,
            cpu.sp,
            cpu.pc
        ]
    );
}
#[test]
fn ret_test() {
    let mut cpu = Cpu::new();
    let mut mmu = Interconnect::new();

    cpu.pc = 0x3535;
    cpu.sp = 0x2000;
    mmu.write_mem(cpu.sp, 0xB5);
    mmu.write_mem(cpu.sp + 1, 0x18);

    ret(&mut cpu, &mmu);

    let check: Vec<u16> = vec![0x2002, 0x18B5];

    assert_eq!(check, [cpu.sp, cpu.pc]);
}

#[test]
fn ret_z_test() {
    let mut cpu = Cpu::new();
    let mut interconnect = Interconnect::new();

    cpu.registers.f.set_zero_flag();

    cpu.pc = 0x3535;
    cpu.sp = 0x2000;
    interconnect.write_mem(cpu.sp, 0xB5);
    interconnect.write_mem(cpu.sp + 1, 0x18);

    ret_z(&mut cpu, &mut interconnect);

    let check: Vec<u16> = vec![0x2002, 0x18B5];

    assert_eq!(check, [cpu.sp, cpu.pc]);
}

#[test]
fn ret_nz_test() {
    let mut cpu = Cpu::new();
    let mut interconnect = Interconnect::new();

    cpu.registers.f.clear_zero_flag();

    cpu.pc = 0x3535;
    cpu.sp = 0x2000;
    interconnect.write_mem(cpu.sp, 0xB5);
    interconnect.write_mem(cpu.sp + 1, 0x18);

    ret_nz(&mut cpu, &mut interconnect);

    let check: Vec<u16> = vec![0x2002, 0x18B5];

    assert_eq!(check, [cpu.sp, cpu.pc]);
}

/*************************************************************************
 * Flags Tests
 *************************************************************************/
#[test]
fn set_flags() {
    let mut cpu = Cpu::new();

    cpu.registers.f.set_zero_flag();
    cpu.registers.f.set_sub_flag();
    cpu.registers.f.set_half_carry_flag();
    cpu.registers.f.set_carry_flag();

    assert_eq!(cpu.registers.f.data, 0b11110000);
}

#[test]
fn clear_flags() {
    let mut cpu = Cpu::new();

    cpu.registers.f.set_zero_flag();
    cpu.registers.f.set_sub_flag();
    cpu.registers.f.set_half_carry_flag();
    cpu.registers.f.set_carry_flag();

    cpu.registers.f.clear_zero_flag();
    cpu.registers.f.clear_sub_flag();
    cpu.registers.f.clear_half_carry_flag();
    cpu.registers.f.clear_carry_flag();

    assert_eq!(cpu.registers.f.data, 0b00000000);
}

/*************************************************************************
 * Rotate and Shift Tests
 *************************************************************************/

#[test]
fn rlca_test() {
    let mut cpu = Cpu::new();

    cpu.registers.a = 0x88;

    rlca(&mut cpu);

    assert_eq!(cpu.registers.a, 0x11);
}

#[test]
fn rla_test() {
    let mut cpu = Cpu::new();

    cpu.registers.f.set_carry_flag();
    cpu.registers.a = 0x76;
    rla(&mut cpu);

    assert_eq!(cpu.registers.a, 0b11101101);
}
#[test]
fn rlc_r_test() {
    let mut cpu = Cpu::new();

    //136
    cpu.registers.b = 0x88;

    rlc(&mut cpu.registers.f, &mut cpu.registers.b);

    assert_eq!(cpu.registers.f.carry_flag(), 1);
    assert_eq!(cpu.registers.b, 0x11);
}

#[test]
fn rlc_hl_test() {
    let mut cpu = Cpu::new();
    let mut mmu = Interconnect::new();

    let addr = 0xFFF;

    mmu.write_mem(0xFFF, 0x88);

    rlc_hl(&mut cpu.registers.f, &mut mmu, addr);

    let check = vec![mmu.read_mem(0xFFF), 1];
    assert_eq!(check, [0x11, 1]);
}

#[test]
fn rrca_test() {
    let mut cpu = Cpu::new();

    cpu.registers.a = 0x11;

    rrca(&mut cpu);

    assert_eq!(cpu.registers.a, 0b10001000);
}

#[test]
fn rra_test() {
    let mut cpu = Cpu::new();

    cpu.registers.a = 0xE1;

    cpu.registers.f.set_carry_flag();

    rra(&mut cpu);

    assert_eq!(cpu.registers.a, 0xF0);
    assert_eq!(cpu.registers.f.carry_flag(), 0x01);
}

#[test]
fn rrc_test() {
    let mut cpu = Cpu::new();

    cpu.registers.b = 0x31;

    rrc(&mut cpu.registers.f, &mut cpu.registers.b);

    assert_eq!(cpu.registers.b, 0x98);
}

#[test]
fn rrc_hl_test() {
    let mut cpu = Cpu::new();

    let mut mmu = Interconnect::new();

    let addr: u16 = 0xFFF;
    let value: u8 = 0x31;
    mmu.write_mem(addr, value);

    rrc_hl(&mut cpu.registers.f, &mut mmu, addr);

    assert_eq!(mmu.read_mem(addr), 0x98);
}

#[test]
fn rl_test() {
    let mut cpu = Cpu::new();
    cpu.registers.f.clear_all();

    cpu.registers.d = 0x8F;

    rl(&mut cpu.registers.f, &mut cpu.registers.d);

    let check = vec![cpu.registers.d, cpu.registers.f.carry_flag()];

    assert_eq!(check, [0x1E, 0x01]);
}

#[test]
fn rl_hl_test() {
    let mut cpu = Cpu::new();
    cpu.registers.f.clear_all();
    let mut mmu = Interconnect::new();

    let addr = 0xFFF;
    let value = 0x8F;

    mmu.write_mem(addr, value);

    rl_hl(&mut cpu.registers.f, &mut mmu, addr);

    let check = vec![mmu.read_mem(addr), cpu.registers.f.carry_flag()];

    assert_eq!(check, [0x1E, 0x01]);
}

#[test]
fn rr_test() {
    let mut cpu = Cpu::new();
    cpu.registers.f.clear_all();

    cpu.registers.b = 0xDD;

    rr(&mut cpu.registers.f, &mut cpu.registers.b);

    let check = vec![cpu.registers.b, cpu.registers.f.carry_flag()];

    assert_eq!(check, [0x6E, 0x01]);
}

#[test]
fn rr_hl_test() {
    let mut cpu = Cpu::new();
    cpu.registers.f.clear_all();
    let mut mmu = Interconnect::new();

    let addr = 0xFFF;
    let value = 0xDD;

    mmu.write_mem(addr, value);

    rr_hl(&mut cpu.registers.f, &mut mmu, addr);

    let check = vec![mmu.read_mem(addr), cpu.registers.f.carry_flag()];

    assert_eq!(check, [0x6E, 0x01]);
}

#[test]
fn sla_test() {
    let mut cpu = Cpu::new();

    cpu.registers.l = 0xB1;
    sla(&mut cpu.registers.f, &mut cpu.registers.l);

    let check = vec![cpu.registers.l, cpu.registers.f.carry_flag()];

    assert_eq!(check, [0x62, 0x01]);
}

#[test]
fn sla_hl_test() {
    let mut cpu = Cpu::new();
    let mut mmu = Interconnect::new();

    let addr = 0xFFF;
    let value = 0xB1;
    mmu.write_mem(addr, value);

    sla_hl(&mut cpu.registers.f, &mut mmu, addr);

    let check = vec![mmu.read_mem(addr), cpu.registers.f.carry_flag()];

    assert_eq!(check, [0x62, 0x01]);
}

#[test]
fn sra_test() {
    let mut cpu = Cpu::new();

    cpu.registers.b = 0xB8;
    sra(&mut cpu.registers.f, &mut cpu.registers.b);

    let check = vec![cpu.registers.b, cpu.registers.f.carry_flag()];

    assert_eq!(check, [0xDC, 0x00]);
}

#[test]
fn sra_hl_test() {
    let mut cpu = Cpu::new();
    let mut mmu = Interconnect::new();

    let addr = 0xFFF;
    let value = 0xB8;
    mmu.write_mem(addr, value);

    sra_hl(&mut cpu.registers.f, &mut mmu, addr);

    let check = vec![mmu.read_mem(addr), cpu.registers.f.carry_flag()];
    assert_eq!(check, [0xDC, 0x00]);
}

#[test]
fn swap_test() {
    let mut cpu = Cpu::new();

    cpu.registers.b = 0xBC;

    swap(&mut cpu.registers.f, &mut cpu.registers.b);

    assert_eq!(cpu.registers.b, 0xCB);
}

#[test]
fn swap_hl_test() {
    let mut cpu = Cpu::new();
    let mut mmu = Interconnect::new();

    let addr = 0xFF80;
    let value = 0xBC;

    mmu.write_mem(addr, value);

    swap_hl(&mut cpu.registers.f, &mut mmu, addr);

    assert_eq!(mmu.read_mem(addr), 0xCB);
}

#[test]
fn srl_test() {
    let mut cpu = Cpu::new();

    cpu.registers.b = 0x8F;

    srl(&mut cpu.registers.f, &mut cpu.registers.b);

    let check = vec![cpu.registers.b, cpu.registers.f.carry_flag()];

    assert_eq!(check, [0x47, 0x01]);
}

#[test]
fn srl_hl_test() {
    let mut cpu = Cpu::new();
    let mut mmu = Interconnect::new();

    let addr = 0xFFFF;
    let value = 0x8F;

    mmu.write_mem(addr, value);

    srl_hl(&mut cpu.registers.f, &mut mmu, addr);

    let check = vec![mmu.read_mem(addr), cpu.registers.f.carry_flag()];

    assert_eq!(check, [0x47, 0x01]);
}

#[test]
fn bit_n_r_test() {
    let mut cpu = Cpu::new();

    cpu.registers.b = 0;

    bit_n_r(&mut cpu.registers.f, &mut cpu.registers.b, 2);

    let check = vec![cpu.registers.b, cpu.registers.f.zero_flag()];

    assert_eq!(check, [0x00, 0x01]);
}

#[test]
fn bit_n_hl_test() {
    let mut cpu = Cpu::new();
    let mut mmu = Interconnect::new();

    let addr = 0xFFF;

    mmu.write_mem(addr, 0);

    bit_n_hl(&mut cpu.registers.f, &mut mmu, addr, 2);

    let check = vec![mmu.read_mem(addr), cpu.registers.f.zero_flag()];

    assert_eq!(check, [0x00, 0x01]);
}

#[test]
fn res_n_r_test() {
    let mut cpu = Cpu::new();

    cpu.registers.b = 0x69;

    res_n_r(&mut cpu.registers.b, 6);

    assert_eq!(cpu.registers.b, 0x29)
}

#[test]
fn res_n_hl_test() {
    let mut mmu = Interconnect::new();

    let addr: u16 = 0xFFF;

    mmu.write_mem(addr, 0x69);

    res_n_hl(&mut mmu, addr, 6);

    assert_eq!(mmu.read_mem(addr), 0x29);
}

#[test]
fn set_n_r_test() {
    let mut cpu = Cpu::new();

    cpu.registers.b = 0x69;

    set_n_r(&mut cpu.registers.b, 7);

    assert_eq!(cpu.registers.b, 0xE9);
}

#[test]

fn set_n_hl_test() {
    let mut mmu = Interconnect::new();

    let addr: u16 = 0xFFF;

    mmu.write_mem(addr, 0x69);

    set_n_hl(&mut mmu, addr, 7);

    assert_eq!(mmu.read_mem(addr), 0xE9);
}

/*************************************************************************
 * IO TESTS
 *************************************************************************/

#[test]
fn ld_a_from_io_test() {
    let mut cpu = Cpu::new();
    let mut mmu = Interconnect::new();

    let offset: u8 = 0x02;
    let value = 0xAB;
    mmu.write_mem(0xFF00 + (offset as u16), value);

    ld_a_from_io(&mut cpu, &mmu, offset);

    assert_eq!(cpu.registers.a, mmu.read_mem(0xFF02));
}

#[test]
fn ld_a_from_io_c_test() {
    let mut cpu = Cpu::new();
    let mut mmu = Interconnect::new();

    let value = 0xAB;
    cpu.registers.c = 0xC;
    mmu.write_mem(0xFF0C, value);

    ld_a_from_io_c(&mut cpu, &mmu);

    assert_eq!(cpu.registers.a, mmu.read_mem(0xFF0C));
}

#[test]
fn ld_io_from_a_test() {
    let mut cpu = Cpu::new();
    let mut mmu = Interconnect::new();

    let offset = 0x02;

    cpu.registers.a = 0x81;

    ld_io_from_a(&cpu, &mut mmu, offset);

    assert_eq!(cpu.registers.a, mmu.read_mem(0xFF02));
}

#[test]
fn ld_io_c_from_a_test() {
    let mut cpu = Cpu::new();
    let mut mmu = Interconnect::new();

    cpu.registers.a = 0x81;
    cpu.registers.c = 0x0C;

    ld_io_c_from_a(&cpu, &mut mmu);

    assert_eq!(cpu.registers.a, mmu.read_mem(0xFF0C));
}
