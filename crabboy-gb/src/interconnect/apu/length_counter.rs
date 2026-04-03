use crate::nth_bit;

#[derive(Clone, Copy)]
pub struct LengthCounter {
    enabled: bool,
    full_length: i32,
    length: i32,
    frame_sequencer: i32,
}

impl LengthCounter {
    pub fn new() -> LengthCounter {
        LengthCounter {
            enabled: false,
            full_length: 0,
            length: 0,
            frame_sequencer: 0,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn zero(&self) -> bool {
        self.length == 0
    }

    pub fn set_length(&mut self, value: u8) {
        self.length = value as i32;
    }

    pub fn length(&self) -> i32 {
        self.length
    }

    pub fn set_full_length(&mut self, value: i32) {
        self.full_length = value;
    }

    pub fn full_length(&self) -> i32 {
        self.full_length
    }

    pub fn set_frame_sequencer(&mut self, frame_sequencer: i32) {
        self.frame_sequencer = frame_sequencer;
    }

    pub fn frame_sequencer(&self) -> i32 {
        self.frame_sequencer
    }

    pub fn step(&mut self) {
        if self.enabled && self.length > 0 {
            self.length = self.length - 1;
        }
    }

    pub fn power_off(&mut self) {
        self.enabled = false;
        self.frame_sequencer = 0;
    }

    pub fn write_nr4(&mut self, value: u8) {
        let enable = nth_bit!(value, 6) != 0;
        let trigger = nth_bit!(value, 7) != 0;

        if self.enabled {
            if trigger && self.length == 0 {
                self.length = self.full_length - 1;
            } else {
                self.length = self.full_length;
            }
        } else if enable {
            if (self.frame_sequencer & 1) == 0 {
                if self.length != 0 {
                    self.length = self.length - 1;
                }

                if trigger && self.length == 0 {
                    self.length = self.full_length - 1;
                }
            }
        } else {
            if trigger && self.length == 0 {
                self.length = self.full_length;
            }
        }

        self.enabled = enable;
    }
}
