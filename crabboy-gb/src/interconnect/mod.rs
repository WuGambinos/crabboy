#![allow(clippy::must_use_candidate)]

pub mod cartridge;
pub mod joypad;
mod mmu;
pub mod ppu;
mod serial;

use log::debug;
use log::warn;

use crate::constants::{
    BCPD, BCPS, BOOT, BOOT_END, BOOT_START, EXTERNAL_RAM, EXTERNAL_RAM_END, EXTERNAL_RAM_START,
    HIGH_RAM, HIGH_RAM_END, HIGH_RAM_START, INTERRUPT_ENABLE, IO, IO_END, IO_START, LCD, LCD_END,
    LCD_START, OAM, OAM_END, OAM_START, RAM_BANK_SIZE, ROM_BANK, ROM_BANK_END, ROM_BANK_SIZE,
    ROM_BANK_START, TIMER, TIMER_END, TIMER_START, VRAM, VRAM_END, VRAM_START, WORK_RAM,
    WORK_RAM_END, WORK_RAM_START, OCPS, OCPD,
};
use crate::cpu::interrupts::request_interrupt;
use crate::cpu::interrupts::InterruptType;
use crate::cpu::timer::Timer;
use crate::interconnect::joypad::Joypad;
use crate::interconnect::mmu::Mmu;
use crate::interconnect::serial::SerialOutput;
use crate::interconnect::ppu::{PaletteSpec, Rgb, Ppu};

use self::cartridge::Cartridge;
use self::joypad::Key;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Interconnect {
    pub cartridge: Cartridge,
    pub mmu: Mmu,
    pub timer: Timer,
    pub ppu: Ppu,
    pub serial: SerialOutput,
    pub joypad: Joypad,
    pub boot_active: bool,
    pub write_enabled: bool,
    pub ticks: u64,
    pub cgb_mode: bool,
}

impl Interconnect {
    pub fn new() -> Self {
        Self {
            cartridge: Cartridge::empty(),
            mmu: Mmu::new(),
            timer: Timer::new(),
            ppu: Ppu::new(),
            serial: SerialOutput::new(),
            joypad: Joypad::init(),
            boot_active: true,
            write_enabled: true,
            ticks: 0,
            cgb_mode: false,
        }
    }

    pub fn key_down(&mut self, key: Key) {
        self.joypad.key_down(key);
        request_interrupt(self, InterruptType::Joypad);
    }

    pub fn key_up(&mut self, key: Key) {
        self.joypad.key_up(key);
        request_interrupt(self, InterruptType::Joypad);
    }

    pub fn log_timer(&self) {
        debug!(
            "DIV: {:#X} TIMA: {:#X} TMA: {:#X} TAC: {:#X}",
            self.timer.div(),
            self.timer.tima(),
            self.timer.tma(),
            self.timer.tac()
        );
    }

    pub fn log_vram(&self) {
        for i in (0x8000..0x9FFF).rev() {
            debug!("Addr: {:#X} Val: {:#X}", i, self.read_mem(i));
        }
    }

    pub fn dma_transfer(&mut self, value: u8) {
        let addr: u16 = u16::from(value) << 8;

        for i in 0..0xA0 {
            self.write_mem(0xFE00 + i, self.read_mem(addr + i));
        }
    }

