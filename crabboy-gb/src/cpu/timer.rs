use log::{debug, warn};

use crate::constants::{DIV, TAC, TIMA, TMA};
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Clock {
    pub period: u32,
    pub n: u32,
}

impl Clock {
    pub fn power_up(period: u32) -> Self {
        Self { period, n: 0x00 }
    }

    pub fn next(&mut self, cycles: u32) -> u32 {
        self.n += cycles;
        let res = self.n / self.period;
        self.n %= self.period;
        res
    }
}

/// Gameboy Timer
#[derive(Debug, Deserialize, Serialize)]
pub struct Timer {
    /// Divider Register - Incremented at rate of 16384Hz, Writing any vlaue to this register
    /// resets it to 0x00
    div: u8,

    /// Internal Timer Counter(R/W) - Incremented by clock frequency specified by the TAC register
    /// When the value overflows then it will be reset to value specified in TMA and interrupt
    /// will be request
    tima: u8,

    /// Timer Modulo (R/W)
    tma: u8,

    /// Timer Control (R/W)
    tac: u8,

    counter: u64,

    pub div_clock: Clock,
    pub tima_clock: Clock,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            counter: 0,
            div_clock: Clock::power_up(256),
            tima_clock: Clock::power_up(1024),
        }
    }

    pub fn set_div(&mut self, value: u8) {
        self.div = value;
    }
    pub fn div(&self) -> u8 {
        self.div
    }

    pub fn set_tima(&mut self, value: u8) {
        self.tima = value;
    }

    pub fn tima(&self) -> u8 {
        self.tima
    }

    pub fn set_tma(&mut self, value: u8) {
        self.tma = value;
    }

    pub fn tma(&self) -> u8 {
        self.tma
    }

    pub fn set_tac(&mut self, value: u8) {
        self.tac = value;
    }

    pub fn tac(&self) -> u8 {
        self.tac
    }

    pub fn set_counter(&mut self, value: u64) {
        self.counter = value;
    }

    pub fn counter(&self) -> u64 {
        self.counter
    }

    pub fn div_clock(&self) -> Clock {
        self.div_clock
    }

    pub fn tima_clock(&self) -> Clock {
        self.tima_clock
    }

    pub fn log_timer(&self) {
        debug!(
            "DIV: {} TIMA: {} TMA: {} TAC: {}",
            self.div, self.tima, self.tma, self.tac
        );
    }

    pub fn timer_read(&self, addr: u16) -> u8 {
        match addr {
            DIV => self.div,
            TIMA => self.tima,
            TMA => self.tma,
            TAC => self.tac,
            _ => panic!("{}:  NOT A READABLE TIMER ADRESS", addr),
        }
    }

    pub fn timer_write(&mut self, addr: u16, value: u8) {
        match addr {
            DIV => {
                self.div = 0x00;
                self.div_clock.n = 0x00;
                self.tima = 0x00;
            }
            TIMA => self.tima = value,
            TMA => self.tma = value,
            TAC => {
                let clocked_enabled: bool = (self.tac & 0x03) != (value & 0x03);
                if clocked_enabled {
                    self.tima_clock.n = 0x00;
                    let clock_select = value & 0x03;
                    self.tima_clock.period = match clock_select {
                        0x00 => 1024,
                        0x01 => 16,
                        0x02 => 64,
                        0x03 => 256,
                        _ => panic!(""),
                    };
                    self.tima = self.tma;
                }
                self.tac = value;
            }

            _ => warn!("{}: NOT A WRITABLE TIMER ADDRESS", addr),
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
