pub mod cartridge_info;
mod mbc1;
mod mbc2;
mod mbc3;
mod mbc5;
mod nombc;

use cartridge_info::CartridgeType;
use mbc1::Mbc1State;
use mbc2::Mbc2State;
use mbc3::Mbc3State;
use mbc5::Mbc5State;
use nombc::NoMbcState;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Mbc { NoMbc(NoMbcState),
    Mbc1(Mbc1State),
    Mbc2(Mbc2State),
    Mbc3(Mbc3State),
    Mbc5(Mbc5State),
}

impl Mbc {
    pub fn read(&self, addr: u16) -> u8 {
        match self {
            Mbc::NoMbc(mbc) => mbc.read(addr),
            Mbc::Mbc1(mbc) => mbc.read(addr),
            Mbc::Mbc2(mbc) => mbc.read(addr),
            Mbc::Mbc3(mbc) => mbc.read(addr),
            Mbc::Mbc5(mbc) => mbc.read(addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match self {
            Mbc::NoMbc(mbc) => mbc.write(addr, value),
            Mbc::Mbc1(mbc) => mbc.write(addr, value),
            Mbc::Mbc2(mbc) => mbc.write(addr, value),
            Mbc::Mbc3(mbc) => mbc.write(addr, value),
            Mbc::Mbc5(mbc) => mbc.write(addr, value),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cartridge {
    pub title: String,
    pub cartridge_type: CartridgeType,
    pub valid_checksum: bool,
    pub mbc: Mbc,
}

impl Cartridge {
    pub fn empty() -> Cartridge {
        Cartridge {
            title: String::new(),
            cartridge_type: CartridgeType::ROMOnly,
            valid_checksum: false,
            mbc: Mbc::NoMbc(NoMbcState::new(&[])),
        }
    }

    pub fn new(rom: &Vec<u8>, ram: &Vec<u8>, cart_type: &CartridgeType) -> Cartridge {
        let mbc_test: Mbc = match cart_type {
            CartridgeType::ROMOnly => Mbc::NoMbc(NoMbcState::new(rom)),
            CartridgeType::MBC1 => Mbc::Mbc1(Mbc1State::new(rom, ram)),
            CartridgeType::MBC1RAM => Mbc::Mbc1(Mbc1State::new(rom, ram)),
            CartridgeType::MBC1RAMBattery => Mbc::Mbc1(Mbc1State::new(rom, ram)),
            CartridgeType::MBC2 => Mbc::Mbc2(Mbc2State::new(rom)),
            CartridgeType::MBC2Battery => Mbc::Mbc2(Mbc2State::new(rom)),
            CartridgeType::MBC3 => Mbc::Mbc3(Mbc3State::new(rom, ram)),
            CartridgeType::MBC3RAM => Mbc::Mbc3(Mbc3State::new(rom, ram)),
            CartridgeType::MBC3RAMBattery => Mbc::Mbc3(Mbc3State::new(rom, ram)),
            CartridgeType::MBC3TimerBattery => Mbc::Mbc3(Mbc3State::new(rom, ram)),
            CartridgeType::MBC3TimerRAMBattery => Mbc::Mbc3(Mbc3State::new(rom, ram)),
            CartridgeType::MBC5 => Mbc::Mbc5(Mbc5State::new(rom, ram)),
            CartridgeType::MBC5RAM => Mbc::Mbc5(Mbc5State::new(rom, ram)),
            CartridgeType::MBC5RAMBattery => Mbc::Mbc5(Mbc5State::new(rom, ram)),
            CartridgeType::MBC5Rumble => Mbc::Mbc5(Mbc5State::new(rom, ram)),
            CartridgeType::MBC5RumbleRAM => Mbc::Mbc5(Mbc5State::new(rom, ram)),
            CartridgeType::MBC5RumbleRAMBattery => Mbc::Mbc5(Mbc5State::new(rom, ram)),
            _ => Mbc::NoMbc(NoMbcState::new(rom)),
        };

        Cartridge {
            title: String::new(),
            cartridge_type: CartridgeType::ROMOnly,
            valid_checksum: false,
            mbc: mbc_test,
        }
    }

    pub fn checksum(&mut self) -> bool {
        let mut check_sum: i16 = 0;

        for addr in 0x134..=0x14C {
            check_sum = check_sum
                .wrapping_sub(self.mbc.read(addr) as i16)
                .wrapping_sub(1);
        }

        self.valid_checksum = (check_sum & 0xF) == (self.mbc.read(0x14D) & 0xF) as i16;
        self.valid_checksum
    }
}