    pub fn write_mem(&mut self, addr: u16, value: u8) {
        match addr {
            ROM_BANK_START..ROM_BANK_END => self.cartridge.mbc.write(addr, value),
            VRAM_START..VRAM_END => self.ppu.write_vram(addr, value),
            EXTERNAL_RAM_START..EXTERNAL_RAM_END => self.cartridge.mbc.write(addr, value),
            WORK_RAM_START..WORK_RAM_END => self.mmu.write_work_ram(addr - 0xC000, value),

            OAM_START..OAM_END => {
                if self.ppu.dma_transferring() {
                    return;
                }
                self.ppu.write_oam(addr, value);
            }

            TIMER_START..TIMER_END => self.timer.timer_write(addr, value),
            IO_START..IO_END => match addr {
                a if self.cgb_mode && a == BCPS => {
                    log::info!("WRITE MEM CGB BG COLOR SPEC");
                    self.ppu.bg_colors[self.ppu.bcps.addr() as usize] = Rgb::new(0, 0, 0);
                    if  self.ppu.bcps.auto_increment() == 1 {
                        self.ppu.bcps.set_addr(self.ppu.bcps.addr() + 1);
                    }
                }

                a if self.cgb_mode && a == BCPD => {
                    log::info!("WRITE MEM CGB BG COLOR DATA");
                    self.ppu.bg_colors[self.ppu.bcps.addr() as usize] = Rgb::new(0, 0, 0);
                }

                a if self.cgb_mode && a == OCPS => {
                    log::info!("WRITE MEM CGB BG COLOR SPEC");
                    self.ppu.ocps = PaletteSpec::from_bytes([value]);
                }

                a if self.cgb_mode && a == OCPD => {
                    log::info!("WRITE MEM CGB BG COLOR DATA");
                    self.ppu.sprite_colors[self.ppu.ocps.addr() as usize] = Rgb::new(1, 0, 0);
                    if  self.ppu.ocps.auto_increment() == 1 {
                        self.ppu.ocps.set_addr(self.ppu.ocps.addr() + 1);
                    }
                }

                0xFFF0 => self.joypad.write(value),
                LCD_START..LCD_END => self.ppu.write_lcd(addr, value),

                /*
                0xFF51..0xFF70 => {
                    log::info!("WRITE IO: {:#X}", addr);
                    std::process::exit(0);
                }
                */
                _ => self.mmu.write_io(addr - 0xFF00, value),
            },

            HIGH_RAM_START..HIGH_RAM_END => self.mmu.write_hram(addr - 0xFF80, value),
            INTERRUPT_ENABLE => self.mmu.enable_interrupt(value),
            _ => panic!("NOT A VALID WRITE ADDRESS"),
        }
    }

    pub fn read_mem(&self, addr: u16) -> u8 {
        match addr {
            a if self.boot_active && BOOT.contains(&a) => self.mmu.read_boot(a),
            ROM_BANK_START..ROM_BANK_END => self.cartridge.mbc.read(addr),
            VRAM_START..VRAM_END => self.ppu.read_vram(addr),
            EXTERNAL_RAM_START..EXTERNAL_RAM_END => self.cartridge.mbc.read(addr),
            WORK_RAM_START..WORK_RAM_END => self.mmu.read_work_ram(addr - 0xC000),

            OAM_START..OAM_END => {
                if self.ppu.dma_transferring() {
                    return 0xFF;
                }

                self.ppu.read_oam(addr)
            }

            TIMER_START..TIMER_END => self.timer.timer_read(addr),
            IO_START..IO_END => match addr {
                a if self.cgb_mode && a == BCPS => {
                    //log::info!("READ MEM CGB BCPS");
                    self.ppu.bcps.into_bytes()[0]
                }

                a if self.cgb_mode && a == BCPD => {
                    log::info!("READ MEM CGB BCPD");
                    std::process::exit(0);
                    0
                }

                a if self.cgb_mode && a == OCPS => {
                    //log::info!("READ MEM CGB OCPS");
                    self.ppu.ocps.into_bytes()[0]
                }

                a if self.cgb_mode && a == OCPD => {
                    log::info!("READ MEM CGB OCPD");
                    std::process::exit(0);
                    0
                }

                0xFFF0 => self.joypad.read(),
                LCD_START..LCD_END => self.ppu.read_lcd(addr),

                /*
                0xFF51..0xFF70 => {
                    log::info!("READ IO: {:#X}", addr);
                    std::process::exit(0);
                }
                */
                _ => self.mmu.read_io(addr - 0xFF00),
            },

            HIGH_RAM_START..HIGH_RAM_END => self.mmu.read_hram(addr - 0xFF80),
            INTERRUPT_ENABLE => self.mmu.read_interrupt_enable(),
            _ => {
                warn!("NOT REACHABLE ADDR: {:#X}", addr);
                0
            }
        }
        /*
        if self.boot_active && BOOT.contains(&addr) {
            self.mmu.read_boot(addr)
        } else if ROM_BANK.contains(&addr) {
            self.cartridge.mbc.read(addr)
        } else if VRAM.contains(&addr) {
            self.ppu.read_vram(addr)
        } else if EXTERNAL_RAM.contains(&addr) {
            self.cartridge.mbc.read(addr)
        } else if WORK_RAM.contains(&addr) {
            self.mmu.read_work_ram(addr - 0xC000)
        } else if OAM.contains(&addr) {
            if self.ppu.dma_transferring() {
                0xFF
            } else {
                self.ppu.read_oam(addr)
            }
        } else if TIMER.contains(&addr) {
            self.timer.timer_read(addr)
        } else if IO.contains(&addr) {
            if addr == 0xFF68 {
                log::info!("READ MEM CGB BG COLOR SPEC");
                self.ppu.cgb_bg_color_spec.into_bytes()[0]
            } else if addr == 0xFF69 {
                log::info!("READ MEM CGB BG COLOR DATA");
                self.ppu.cgb_bg_color_data.into_bytes()[0]
            } else if (addr >= 0xFF51 && addr <= 0xFF70) {
                log::info!("READ IO: {:#X}", addr);
                std::process::exit(0);
            } else if addr == 0xFF00 {
                self.joypad.read()
            } else if LCD.contains(&addr) {
                self.ppu.read_lcd(addr)
            } else {
                self.mmu.read_io(addr - 0xFF00)
            }
        } else if HIGH_RAM.contains(&addr) {
            self.mmu.read_hram(addr - 0xFF80)
        } else if addr == INTERRUPT_ENABLE {
            self.mmu.read_interrupt_enable()
        } else {
            warn!("NOT REACHABLE ADDR: {:#X}", addr);
            0
        }
        */
    }

