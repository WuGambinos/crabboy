use crate::nth_bit;

use super::{envelope::VolumeEnvelope, length_counter::LengthCounter, Channel, DUTY_CYCLES};

#[derive(Clone, Copy)]
pub struct Square2 {
    vol_envelope: VolumeEnvelope,
    pub length_counter: LengthCounter,
    duty: u8,
    sequence: i32,
    timer: i32,
    channel_enabled: bool,
    dac_enabled: bool,
    pub output: u8,
    freq: i32,
}

impl Square2 {
    pub fn new() -> Square2 {
        let mut len_counter = LengthCounter::new();
        len_counter.set_full_length(64);
        Square2 {
            timer: 0,
            sequence: 0,
            freq: 0,
            duty: 0,
            channel_enabled: false,
            dac_enabled: false,
            output: 0,
            vol_envelope: VolumeEnvelope::new(),
            length_counter: len_counter,
        }
    }

    pub fn tick(&mut self) {
        if self.timer <= 0 {
            self.timer = (2048 - self.freq) * 4;

            self.sequence = (self.sequence + 1) & 7;

            if self.enabled() {
                self.output = if DUTY_CYCLES[self.duty as usize][self.sequence as usize] {
                    self.vol_envelope.volume()
                } else {
                    0
                };
            }
        }
    }

    pub fn envelope_clock(&mut self) {
        self.vol_envelope.step();
    }

    pub fn power_off(&mut self) {
        self.vol_envelope.power_off();
        self.length_counter.power_off();

        self.channel_enabled = false;
        self.dac_enabled = false;

        self.sequence = 0;
        self.freq = 0;

        self.duty = 0;
    }

    pub fn trigger(&mut self) {
        self.timer = (2048 - self.freq) * 4;
        self.vol_envelope.trigger();
        self.channel_enabled = self.dac_enabled;
    }
}

impl Channel for Square2 {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF15 => 0xFF,
            0xFF16 => (self.duty << 6) | 0x3F,
            0xFF17 => self.vol_envelope.read_nr2(),
            0xFF18 => 0xFF,
            0xFF19 => ((self.length_counter.enabled() as u8) << 6) | 0xBF,
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF16 => {
                self.duty = value >> 6;
                self.length_counter.set_length(value & 0x3F);
            }

            0xFF17 => {
                self.dac_enabled = (value & 0xF8) != 0;
                self.channel_enabled &= self.dac_enabled;
                self.vol_envelope.write_nr2(value);
            }

            0xFF18 => {
                self.freq = (self.freq & 0x700) | value as i32;
            }

            0xFF19 => {
                self.freq = (self.freq & 0xFF) | ((value as i32 & 0b111) << 8);
                self.length_counter.write_nr4(value);

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
