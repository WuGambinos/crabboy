use crate::nth_bit;

#[derive(Clone, Copy)]
pub struct FrequencySweep {
    freq: i32,
    shadow_freq: i32,
    timer: i32,

    period: u8,
    negative_direction: u8,
    shift: u8,

    enabled: bool,
    overflow: bool,
}

impl FrequencySweep {
    pub fn new() -> FrequencySweep {
        FrequencySweep {
            freq: 0,
            shadow_freq: 0,
            timer: 0,
            period: 0,
            negative_direction: 0,
            shift: 0,
            enabled: false,
            overflow: false,
        }
    }
    pub fn enabled(&self) -> bool {
        !self.overflow
    }

    pub fn frequency(&self) -> i32 {
        self.freq
    }

    pub fn write_nr10(&mut self, value: u8) {
        self.period = (value >> 4) & 0b111;
        self.negative_direction = nth_bit!(value, 3);
        self.shift = value & 0b111;
    }

    pub fn read_nr10(&self) -> u8 {
        (self.period << 4) | ((self.negative_direction as u8) << 3) | self.shift
    }

    pub fn write_nr13(&mut self, value: u8) {
        self.freq = (self.freq & 0x700) | value as i32;
    }

    pub fn write_nr14(&mut self, value: u8) {
        self.freq = (self.freq & 0xFF) | ((value as i32 & 0b111) << 8);
    }

    pub fn power_off(&mut self) {
        self.enabled = false;
        self.overflow = false;

        self.timer = 0;
        self.freq = 0;
        self.shadow_freq = 0;

        self.period = 0;
        self.negative_direction = 0;
        self.shift = 0;
    }

    pub fn trigger(&mut self) {
        let period = self.period;
        self.overflow = false;
        self.shadow_freq = self.freq;

        self.timer = if period != 0 { period as i32 } else { 8 };

        self.enabled = period != 0 || self.shift != 0;

        if self.shift > 0 {
            self.calc_freq();
        }
    }

    pub fn calc_freq(&mut self) -> i32 {
        let mut new_frequency = self.shadow_freq >> self.shift;

        let is_decreasing = self.negative_direction == 1;
        if is_decreasing {
            new_frequency = self.shadow_freq - new_frequency;
        } else {
            new_frequency = self.shadow_freq + new_frequency;
        }

        if new_frequency > 2047 {
            self.overflow = true;
        }

        return new_frequency;
    }

    pub fn step(&mut self) {
        if !self.enabled {
            return;
        }

        if self.timer <= 0 {
            self.timer = if self.period != 0 {
                self.period as i32
            } else {
                8
            };

            if self.period != 0 {
                let new_freq = self.calc_freq();

                if !self.overflow && self.shift != 0 {
                    self.shadow_freq = new_freq;
                    self.freq = new_freq;
                    self.calc_freq();
                }
            }
        }
    }
}
