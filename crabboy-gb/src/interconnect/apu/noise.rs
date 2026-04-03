use crate::nth_bit;

use super::{envelope::VolumeEnvelope, length_counter::LengthCounter, Channel};

const DIVISOR_CODES: [i32; 8] = [8, 16, 32, 48, 64, 80, 96, 112];

#[derive(Clone, Copy)]
pub struct Noise {
    pub length_counter: LengthCounter,
    vol_envelope: VolumeEnvelope,
    timer: i32,
    shift: i32,
    lsfr: u16,
    pub output: u8,
    div_mode: u8,
    width_mode: bool,
    dac_enabled: bool,
    channel_enabled: bool,
}

impl Noise {
    pub fn new() -> Noise {
        let mut lc = LengthCounter::new();
        lc.set_full_length(64);
        Noise {
            length_counter: lc,
            vol_envelope: VolumeEnvelope::new(),
            timer: 0,
            shift: 0,
            lsfr: 0x7FFF,
            output: 0,
            div_mode: 0,
            width_mode: false,
            dac_enabled: false,
            channel_enabled: false,
        }
    }

    pub fn power_off(&mut self) {
        self.vol_envelope.power_off();
        self.length_counter.power_off();

        self.channel_enabled = false;
        self.dac_enabled = false;

        self.shift = 0;
        self.width_mode = false;
        self.div_mode = 0;
    }

    pub fn trigger(&mut self) {
        self.vol_envelope.trigger();
        self.timer = DIVISOR_CODES[self.div_mode as usize] << self.shift;
        self.lsfr = 0x7FFF;
        self.channel_enabled = self.dac_enabled;
    }

    pub fn envelope_clock(&mut self) {
        self.vol_envelope.step();
    }

    pub fn tick(&mut self) {
        if self.timer <= 0 {
            self.timer = DIVISOR_CODES[self.div_mode as usize] << self.shift;
            let xor_result = (self.lsfr & 0b01) ^ ((self.lsfr & 0b10) >> 1);
            self.lsfr = (self.lsfr >> 1) | (xor_result << 14);

            if self.width_mode {
                self.lsfr &= !(1 << 6);
                self.lsfr |= xor_result << 6;
            }

            self.output = if self.enabled() && (self.lsfr & 1) == 0 {
                self.vol_envelope.volume()
            } else {
                0
            }
        }
    }
}

impl Channel for Noise {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF1F => 0xFF,
            0xFF20 => 0xFF,
            0xFF21 => self.vol_envelope.read_nr2(),
            0xFF22 => ((self.shift as u8) << 4) | ((self.width_mode as u8) << 3) | self.div_mode,
            0xFF23 => ((self.length_counter.enabled() as u8) << 6) | 0xBF,
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF20 => self.length_counter.set_length(value & 0x3F),
            0xFF21 => {
                self.dac_enabled = (value & 0xF8) != 0;
                self.channel_enabled &= self.dac_enabled;
                self.vol_envelope.write_nr2(value);
            }

            0xFF22 => {
                self.shift = value as i32 >> 4;
                self.width_mode = nth_bit!(value, 3) != 0;
                self.div_mode = value & 0b111;
            }

            0xFF23 => {
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
