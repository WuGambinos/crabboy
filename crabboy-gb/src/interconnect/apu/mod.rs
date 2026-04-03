use crate::nth_bit;

use self::{noise::Noise, square1::Square1, square2::Square2, wave::Wave};

mod envelope;
mod length_counter;
mod noise;
mod square1;
mod square2;
mod sweep;
mod wave;

pub const DUTY_CYCLES: [[bool; 8]; 4] = [
    [false, false, false, false, false, false, false, true],
    [true, false, false, false, false, false, false, true],
    [true, false, false, false, false, true, true, true],
    [false, true, true, true, true, true, true, false],
];

pub trait Channel {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
    fn length_clock(&mut self);
    fn enabled(&self) -> bool;
}

#[derive(Clone, Copy)]
enum FrameSequencer {
    Step0,
    Step1,
    Step2,
    Step3,
    Step4,
    Step5,
    Step6,
    Step7,
}

fn int_to_frame_sequencer(fs: i32) -> FrameSequencer {
    match fs {
        0 => FrameSequencer::Step0,
        1 => FrameSequencer::Step1,
        2 => FrameSequencer::Step2,
        3 => FrameSequencer::Step3,
        4 => FrameSequencer::Step4,
        5 => FrameSequencer::Step5,
        6 => FrameSequencer::Step6,
        7 => FrameSequencer::Step7,
        _ => FrameSequencer::Step0,
    }
}

fn frame_sequencer_to_int(fs: FrameSequencer) -> i32 {
    match fs {
        FrameSequencer::Step0 => 0,
        FrameSequencer::Step1 => 1,
        FrameSequencer::Step2 => 2,
        FrameSequencer::Step3 => 3,
        FrameSequencer::Step4 => 4,
        FrameSequencer::Step5 => 5,
        FrameSequencer::Step6 => 6,
        FrameSequencer::Step7 => 7,
    }
}

pub struct Apu {
    channel_1: Square1,
    channel_2: Square2,
    channel_3: Wave,
    channel_4: Noise,
    enabled: bool,
    vin_left_enable: bool,
    vin_right_enable: bool,
    left_volume: u8,
    right_volume: u8,
    volume: [f32; 4],
    left_enables: [bool; 4],
    right_enables: [bool; 4],

    frequency_counter: i32,
    frame_sequencer_counter: i32,
    frame_sequencer: FrameSequencer,
}

impl Apu {
    fn new() -> Apu {
        Apu {
            channel_1: Square1::new(),
            channel_2: Square2::new(),
            channel_3: Wave::new(),
            channel_4: Noise::new(),
            enabled: false,
            vin_left_enable: false,
            vin_right_enable: false,
            left_volume: 0,
            right_volume: 0,
            volume: [0.; 4],
            left_enables: [false; 4],
            right_enables: [false; 4],
            frequency_counter: 0,
            frame_sequencer_counter: 0,
            frame_sequencer: FrameSequencer::Step0,
        }
    }

    fn clear_regs(&mut self) {
        self.vin_left_enable = false;
        self.vin_right_enable = false;
        self.left_volume = 0;
        self.right_volume = 0;

        self.enabled = false;

        self.channel_1.power_off();
        self.channel_2.power_off();
        self.channel_3.power_off();
        self.channel_4.power_off();

        for i in 0..4 {
            self.left_enables[i] = false;
            self.right_enables[i] = false;
        }
    }

