use std::ops::Range;
use crate::interconnect::ppu::Rgb;
use crate::cpu::interrupts::InterruptType;

pub const CLOCK_SPEED: usize = 4_194_304;
pub const MAX_CYCLES_PER_FRAME: usize = (CLOCK_SPEED as f32 / 59.7275) as usize;
pub const PC_AFTER_BOOT: u16 = 0x100;
pub const TARGET_FRAME_TIME: u32 = 1000 / 60;


// PPU constants
pub const LINES_PER_FRAME: u8 = 154;
pub const TICKS_PER_LINE: u32 = 456;
pub const Y_RESOLUTION: u8 = 144;
pub const X_RESOLUTION: u8 = 160;
pub const BUFFER_SIZE: usize = (144 * 160) as usize;

pub const TILE_COLORS: [Rgb; 4] = [
    Rgb::new(255, 255, 255),
    Rgb::new(169, 169, 169),
    Rgb::new(84, 84, 84),
    Rgb::new(0, 0, 0),
];

// MMU Addresses
pub const SERIAL_TRASFER_DATA: u16 = 0xFF01;
pub const SERIAL_TRANSFER_CONTROL: u16 = 0xFF02;

pub const DIV: u16 = 0xFF04;
pub const TIMA: u16 = 0xFF05;
pub const TMA: u16 = 0xFF06;
pub const TAC: u16 = 0xFF07;
pub const LCDC: u16 = 0xFF40;

pub const INTERRUPT_FLAG: u16 = 0xFF0F;
pub const INTERRUPT_ENABLE: u16 = 0xFFFF;

pub const BCPS: u16 = 0xFF68;
pub const BCPD: u16 = 0xFF69;
pub const OCPS: u16 = 0xFF6A;
pub const OCPD: u16 = 0xFF6B;

pub const INTERRUPTS: [InterruptType; 5] = [
    InterruptType::VBlank,
    InterruptType::LcdStat,
    InterruptType::Timer,
    InterruptType::Serial,
    InterruptType::Joypad,
];

// MMU Ranges
pub const BOOT: Range<u16> = 0x00..0x100;
pub const ROM_BANK: Range<u16> = 0x0000..0x8000;
pub const VRAM: Range<u16> = 0x8000..0xA000;
pub const EXTERNAL_RAM: Range<u16> = 0xA000..0xC000;
pub const WORK_RAM: Range<u16> = 0xC000..0xE000;
pub const OAM: Range<u16> = 0xFE00..0xFEA0;
pub const TIMER: Range<u16> = 0xFF04..0xFF08;
pub const LCD: Range<u16> = 0xFF40..0xFF4C;
pub const IO: Range<u16> = 0xFF00..0xFF80;
pub const HIGH_RAM: Range<u16> = 0xFF80..0xFFFF;

pub const BOOT_START: u16 = 0x000;
pub const BOOT_END: u16 = 0x100;

pub const ROM_BANK_START: u16 = 0x0000;
pub const ROM_BANK_END: u16 = 0x8000;

pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0xA000;

pub const EXTERNAL_RAM_START: u16 = 0xA000;
pub const EXTERNAL_RAM_END: u16 = 0xC000;

pub const WORK_RAM_START: u16 = 0xC000;
pub const WORK_RAM_END: u16 = 0xE000;

pub const OAM_START: u16 = 0xFE00;
pub const OAM_END: u16 = 0xFEA0;

pub const TIMER_START: u16 = 0xFF04;
pub const TIMER_END: u16 = 0xFF08;

pub const IO_START: u16 = 0xFF00;
pub const IO_END: u16 = 0xFF80;

pub const HIGH_RAM_START: u16 = 0xFF80;
pub const HIGH_RAM_END: u16 = 0xFFFF;

pub const LCD_START: u16 = 0xFF40;
pub const LCD_END: u16 = 0xFF4C;

pub const ROM_BANK_SIZE: usize = 0x4000;
pub const RAM_BANK_SIZE: usize = 0x2000;
