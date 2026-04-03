use core::fmt;
use serde::{Serialize, Deserialize};

fn rom_size_as_str(value: u8) -> &'static str {
    match value {
        0x00 => "32 KiB",
        0x01 => "64 KiB",
        0x02 => "128 KiB",
        0x03 => "256 KiB",
        0x04 => "512 KiB",
        0x05 => "1 MiB",
        0x06 => "2 MiB",
        0x07 => "4 MiB",
        0x08 => "8 MiB",
        0x52 => "1.1 MiB",
        0x53 => "1.2 MiB",
        0x54 => "1.5 MiB",
        _ => panic!("NOT A  ROM SIZE"),
    }
}

pub fn ram_size(value: u8) -> usize {
    match value {
        0x00 => 0,
        0x01 => 0,
        0x02 => 0x2000,
        0x03 => 0x8000,
        0x04 => 0x20000,
        0x05 => 0x10000,
        _ => panic!("NOT A RAM SIZE"),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CartridgeType {
    ROMOnly,
    MBC1,
    MBC1RAM,
    MBC1RAMBattery,
    MBC2,
    MBC2Battery,
    ROMRAM1,
    ROMRAMBattery1,
    MMM01,
    MMM01RAM,
    MMM01RAMBattery,
    MBC3TimerBattery,
    MBC3TimerRAMBattery,
    MBC3,
    MBC3RAM,
    MBC3RAMBattery,
    MBC5,
    MBC5RAM,
    MBC5RAMBattery,
    MBC5Rumble,
    MBC5RumbleRAM,
    MBC5RumbleRAMBattery,
}

impl std::fmt::Display for CartridgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub fn u8_to_cart_type(value: u8) -> CartridgeType {
    match value {
        0x00 => CartridgeType::ROMOnly,
        0x01 => CartridgeType::MBC1,
        0x02 => CartridgeType::MBC1RAM,
        0x03 => CartridgeType::MBC1RAMBattery,
        0x05 => CartridgeType::MBC2,
        0x06 => CartridgeType::MBC2Battery,
        0x0F => CartridgeType::MBC3TimerBattery,
        0x10 => CartridgeType::MBC3TimerRAMBattery,
        0x11 => CartridgeType::MBC3,
        0x12 => CartridgeType::MBC3RAM,
        0x13 => CartridgeType::MBC3RAMBattery,
        0x19 => CartridgeType::MBC5,
        0x1A => CartridgeType::MBC5RAM,
        0x1B => CartridgeType::MBC5RAMBattery,
        0x1C => CartridgeType::MBC5Rumble,
        0x1D => CartridgeType::MBC5RumbleRAM,
        0x1E => CartridgeType::MBC5RumbleRAMBattery,
        _ => panic!("CARTYPE TYPE NOT IMPLEMENTED: {:#X}", value),
    }
}