    fn tick(&mut self) {
        if self.frame_sequencer_counter <= 0 {
            self.frame_sequencer_counter = 8192;

            match self.frame_sequencer {
                FrameSequencer::Step0 => {
                    self.channel_1.length_clock();
                    self.channel_2.length_clock();
                    self.channel_3.length_clock();
                    self.channel_4.length_clock();
                }
                FrameSequencer::Step2 => {
                    self.channel_1.sweep_clock();
                    self.channel_1.length_clock();
                    self.channel_2.length_clock();
                    self.channel_3.length_clock();
                    self.channel_4.length_clock();
                }
                FrameSequencer::Step4 => {
                    self.channel_1.length_clock();
                    self.channel_2.length_clock();
                    self.channel_3.length_clock();
                    self.channel_4.length_clock();
                }
                FrameSequencer::Step6 => {
                    self.channel_1.sweep_clock();
                    self.channel_1.length_clock();
                    self.channel_2.length_clock();
                    self.channel_3.length_clock();
                    self.channel_4.length_clock();
                }
                FrameSequencer::Step7 => {
                    self.channel_1.envelope_clock();
                    self.channel_2.envelope_clock();
                    self.channel_4.envelope_clock();
                }

                _ => (),
            }

            let new_frame_sequencer = (self.frame_sequencer as i32 + 1) & 7;
            self.frame_sequencer = int_to_frame_sequencer(new_frame_sequencer);

            self.channel_1
                .length_counter
                .set_frame_sequencer(new_frame_sequencer);
            self.channel_2
                .length_counter
                .set_frame_sequencer(new_frame_sequencer);
            self.channel_3
                .length_counter
                .set_frame_sequencer(new_frame_sequencer);
            self.channel_4
                .length_counter
                .set_frame_sequencer(new_frame_sequencer);
        }

        self.channel_1.tick();
        self.channel_2.tick();
        self.channel_3.tick();
        self.channel_4.tick();

        if self.frequency_counter <= 0 {
            self.frequency_counter = 95;

            let mut left = 0;
            let mut right = 0;

            let mut output = 0;

            for i in 0..4 {
                output = match i {
                    0 => self.channel_1.output * self.volume[i] as u8,
                    1 => self.channel_2.output * self.volume[i] as u8,
                    2 => self.channel_3.output * self.volume[i] as u8,
                    3 => self.channel_4.output * self.volume[i] as u8,
                    _ => 0,
                };
                if self.left_enables[i] {
                    left += output;
                }

                if self.right_enables[i] {
                    right += output
                }
            }
        }
    }

    fn read(&self, addr: u16) -> u8 {
        if (0xFF10..=0xFF14).contains(&addr) {
            return self.channel_1.read(addr);
        } else if (0xFF15..=0xFF19).contains(&addr) {
            return self.channel_2.read(addr);
        } else if (0xFF1A..=0xFF1E).contains(&addr) {
            return self.channel_3.read(addr);
        } else if (0xFF1F..=0xFF23).contains(&addr) {
            return self.channel_4.read(addr);
        }

        let mut result = 0;

        match addr {
            0xFF24 => {
                ((self.vin_left_enable as u8) << 7)
                    | (self.left_volume << 4)
                    | ((self.vin_right_enable as u8) << 3)
                    | self.right_volume
            }
            0xFF25 => {
                for i in 0..4 {
                    result |= (self.right_enables[i] as u8) << i;
                    result |= (self.left_enables[i] as u8) << i;
                }

                result
            }
            0xFF26 => {
                result = (self.enabled as u8) << 7;
                for i in 0..4 {
                    let channel: Box<dyn Channel> = match i {
                        0 => Box::new(self.channel_1),
                        1 => Box::new(self.channel_2),
                        2 => Box::new(self.channel_3),
                        3 => Box::new(self.channel_4),
                        _ => Box::new(self.channel_1),
                    };
                    result |= (channel.enabled() as u8) << i;
                }
                result | 0x70
            }
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        if addr == 0xFF26 {
            let enable = nth_bit!(value, 7) != 0;

            if self.enabled && !enable {
                self.clear_regs();
            } else if !self.enabled && enable {
                self.frame_sequencer = int_to_frame_sequencer(0);
            }

            self.enabled = enable;
            return;
        } else if (0xFF30..=0xFF3F).contains(&addr) {
            self.channel_3.write(addr, value);
            return;
        }

        if !self.enabled {
            match addr {
                0xFF11 => self.channel_1.write(addr, value & 0x3F),
                0xFF16 => self.channel_2.write(addr, value & 0x3F),
                0xFF1B => self.channel_3.write(addr, value & 0x3F),
                0xFF20 => self.channel_4.write(addr, value & 0x3F),
                _ => (),
            }

            return;
        }

        if (0xFF10..=0xFF14).contains(&addr) {
            self.channel_1.write(addr, value);
            return;
        } else if (0xFF15..=0xFF19).contains(&addr) {
            self.channel_2.write(addr, value);
            return;
        } else if (0xFF1A..=0xFF1E).contains(&addr) {
            self.channel_3.write(addr, value);
            return;
        } else if (0xFF1F..=0xFF23).contains(&addr) {
            self.channel_4.write(addr, value);
            return;
        }

        if (0xFF27..=0xFF2F).contains(&addr) {
            return;
        }

        match addr {
            0xFF24 => {
                self.right_volume = value & 0b111;
                self.vin_right_enable = nth_bit!(value, 3) != 0;
                self.left_volume = (value >> 4) & 0b111;
                self.vin_left_enable = nth_bit!(value, 7) != 0;
            }

            0xFF25 => {
                for i in 0..4 {
                    self.right_enables[i] = ((value >> i) & 1) != 0;
                    self.left_enables[i] = ((value >> (i + 4)) & 1) != 0;
                }
            }

            _ => (),
        }
    }
}