    pub fn load_game_rom(&mut self, rom: &[u8]) {
        /*
        for (i, _) in rom.iter().enumerate() {
            self.write_mem(i as u16, rom[i]);
        }
        */
        //self.cartridge.mbc.read(addr) = rom.to_vec();
    }

    pub fn load_boot_rom(&mut self, rom: &[u8]) {
        for (i, _) in rom.iter().enumerate() {
            self.mmu.write_boot(i as u16, rom[i]);
        }
    }

    pub fn emu_tick(&mut self, m_cycles: u32) {
        // Convert M cycles to T cycles
        let t_cycles = m_cycles * 4;

        for _ in 0..t_cycles {
            let interrupts = self.ppu.tick();

            for int in interrupts {
                request_interrupt(self, int);
            }
        }

        self.ticks = u64::from(t_cycles);

        let div_value: u8 = self.timer.div_clock.next(t_cycles) as u8;
        self.timer.set_div(self.timer.div().wrapping_add(div_value));

        let timer_enabled: bool = (self.timer.tac() & 0x04) != 0x00;
        if timer_enabled {
            let n = self.timer.tima_clock.next(t_cycles);

            for _ in 0..n {
                let tima_value = self.timer.tima().wrapping_add(1);
                self.timer.set_tima(tima_value);

                if self.timer.tima() == 0x00 {
                    self.timer.set_tima(self.timer.tma());
                    request_interrupt(self, InterruptType::Timer);
                }
            }
        }

        let dma_cycles = m_cycles;
        for _ in 0..dma_cycles {
            self.dma_tick();
        }
    }

    pub fn dma_tick(&mut self) {
        if !self.ppu.dma_active() {
            return;
        }

        if self.ppu.dma_start_delay() > 0 {
            let delay_value = self.ppu.dma_start_delay().wrapping_add(1);
            self.ppu.set_dma_start_delay(delay_value);
            return;
        }

        let addr: u16 = (u16::from(self.ppu.dma_value()) * 0x100) + u16::from(self.ppu.dma_byte());

        self.ppu
            .write_oam(u16::from(self.ppu.dma_byte()), self.read_mem(addr));

        let byte_value = self.ppu.dma_byte().wrapping_add(1);
        self.ppu.set_dma_byte(byte_value);

        self.ppu.set_dma_active(self.ppu.dma_byte() < 0xA0);
    }
}

impl Default for Interconnect {
    fn default() -> Self {
        Self::new()
    }
}
