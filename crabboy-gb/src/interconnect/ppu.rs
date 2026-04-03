#![allow(clippy::must_use_candidate)]
use crate::{constants::*, cpu::interrupts::InterruptType};
use modular_bitfield::prelude::*;
use std::cmp::Ordering;
use wasm_bindgen::prelude::wasm_bindgen;

use serde::{Serialize, Deserialize};
use serde_big_array::BigArray;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb { r, g, b }
    }

    pub fn get_rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    pub fn to_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

#[derive(Clone, Copy)]
pub enum PaletteType {
    Background,
    Sprite0,
    Sprite1,
}

#[derive(Debug, Clone, Copy)]
pub enum LcdMode {
    HBlank,
    VBlank,
    Oam,
    Transfer,
}

#[bitfield]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SpriteFlags {
    palette_number_cgb: B3,
    tile_vram_bank: B1,
    palette_number: B1,
    x_flip: B1,
    y_flip: B1,
    bg_window: B1,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SpriteEntry {
    y: u8,
    x: u8,
    tile_index: u8,
    flags: SpriteFlags,
}

impl SpriteEntry {
    fn new() -> Self {
        Self {
            y: 0,
            x: 0,
            tile_index: 0,
            flags: SpriteFlags::new(),
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Dma {
    pub active: bool,
    pub byte: u8,
    pub value: u8,
    pub start_delay: u8,
}

impl Dma {
    fn new() -> Self {
        Self {
            active: false,
            byte: 0,
            start_delay: 0,
            value: 0,
        }
    }
}

#[bitfield]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Control {
    bg_window: B1,
    sprite_enable: B1,
    sprite_size: B1,
    bg_tile_map_area: B1,
    bg_window_tile_data_area: B1,
    window_enable: B1,
    window_tile_map_area: B1,
    lcd_ppu_enable: B1,
}

#[bitfield]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Status {
    mode: B2,
    lyc_ly_compare: B1,
    hblank_interrupt_soruce: B1,
    vblank_interrupt_source: B1,
    oam_interrupt_source: B1,
    lyc_ly_interrupt_source: B1,
    empty: B1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ppu {
    #[serde(with = "BigArray")]
    vram: [u8; 0x2000],

    #[serde(with = "BigArray")]
    oam: [SpriteEntry; 40],

    line_ticks: u32,

    pub dma: Dma,

    #[serde(with = "BigArray")]
    pub video_buffer: [Rgb; BUFFER_SIZE],

    status: Status,
    control: Control,

    // Viewport X position
    scroll_x: u8,

    // Viewport Y position
    scroll_y: u8,

    // LCD Y coordinate (read only)
    ly: u8,

    // LY compare
    lyc: u8,

    // Window X position (Top left)
    window_x: u8,

    // Window Y position (Top left)
    window_y: u8,

    // Background palette (Non-CGB mode only)
    pub bg_palette: [Rgb; 4],
    pub bg_palette_data: u8,

    // Sprite palettes data (Non-CGB mode only)
    pub sprite0_palette: [Rgb; 4],
    pub sprite0_palette_data: u8,

    pub sprite1_palette: [Rgb; 4],
    pub sprite1_palette_data: u8,

    #[serde(with = "BigArray")]
    pub bg_priority: [bool; X_RESOLUTION as usize],
}

impl Ppu {
    pub fn new() -> Self {
        let mut ppu = Self {
            vram: [0; 0x2000],
            oam: [SpriteEntry::new(); 40],
            dma: Dma::new(),
            line_ticks: 0,
            video_buffer: [Rgb::new(0, 0, 0); BUFFER_SIZE],

            status: Status::new(),
            control: Control::from_bytes([0x91]),

            scroll_x: 0,
            scroll_y: 0,
            ly: 0,
            lyc: 0,
            window_x: 0,
            window_y: 0,

            bg_palette: TILE_COLORS,
            bg_palette_data: 0,

            sprite0_palette: TILE_COLORS,
            sprite0_palette_data: 0,

            sprite1_palette: TILE_COLORS,
            sprite1_palette_data: 0,
            bg_priority: [false; X_RESOLUTION as usize],
        };

        ppu.set_stat_mode(LcdMode::Oam);
        ppu
    }

    pub fn dma_transferring(&self) -> bool {
        self.dma.active
    }

    pub fn set_dma_active(&mut self, value: bool) {
        self.dma.active = value;
    }

    pub fn dma_active(&self) -> bool {
        self.dma.active
    }

    pub fn set_dma_byte(&mut self, value: u8) {
        self.dma.byte = value;
    }

    pub fn dma_byte(&self) -> u8 {
        self.dma.byte
    }

    pub fn set_dma_start_delay(&mut self, value: u8) {
        self.dma.start_delay = value;
    }

    pub fn dma_start_delay(&self) -> u8 {
        self.dma.start_delay
    }

    pub fn set_dma_value(&mut self, value: u8) {
        self.dma.value = value;
    }

    pub fn dma_value(&self) -> u8 {
        self.dma.value
    }

    pub fn dma_start(&mut self, value: u8) {
        self.dma.active = true;
        self.dma.byte = 0;
        self.dma.start_delay = 2;
        self.dma.value = value;
    }

    pub fn set_line_ticks(&mut self, value: u32) {
        self.line_ticks = value;
    }

    pub fn line_ticks(&self) -> u32 {
        self.line_ticks
    }

    pub fn increment_line_ticks(&mut self) {
        self.line_ticks = self.line_ticks.wrapping_add(1);
    }

    pub fn set_ly(&mut self, value: u8) {
        self.ly = value;
    }

    pub fn ly(&self) -> u8 {
        self.ly
    }

    pub fn set_lyc(&mut self, value: u8) {
        self.lyc = value;
    }

    pub fn lyc(&self) -> u8 {
        self.lyc
    }

    pub fn set_scroll_x(&mut self, value: u8) {
        self.scroll_x = value;
    }

    pub fn scroll_x(&self) -> u8 {
        self.scroll_x
    }

    pub fn set_scroll_y(&mut self, value: u8) {
        self.scroll_y = value;
    }

    pub fn scroll_y(&self) -> u8 {
        self.scroll_y
    }

    pub fn set_window_x(&mut self, value: u8) {
        self.window_x = value;
    }

    pub fn window_x(&self) -> u8 {
        self.window_x
    }

    pub fn set_window_y(&mut self, value: u8) {
        self.window_y = value;
    }

    pub fn window_y(&self) -> u8 {
        self.window_y
    }

    pub fn stat(&self) -> Status {
        self.status
    }

    pub fn control(&self) -> Control {
        self.control
    }

    pub fn update_palette(&mut self, palette_type: &PaletteType, palette_data: u8) {
        let (palette_colors, pal_data) = match palette_type {
            PaletteType::Background => (&mut self.bg_palette, &mut self.bg_palette_data),
            PaletteType::Sprite0 => (&mut self.sprite0_palette, &mut self.sprite0_palette_data),
            PaletteType::Sprite1 => (&mut self.sprite1_palette, &mut self.sprite1_palette_data),
        };

        *pal_data = palette_data;

        palette_colors[0] = TILE_COLORS[(palette_data & 0b11) as usize];
        palette_colors[1] = TILE_COLORS[((palette_data >> 2) & 0b11) as usize];
        palette_colors[2] = TILE_COLORS[((palette_data >> 4) & 0b11) as usize];
        palette_colors[3] = TILE_COLORS[((palette_data >> 6) & 0b11) as usize];
    }

    pub fn write_oam(&mut self, addr: u16, value: u8) {
        if addr >= 0xFE00 {
            let index = ((addr - 0xFE00) / 4) as usize;
            let inner_index = ((addr - 0xFE00) % 4) as usize;
            match inner_index {
                0 => self.oam[index].y = value,
                1 => self.oam[index].x = value,
                2 => self.oam[index].tile_index = value,
                3 => self.oam[index].flags = SpriteFlags::from_bytes([value]),
                _ => panic!("NOT AN INDEX"),
            }
        } else {
            let index = (addr / 4) as usize;
            let inner_index = (addr % 4) as usize;
            match inner_index {
                0 => self.oam[index].y = value,
                1 => self.oam[index].x = value,
                2 => self.oam[index].tile_index = value,
                3 => self.oam[index].flags = SpriteFlags::from_bytes([value]),
                _ => panic!("NOT AN INDEX"),
            }
        }
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        let index = ((addr - 0xFE00) / 4) as usize;
        let inner_index = ((addr - 0xFE00) % 4) as usize;
        match inner_index {
            0 => self.oam[index].y,
            1 => self.oam[index].x,
            2 => self.oam[index].tile_index,
            3 => self.oam[index].flags.into_bytes()[0],
            _ => panic!("NOT AN INDEX"),
        }
    }

    pub fn write_vram(&mut self, addr: u16, value: u8) {
        self.vram[(addr - 0x8000) as usize] = value;
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        self.vram[(addr - 0x8000) as usize]
    }

    pub fn write_lcd(&mut self, addr: u16, value: u8) {
        let index: u8 = (addr - 0xFF40) as u8;

        match index {
            0x0 => self.control = Control::from_bytes([value]),
            0x1 => self.status = Status::from_bytes([value]),
            0x2 => self.scroll_y = value,
            0x3 => self.scroll_x = value,
            0x4 => self.ly = value,
            0x5 => self.lyc = value,
            0x6 => {
                log::info!("DMA START");
                self.dma_start(value);
            }
            0x7 => {
                self.update_palette(&PaletteType::Background, value);
            }
            0x8 => {
                self.update_palette(&PaletteType::Sprite0, value & 0b1111_1100);
            }
            0x9 => {
                self.update_palette(&PaletteType::Sprite1, value & 0b1111_1100);
            }
            0xA => self.window_y = value,
            0xB => self.window_x = value,
            _ => panic!("NOT AN INDEX"),
        }
    }

    pub fn read_lcd(&self, addr: u16) -> u8 {
        let index: u8 = (addr - 0xFF40) as u8;

        match index {
            0x0 => self.control.bytes[0],
            0x1 => self.status.bytes[0],
            0x2 => self.scroll_y,
            0x3 => self.scroll_x,
            0x4 => self.ly,
            0x5 => self.lyc,
            0x6 => {
                log::warn!("MAY BE A BUG HERE: DMA: {:?}", self.dma.value);
                self.dma.value
            }
            0x7 => self.bg_palette_data,
            0x8 => self.sprite0_palette_data,
            0x9 => self.sprite1_palette_data,
            0xA => self.window_y,
            0xB => self.window_x,
            _ => panic!("NOT AN INDEX"),
        }
    }

    pub fn bg_tile_map_addr(&self) -> u16 {
        if self.control().bg_tile_map_area() == 1 {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn bg_window_data_area(&self) -> u16 {
        if self.control().bg_window_tile_data_area() == 1 {
            0x8000
        } else {
            0x8800
        }
    }

    pub fn window_map_area(&self) -> u16 {
        if self.control().window_tile_map_area() == 1 {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn stat_mode(&self) -> LcdMode {
        let bits = self.stat().bytes[0] & 0b11;

        match bits {
            0 => LcdMode::HBlank,
            1 => LcdMode::VBlank,
            2 => LcdMode::Oam,
            3 => LcdMode::Transfer,
            _ => panic!("NOT AN LCD MODE"),
        }
    }

    pub fn set_stat_mode(&mut self, mode: LcdMode) {
        let bits = self.status.bytes[0] & !0b11;
        let value = bits | mode as u8;
        self.status = Status::from_bytes([value]);
    }

    pub fn increment_ly(&mut self) -> Vec<InterruptType> {
        let value = self.ly().wrapping_add(1);
        let mut vec: Vec<InterruptType> = Vec::new();
        self.set_ly(value);

        if self.ly() == self.lyc() {
            self.status.set_lyc_ly_compare(1);

            if self.stat().lyc_ly_interrupt_source() == 1 {
                vec.push(InterruptType::LcdStat);
            }
        } else {
            self.status.set_lyc_ly_compare(0);
        }

        vec
    }

    // Search OAM for Sprites whose Y coordinate
    // overlaps this line
    //
    // Duration: 80 "dots"
    pub fn oam_mode(&mut self) {
        let oam_is_over = self.line_ticks() >= 80;
        if oam_is_over {
            self.set_stat_mode(LcdMode::Transfer);
        }
    }

    // Reading OAM and VRAM to generate picture
    //
    // Duration: 168-291 "dots", depends on sprite count
    //
    pub fn transfer_mode(&mut self, interrupts: &mut Vec<InterruptType>) {
        self.draw_line();
        self.set_stat_mode(LcdMode::HBlank);
        if self.stat().hblank_interrupt_soruce() == 1 {
            interrupts.push(InterruptType::LcdStat);
        }
    }

    // Duration: 4560 "dots" (10 scanlines)
    pub fn vblank_mode(&mut self, interrupts: &mut Vec<InterruptType>) {
        let end_of_scanline = self.line_ticks() >= TICKS_PER_LINE;
        if end_of_scanline {
            let mut ints = self.increment_ly();
            interrupts.append(&mut ints);

            let onto_next_screen = self.ly() >= LINES_PER_FRAME;
            if onto_next_screen {
                self.set_stat_mode(LcdMode::Oam);
                self.set_ly(0);
            }

            self.set_line_ticks(0);
        }
    }

    // Duration: 87-204 "dots"
    pub fn hblank_mode(&mut self, interrupts: &mut Vec<InterruptType>) {
        let end_of_scanline = self.line_ticks() >= TICKS_PER_LINE;
        if end_of_scanline {
            let mut ints = self.increment_ly();
            interrupts.append(&mut ints);

            if self.ly() >= Y_RESOLUTION {
                self.set_stat_mode(LcdMode::VBlank);
                interrupts.push(InterruptType::VBlank);

                if self.stat().vblank_interrupt_source() == 1 {
                    interrupts.push(InterruptType::VBlank);
                }
            } else {
                self.set_stat_mode(LcdMode::Oam);
            }

            self.set_line_ticks(0);
        }
    }

    pub fn tick(&mut self) -> Vec<InterruptType> {
        self.increment_line_ticks();

        let mut interrupts: Vec<InterruptType> = Vec::new();

        match self.stat_mode() {
            LcdMode::Oam => self.oam_mode(),
            LcdMode::Transfer => self.transfer_mode(&mut interrupts),
            LcdMode::VBlank => self.vblank_mode(&mut interrupts),
            LcdMode::HBlank => self.hblank_mode(&mut interrupts),
        };

        interrupts
    }

    pub fn draw_line(&mut self) {
        let slice_start = (X_RESOLUTION as usize) * (self.ly() as usize);
        let slice_end = (X_RESOLUTION as usize) + slice_start;

        let background_on = self.control().bg_window() == 1;
        if background_on {
            let map_y: u8 = self.ly().wrapping_add(self.scroll_y());
            let row: u8 = map_y / 8;

            for x_pos in 0..X_RESOLUTION {
                let map_x: u8 = x_pos.wrapping_add(self.scroll_x());
                let col: u8 = map_x / 8;
                let tile_number: u8 = (map_y % 8) * 2;

                let tile_data_addr: u16 =
                    self.bg_tile_map_addr() + (u16::from(col)) + (u16::from(row) * 32);
                let mut tile_data: u8 = self.vram[(tile_data_addr - 0x8000) as usize];
                if self.bg_window_data_area() == 0x8800 {
                    tile_data = tile_data.wrapping_add(128);
                }

                let hi_addr: u16 =
                    self.bg_window_data_area() + tile_data as u16 * 16 + tile_number as u16;
                let low_addr: u16 =
                    self.bg_window_data_area() + tile_data as u16 * 16 + tile_number as u16 + 1;
                let hi: u8 = self.vram[(hi_addr - 0x8000) as usize];
                let low: u8 = self.vram[(low_addr - 0x8000) as usize];

                let bit = (map_x % 8).wrapping_sub(7).wrapping_mul(0xFF) as usize;
                let hi_bit = (hi >> bit) & 1;
                let low_bit = ((low >> bit) & 1) << 1;
                let color_value = (hi_bit | low_bit) as usize;
                let color = self.bg_palette[color_value];
                self.bg_priority[x_pos as usize] = color_value != 0;

                let pixels = &mut self.video_buffer[slice_start..slice_end];
                pixels[x_pos as usize] = color;
            }
        }

        let window_enabled: bool =
            self.control().window_enable() == 1 && self.window_y() <= self.ly();
        if window_enabled {
            let window_x = self.window_x().wrapping_sub(7);

            let map_y = self.ly() - self.window_y();
            let row = map_y / 8;

            for i in (window_x as usize)..X_RESOLUTION as usize {
                let mut map_x = (i as u8).wrapping_add(self.scroll_x());

                if map_x >= window_x {
                    map_x = i as u8 - window_x;
                }
                let col = map_x / 8;

                let tile_data_addr: u16 =
                    self.window_map_area() + (u16::from(col)) + (u16::from(row) * 32);
                let mut tile_data: u8 = self.vram[(tile_data_addr - 0x8000) as usize];
                if self.bg_window_data_area() == 0x8800 {
                    tile_data = tile_data.wrapping_add(128);
                }

                let tile_number = (map_y % 8) * 2;

                let hi_addr: u16 =
                    self.bg_window_data_area() + tile_data as u16 * 16 + tile_number as u16;
                let low_addr: u16 =
                    self.bg_window_data_area() + tile_data as u16 * 16 + tile_number as u16 + 1;

                let hi = self.vram[(hi_addr - 0x8000) as usize];
                let low = self.vram[(low_addr - 0x8000) as usize];

                let bit = (map_x % 8).wrapping_sub(7).wrapping_mul(0xFF) as usize;
                let hi_bit = (hi >> bit) & 1;
                let low_bit = ((low >> bit) & 1) << 1;
                let color_value = (hi_bit | low_bit) as usize;
                let color = self.bg_palette[color_value];
                self.bg_priority[i] = color_value != 0;

                let pixels = &mut self.video_buffer[slice_start..slice_end];
                pixels[i] = color;
            }
        }

        if self.control().sprite_enable() == 1 {
            let sprite_size = if self.control().sprite_size() == 1 {
                16
            } else {
                8
            };

            let current_line = self.ly();

            let mut sprites_to_draw: Vec<(usize, SpriteEntry)> = Vec::with_capacity(10);

            for (index, sprite) in self.oam.iter().enumerate() {
                let y = sprite.y.wrapping_sub(16);
                let x = sprite.x.wrapping_sub(8);
                let flags = sprite.flags;

                if current_line.wrapping_sub(y) < sprite_size {
                    let res = SpriteEntry {
                        y,
                        x,
                        tile_index: sprite.tile_index,
                        flags,
                    };
                    if sprites_to_draw.len() < 10 {
                        sprites_to_draw.push((index, res));
                    }
                }
            }

            sprites_to_draw.sort_by(|&(a_index, a), &(b_index, b)| {
                match a.x.cmp(&b.x) {
                    // If X coordinates are the same, use OAM index as priority (low index => draw last)
                    Ordering::Equal => a_index.cmp(&b_index).reverse(),
                    // Use X coordinate as priority (low X => draw last)
                    other => other.reverse(),
                }
            });

            for (_, sprite) in sprites_to_draw {
                let palette = if sprite.flags.palette_number() == 1 {
                    &self.sprite1_palette
                } else {
                    &self.sprite0_palette
                };

                let mut tile_num = sprite.tile_index as usize;
                if sprite_size == 16 {
                    tile_num &= 0xFE;
                } else {
                    tile_num &= 0xFF;
                }

                let mut line = if sprite.flags.y_flip() == 1 {
                    sprite_size - current_line.wrapping_sub(sprite.y) - 1
                } else {
                    current_line.wrapping_sub(sprite.y)
                };

                if line >= 8 {
                    tile_num += 1;
                    line -= 8;
                }
                line *= 2;

                let hi_addr: u16 = 0x8000 + tile_num as u16 * 16 + (line) as u16;
                let low_addr: u16 = 0x8000 + tile_num as u16 * 16 + (line) as u16 + 1;
                let hi = self.vram[(hi_addr - 0x8000) as usize];
                let low = self.vram[(low_addr - 0x8000) as usize];

                for x in (0..8).rev() {
                    let bit = if sprite.flags.x_flip() == 1 { 7 - x } else { x } as usize;

                    let hi_bit = (hi >> bit) & 1;
                    let low_bit = ((low >> bit) & 1) << 1;
                    let color_value = (hi_bit | low_bit) as usize;
                    let color = palette[color_value];
                    let target_x = sprite.x.wrapping_add(7 - x);
                    if target_x < X_RESOLUTION && color_value != 0 {
                        if sprite.flags.bg_window() == 0 || !self.bg_priority[target_x as usize] {
                            let pixels = &mut self.video_buffer[slice_start..slice_end];
                            pixels[target_x as usize] = color;
                        }
                    }
                }
            }
        }
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}
