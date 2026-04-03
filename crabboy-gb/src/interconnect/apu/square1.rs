use crate::interconnect::apu::Channel;
use crate::interconnect::apu::DUTY_CYCLES;
use crate::nth_bit;

use super::envelope::VolumeEnvelope;
use super::length_counter::LengthCounter;
use super::sweep::FrequencySweep;

#[derive(Clone, Copy)]
pub struct Square1 {
    vol_envelope: VolumeEnvelope,
    freq_sweep: FrequencySweep,
    pub length_counter: LengthCounter,
    timer: i32,
    sequence: i32,
    duty: u8,
    pub output: u8,
    channel_enabled: bool,
    dac_enabled: bool,
}

impl Square1 {
    pub fn new() -> Square1 {
        let mut lc = LengthCounter::new();
        lc.set_full_length(64);

        Square1 {
            vol_envelope: VolumeEnvelope::new(),
            freq_sweep: FrequencySweep::new(),
            length_counter: LengthCounter::new(),
            timer: 0,
            sequence: 0,
            duty: 0,
            output: 0,
            channel_enabled: false,
            dac_enabled: false,
        }
    }
    pub fn power_off(&mut self) {
        self.freq_sweep.power_off();
        self.vol_envelope.power_off();
        self.length_counter.power_off();

        self.channel_enabled = false;
        self.dac_enabled = false;
        self.sequence = 0;
        self.duty = 0;
    }


    pub fn trigger(&mut self) {
        self.timer = (2048 * self.freq_sweep.frequency()) * 4;

        self.vol_envelope.trigger();
        self.freq_sweep.trigger();

        if self.freq_sweep.enabled() {
            self.channel_enabled = self.dac_enabled;
        } else {
            self.channel_enabled = false;
        }
    }

    pub fn tick(&mut self) {
        if self.timer <= 0 {
            self.timer = (2048 - self.freq_sweep.frequency()) * 4;

            self.sequence = (self.sequence + 1) & 7;

            if self.enabled() {
                self.output = if DUTY_CYCLES[self.duty as usize][self.sequence as usize] {
                    self.vol_envelope.volume()
                } else {
                    0
                };
            } else {
                self.output = 0;
            }
        }
    }

    pub fn sweep_clock(&mut self) {
        self.freq_sweep.step();

        if self.freq_sweep.enabled() {
            self.channel_enabled = false;
        }
    }

    pub fn envelope_clock(&mut self) {
        self.vol_envelope.step();
    }
}

impl Channel for Square1 {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF10 => self.freq_sweep.read_nr10() | 0x80,
            0xFF11 => (self.duty << 6) | 0x3F,
            0xFF12 => self.vol_envelope.read_nr2(),
            0xFF13 => 0xFF,
            0xFF14 => ((self.length_counter.enabled() as u8) >> 6) | 0xBF,
            _ => panic!("NOT A CHANNEL 1 REGISTER"),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF10 => {
                self.freq_sweep.write_nr10(value);

                if !self.freq_sweep.enabled() {
                    self.channel_enabled = false;
                }
            }
            0xFF11 => {
                self.duty = value >> 6;
                self.length_counter.set_length(value & 0x3F);
            }
            0xFF12 => {
                self.dac_enabled = (value & 0xF8) != 0;
                self.channel_enabled &= self.dac_enabled;

                self.vol_envelope.write_nr2(value);
            }
            0xFF13 => self.freq_sweep.write_nr13(value),
            0xFF14 => {
                self.freq_sweep.write_nr14(value);
                self.length_counter.write_nr4(value);

                if self.length_counter.enabled() && self.length_counter.zero() {
                    self.channel_enabled = false;
                } else if nth_bit!(value, 7) == 1 {
                    self.trigger();
                }
            }
            _ => (),
        };
    }

    fn length_clock(&mut self) {
        self.length_counter.step();

        if self.length_counter.enabled() && self.length_counter.zero() {
            self.channel_enabled = false;
        }
    }

    fn enabled(&self) -> bool {
        self.dac_enabled && self.channel_enabled
    }
}
