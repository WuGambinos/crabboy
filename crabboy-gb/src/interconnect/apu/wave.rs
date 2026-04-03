use crate::nth_bit;

use super::{length_counter::LengthCounter, Channel};

#[derive(Clone, Copy)]
pub struct Wave {
    dac_enabled: bool,
    channel_enabled: bool,
    position: i32,
    timer: i32,
    ticks_since_read: i32,

    frequency: i32,
    volume_code: i32,
    last_address: i32,

    pub length_counter: LengthCounter,
    pub output: u8,

    waves: [u8; 16],
}

impl Wave {
    pub fn new() -> Wave {
        let mut lc = LengthCounter::new();
        lc.set_full_length(64);
        Wave {
            timer: 0,
            position: 4,
            ticks_since_read: 0,

            frequency: 0,
            volume_code: 0,
            last_address: 0,
            length_counter: lc,

            channel_enabled: false,
            output: 0,
            dac_enabled: false,
            waves: [0; 16],
        }
    }
    pub fn power_off(&mut self) {
        self.length_counter.power_off();

        self.channel_enabled = false;
        self.dac_enabled = false;

        self.position = 0;
        self.frequency = 0;
        self.volume_code = 0;

        self.ticks_since_read = 0;
        self.last_address = 0;
    }

    pub fn trigger(&mut self) {
        if self.enabled() && self.timer == 2 {
            let mut pos = self.position >> 1;

            if pos < 4 {
                self.waves[0] = self.waves[pos as usize];
            } else {
                pos &= !(0b11);
                self.waves.copy_within((pos as usize)..4, 0);
            }
        }

        self.timer = 6;

        self.position = 0;
        self.last_address = 0;

        self.channel_enabled = self.dac_enabled;
    }

    pub fn tick(&mut self) {
        self.ticks_since_read += 1;

        if self.timer <= 0 {
            self.timer = (2048 - self.frequency) << 1;

            if self.enabled() {
                self.ticks_since_read = 0;

                self.last_address = self.position >> 1;
                self.output = self.waves[self.last_address as usize];

                if (self.position & 1) == 1 {
                    self.output &= 0x0F;
                } else {
                    self.output >>= 4;
                }

                if self.volume_code > 0 {
                    self.output >>= self.volume_code - 1;
                } else {
                    self.output = 0;
                }

                self.position = (self.position + 1) & 31;
            } else {
                self.output = 0;
            }
        }
    }
}

impl Channel for Wave {
    fn read(&self, addr: u16) -> u8 {
        if addr >= 0xFF30 && addr <= 0xFF3F {
            if self.enabled() {
                if self.ticks_since_read < 2 {
                    return self.waves[self.last_address as usize];
                } else {
                    return 0xFF;
                }
            } else {
                return self.waves[addr as usize - 0xFF30];
            }
        }

        match addr {
            0xFF1A => ((self.dac_enabled as u8) << 7) | 0x7F,
            0xFF1B => 0xFF,
            0xFF1C => ((self.volume_code as u8) << 5) | 0x9F,
            0xFF1D => 0xFF,
            0xFF1E => ((self.length_counter.enabled() as u8) << 6) | 0xBF,
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        if addr >= 0xFF30 && addr <= 0xFF3F {
            if self.enabled() {
                if self.ticks_since_read < 2 {
                    self.waves[self.last_address as usize] = value;
                }
            } else {
                self.waves[addr as usize - 0xFF30] = value;
            }
            return;
        }

        match addr {
            0xFF1A => {
                self.dac_enabled = nth_bit!(value, 7) != 0;
                self.channel_enabled &= self.dac_enabled;
            }

            0xFF1B => self.length_counter.set_length(value),

            0xFF1C => self.volume_code = ((value >> 5) & 0b11) as i32,

            0xFF1D => self.frequency = (self.frequency & 0x700) | value as i32,

            0xFF1E => {
                self.length_counter.write_nr4(value);
                self.frequency = (self.frequency & 0xFF) | ((value as i32 & 0b111) << 8);

                if self.length_counter.enabled() && self.length_counter.zero() {
                    self.channel_enabled = false;
                } else if nth_bit!(value, 7) == 1 {
                    self.trigger();
                }
            }
            _ => (),
        }
    }

    fn length_clock(&mut self) {
        if self.length_counter.enabled() && self.length_counter.zero() {
            self.channel_enabled = false;
        }
    }

    fn enabled(&self) -> bool {
        self.dac_enabled && self.channel_enabled
    }
}
