use modular_bitfield::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug)]
pub enum Key {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Start,
    Select,
}

pub fn key_to_u8(key: &Key) -> u8 {
    match key {
        Key::Right | Key::A => 0b0001,
        Key::Left | Key::B => 0b0010,
        Key::Up | Key::Select => 0b0100,
        Key::Down | Key::Start => 0b1000,
    }
}

#[bitfield]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Joypad {
    buttons: B4,
    directions: B4,
    select_direction: B1,
    select_action: B1,
    empty: B6,
}

impl Joypad {
    pub fn init() -> Joypad {
        let mut pad = Joypad::new();
        pad.set_buttons(0xF);
        pad.set_directions(0xF);
        pad.set_select_direction(0x1);
        pad.set_select_action(0x1);

        pad
    }

    pub fn key_down(&mut self, key: Key) {
        match key {
            Key::A | Key::B | Key::Start | Key::Select => {
                let value = self.buttons() & !key_to_u8(&key);
                self.set_buttons(value);
            }
            Key::Right | Key::Left | Key::Up | Key::Down => {
                let value = self.directions() & !key_to_u8(&key);
                self.set_directions(value);
            }
        }
    }

    pub fn key_up(&mut self, key: Key) {
        match key {
            Key::A | Key::B | Key::Start | Key::Select => {
                let value = self.buttons() | key_to_u8(&key);
                self.set_buttons(value);
            }
            Key::Right | Key::Left | Key::Up | Key::Down => {
                let value = self.directions() | key_to_u8(&key);
                self.set_directions(value);
            }
        }
    }

    pub fn read(&self) -> u8 {
        if self.select_direction() == 0 {
            return self.directions();
        }

        if self.select_action() == 0 {
            return self.buttons();
        }

        0xFF
    }

    pub fn write(&mut self, value: u8) {
        let select = (value & 0b0011_0000) >> 4;
        let buttons = (!(select & 0b01)) & 1;
        let directions = (!((select & 0b10) >> 1)) & 1;
        self.set_select_action(buttons);
        self.set_select_direction(directions);
    }
}
