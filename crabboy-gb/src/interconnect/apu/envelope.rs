use crate::nth_bit;

#[derive(Clone, Copy)]
pub struct VolumeEnvelope {
    finished: bool,
    timer: i32,
    starting_volume: u8,
    add_mode: bool,
    period: u8,
    volume: u8,
}

impl VolumeEnvelope {
    pub fn new() -> VolumeEnvelope {
        VolumeEnvelope {
            finished: false,
            timer: 0,
            starting_volume: 0,
            add_mode: false,
            period: 0,
            volume: 0,
        }
    }

    pub fn power_off(&mut self) {
        self.finished = true;
        self.timer = 0;

        self.starting_volume = 0;
        self.add_mode = false;
        self.period = 0;

        self.volume = 0;
    }

    pub fn write_nr2(&mut self, value: u8) {
        self.starting_volume = value >> 4;
        self.add_mode = nth_bit!(value, 3) != 0;
        self.period = value & 0b111;
    }

    pub fn read_nr2(&self) -> u8 {
        let add_m = (self.add_mode as u8) << 3;

        (self.starting_volume << 4) | (add_m) | self.period
    }

    pub fn volume(&self) -> u8 {
        if self.period > 0 {
            return self.volume;
        } else {
            return self.starting_volume;
        }
    }

    pub fn trigger(&mut self) {
        self.volume = self.starting_volume;
        self.finished = false;

        self.timer = if self.period != 0 {
            self.period as i32
        } else {
            8
        };
    }

    pub fn step(&mut self) {
        if self.finished {
            return;
        }

        if self.timer <= 0 {
            self.timer = if self.period != 0 {
                self.period as i32
            } else {
                8
            };

            if self.add_mode && self.volume < 15 {
                self.volume += 1;
            } else if !self.add_mode && self.volume > 0 {
                self.volume -= 1;
            }

            if self.volume == 0 || self.volume == 15 {
                self.finished = true;
            }
        }
    }
}
