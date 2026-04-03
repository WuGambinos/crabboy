use crate::constants::{RAM_BANK_SIZE, ROM_BANK_SIZE};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Mbc5State {
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    rom_bank_number: usize,
    ram_bank_number: usize,
    ram_enabled: bool,
}

impl Mbc5State {
    pub fn new(rom: &[u8], ram: &[u8]) -> Mbc5State {
        Mbc5State {
            rom: rom.to_vec(),
            ram: ram.to_vec(),
            rom_bank_number: 1,
            ram_bank_number: 0,
            ram_enabled: false,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rom[addr as usize],

            0x4000..=0x7FFF => {
                let new_addr = ROM_BANK_SIZE * self.rom_bank_number + (addr & 0x3FFF) as usize;
                let new_addr = new_addr & (self.rom.len() - 1);
                self.rom[new_addr]
            }

            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    let new_addr = (RAM_BANK_SIZE * self.ram_bank_number)
                        + (addr & 0x1FFF) as usize
                        & (self.ram.len() - 1);
                    return self.ram[new_addr];
                }
                0xFF
            }
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => {
                self.ram_enabled = value == 0x0A;
            }

            0x2000..=0x2FFF => {
                self.rom_bank_number = value as usize;
            }

            0x3000..=0x3FFF => {
                let new_value = ((value & 0x1) as u16) << 8;
                self.rom_bank_number |= new_value as usize;
            }
            0x4000..=0x7FFF => {
                self.ram_bank_number = (value & 0xF) as usize;
            }

            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    let new_addr = (RAM_BANK_SIZE * self.ram_bank_number)
                        + (addr & 0x1FFF) as usize
                        & (self.ram.len() - 1);
                    self.ram[new_addr] = value;
                }
            }
            _ => (),
        }
    }
}
