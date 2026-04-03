use crate::constants::ROM_BANK_SIZE;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Mbc2State {
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    rom_bank_number: usize,
    ram_enabled: bool,
}

impl Mbc2State {
    pub fn new(rom: &[u8]) -> Mbc2State {
        Mbc2State {
            rom: rom.to_vec(),
            ram: vec![0; 512],
            rom_bank_number: 1,
            ram_enabled: false,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rom[addr as usize],

            0x4000..=0x7FFF => {
                let new_addr = ROM_BANK_SIZE * self.rom_bank_number + (addr & 0x3FFF )as usize;
                let new_addr = new_addr & (self.rom.len() - 1);
                self.rom[new_addr]
            }

            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    let new_addr = (addr & 0x1FFF) as usize & (self.ram.len() - 1);
                    return self.ram[new_addr] | 0xF0;
                }
                0xFF
            }
            _ => panic!("NOT REACHABLE MBC2 {:#?}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x3FFF => {
                let bit_8_set = (addr & 0x100) > 0;
                let new_value = value & 0xF;
                if bit_8_set {
                    self.rom_bank_number = if new_value == 0 {
                        1
                    } else {
                        new_value as usize
                    };
                } else {
                    self.ram_enabled = (new_value) == 0x0A;
                }
            }

            0x4000..=0x7FFF => {}

            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    let new_addr = (addr & 0x1FFF) as usize & (self.ram.len() - 1);
                    let value = value & 0xF;
                    self.ram[new_addr] = value;
                }
            }
            _ => panic!("NOT REACHABLE MBC2 {:#?}", addr),
        }
    }
}
