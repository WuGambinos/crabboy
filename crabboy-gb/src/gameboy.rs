use crate::constants::PC_AFTER_BOOT;
use crate::cpu::Cpu;
use crate::interconnect::cartridge::cartridge_info::ram_size;
use crate::interconnect::cartridge::cartridge_info::u8_to_cart_type;
use crate::interconnect::cartridge::cartridge_info::CartridgeType;
use crate::interconnect::cartridge::Cartridge;
use crate::interconnect::Interconnect;

use anyhow::Error;
use anyhow::Result;

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use yazi::*;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct GameBoy {
    pub cpu: Cpu,
    pub interconnect: Interconnect,
    pub booted: bool,
}

impl GameBoy {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            interconnect: Interconnect::new(),
            booted: false,
        }
    }

    pub fn get_state(&self) -> Vec<u8> {
        let encoded: Vec<u8> = bitcode::serialize(&self).unwrap();
        let compressed = compress(&encoded, Format::Zlib, CompressionLevel::Default).unwrap();
        return compressed;
    }

    pub fn load_state(&mut self, compressed_state: Vec<u8>) {
        let (decompressed, checksum) = decompress(&compressed_state, Format::Zlib).unwrap();
        let decoded: GameBoy = bitcode::deserialize(&decompressed[..]).unwrap();
        *self = decoded;
    }

    pub fn boot(&mut self, game: &str, skip_boot: bool) -> Result<(), Error> {
        self.booted = true;
        let boot_rom = if !skip_boot {
            let boot_rom = "roms/bootix_dmg.bin";
            let boot_rom_path: &Path = Path::new(boot_rom);
            read_file(boot_rom_path)?
        } else {
            Vec::new()
        };

        let game_rom_path: &Path = Path::new(game);
        let game_rom: Vec<u8> = read_file(game_rom_path)?;

        let cart_type_value: u8 = game_rom[0x147];
        let rom_size: u8 = game_rom[0x148];
        let ram_s: u8 = game_rom[0x149];

        let ram = vec![0x00; ram_size(ram_s) as usize];
        let cart_type: CartridgeType = u8_to_cart_type(cart_type_value);

        self.interconnect.cartridge = Cartridge::new(&game_rom, &ram, &cart_type);
        let file_name: Vec<&str> = game_rom_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split('.')
            .collect();
        self.interconnect.cartridge.title = file_name[0].to_string();
        println!("FILE NAME: {}", file_name[0]);
        println!("CART TYPE: {:?}", cart_type);
        println!("ROM_SIZE: {:#X}", rom_size);
        println!("RAM_SIZE: {:#X} KiB", ram_size(ram_s));
        println!("CHECKSUM: {}", self.interconnect.cartridge.checksum());

        self.cpu.pc = if skip_boot {
            //self.interconnect.load_game_rom(&game_rom);
            self.interconnect.boot_active = false;

            PC_AFTER_BOOT
        } else {
            self.interconnect.load_game_rom(&game_rom);
            self.interconnect.load_boot_rom(&boot_rom);
            0x0000
        };
        Ok(())
    }
}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    fs::read(path)
}
