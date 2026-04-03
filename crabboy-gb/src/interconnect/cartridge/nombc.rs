use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NoMbcState {
    pub rom: Vec<u8>,
}

impl NoMbcState {
    pub fn new(rom: &[u8]) -> NoMbcState {
        NoMbcState { rom: rom.to_vec() }
    }

    pub fn read(&self, addr: u16) -> u8 {
        if addr < 0x8000 {
            self.rom[addr as usize]
        } else {
            0xFF
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {}
}
