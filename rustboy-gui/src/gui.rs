use crate::constants::{
    DEBUG_WINDOW_HEIGHT, DEBUG_WINDOW_WIDTH, DEBUG_WINDOW_X, DEBUG_WINDOW_Y, DISPLAY_INFO_HEIGHT,
    DISPLAY_INFO_WIDTH, DISPLAY_INFO_X, DISPLAY_INFO_Y, GB_POS, GB_SCREEN_SIZE, SCALE, TILE_SCALE,
    TILE_SCREEN_HEIGHT, TILE_SCREEN_WIDTH, TILE_SCREEN_X, TILE_SCREEN_Y,
};

use imgui::{Condition, DrawListMut, ImColor32, Ui};
use rfd::FileDialog;
use rustboy::constants::{TILE_COLORS, X_RESOLUTION, Y_RESOLUTION};
use rustboy::gameboy::GameBoy;
use rustboy::interconnect::Interconnect;

pub fn menu(ui: &mut Ui, picker: &FileDialog, gameboy: &mut GameBoy) {
    if let Some(main) = ui.begin_main_menu_bar() {
        let file_menu = ui.begin_menu("File");
        if let Some(f_menu) = file_menu {
            let select_rom = ui.menu_item("Open Rom");
            let save = ui.menu_item("Save State");
            let load = ui.menu_item("Load State");
            let exit = ui.menu_item("Exit");

            if select_rom {
                if !gameboy.booted {
                    let pick = picker.clone().pick_files();
                    match pick {
                        Some(v) => {
                            let rom_path = v[0].clone().into_os_string().into_string().unwrap();
                            gameboy.boot(&rom_path, true).unwrap();
                        }
                        None => println!("NO ROM SELECTED"),
                    }
                }
            }

            if load {
                let pick = picker.clone().pick_files().unwrap();
                let state_path = pick[0].clone().into_os_string().into_string().unwrap();
                let data: Vec<u8> = std::fs::read(state_path).unwrap();
                gameboy.load_state(data);
                gameboy.booted = true;
            }

            if save {
                let state = gameboy.get_state();
                let path_buf = picker.clone().set_file_name("save_file.sav").save_file();
                match path_buf {
                    Some(buf) => {
                        let path_string = buf.into_os_string().into_string();
                        match path_string {
                            Ok(s) => std::fs::write(format!("{}", s), &state).unwrap(),
                            Err(e) => println!("STRING ERROR: {:?}", e),
                        }
                    }
                    None => println!("NO SAVE FILE WAS MADE"),
                }
            }

            if exit {
                std::process::exit(0);
            }

            f_menu.end();
        }

        main.end();
    }
}

pub fn memory_viewer(ui: &mut Ui, gameboy: &GameBoy) {
    let rom_size = 0xFFFF;

    let mut row = String::new();
    for i in 0..rom_size {
        row.push_str(format!("{:X} ", gameboy.interconnect.read_mem(i)).as_str());
        if i % 16 == 0 && i != 0 {
            ui.text(row.clone());
            row.clear();
            row.push_str(format!("{:#X}0: ", i / 16).as_str());
        }
    }
}

pub fn debug_window(ui: &mut Ui, gameboy: &GameBoy) {
    ui.window("Debug Window")
        .position([DEBUG_WINDOW_X, DEBUG_WINDOW_Y], Condition::FirstUseEver)
        .size(
            [DEBUG_WINDOW_WIDTH, DEBUG_WINDOW_HEIGHT],
            Condition::FirstUseEver,
        )
        .collapsed(false, Condition::FirstUseEver)
        .build(|| {
            let pc = format!("PC: {:#X}", gameboy.cpu.pc);
            let sp = format!("SP: {:#X}", gameboy.cpu.sp);
            let opcode = format!("OPCODE: {:#X}", gameboy.cpu.opcode);
            let a = format!("A: {:#X}", gameboy.cpu.registers.a);
            let b = format!("B: {:#X}", gameboy.cpu.registers.b);
            let c = format!("C: {:#X}", gameboy.cpu.registers.c);
            let d = format!("D: {:#X}", gameboy.cpu.registers.d);
            let e = format!("E: {:#X}", gameboy.cpu.registers.e);
            let h = format!("H: {:#X}", gameboy.cpu.registers.h);
            let l = format!("L: {:#X}", gameboy.cpu.registers.l);
            let flags = format!("Flags: {:#X}", gameboy.cpu.registers.f.data);

            ui.text(pc);
            ui.text(sp);
            ui.text(opcode);
            ui.text(a);
            ui.text(b);
            ui.text(c);
            ui.text(d);
            ui.text(e);
            ui.text(h);
            ui.text(l);
            ui.text(flags);
        });
}

pub fn display_info(ui: &mut Ui, gameboy: &GameBoy) {
    ui.window("Info")
        .size(
            [DISPLAY_INFO_WIDTH, DISPLAY_INFO_HEIGHT],
            Condition::FirstUseEver,
        )
        .position([DISPLAY_INFO_X, DISPLAY_INFO_Y], Condition::FirstUseEver)
        .collapsed(false, Condition::FirstUseEver)
        .build(|| {
            let title = format!("TITLE: {}", &gameboy.interconnect.cartridge.title);
            let cart_type = format!(
                "CART TYPE: {:?}",
                &gameboy.interconnect.cartridge.cartridge_type
            );

            ui.text(title);
            ui.text(cart_type);
        });
}

pub fn display_emulator(ui: &mut Ui, gameboy: &GameBoy) {
    ui.window("Gameboy Emualtor")
        .size(GB_SCREEN_SIZE, Condition::FirstUseEver)
        .position(GB_POS, Condition::FirstUseEver)
        .scroll_bar(false)
        .build(|| {
            let draw_list = ui.get_window_draw_list();
            let origin: [f32; 2] = ui.cursor_screen_pos();
            let video_buffer = gameboy.interconnect.ppu.video_buffer;

            for line_num in 0..Y_RESOLUTION {
                for x in 0..X_RESOLUTION {
                    let new_x = (x as u16 * SCALE as u16) as f32;
                    let new_y = (line_num as u16 * SCALE as u16) as f32;

                    let width = SCALE as f32;
                    let height = SCALE as f32;

                    let index =
                        (u32::from(x) + (u32::from(line_num) * u32::from(X_RESOLUTION))) as usize;
                    let color = video_buffer[index];
                    let (r, g, b) = color.get_rgb();
                    let mut top_left = [new_x, new_y];
                    top_left[0] += origin[0];
                    top_left[1] += origin[1];
                    let bottom_right = [top_left[0] + width, top_left[1] + height];

                    let color = ImColor32::from_rgb(r, g, b);
                    draw_list
                        .add_rect(top_left, bottom_right, color)
                        .filled(true)
                        .build();
                }
            }
        });
}

pub fn draw_tiles(ui: &mut Ui, interconnect: &Interconnect) {
    ui.window("TILES")
        .size(
            [TILE_SCREEN_WIDTH as f32, TILE_SCREEN_HEIGHT as f32],
            Condition::FirstUseEver,
        )
        .position([TILE_SCREEN_X, TILE_SCREEN_Y], Condition::FirstUseEver)
        .collapsed(false, Condition::FirstUseEver)
        .build(|| {
            let draw_list = ui.get_window_draw_list();
            let origin: [f32; 2] = ui.cursor_screen_pos();
            let mut x_draw: i32 = 0;
            let mut y_draw: i32 = 0;
            let mut tile_num: u16 = 0;

            let top_left = origin;
            let bottom_right = [
                origin[0] + TILE_SCREEN_WIDTH as f32,
                origin[1] + TILE_SCREEN_HEIGHT as f32,
            ];
            let background = ImColor32::from_rgb(17, 17, 17);
            draw_list
                .add_rect(top_left, bottom_right, background)
                .filled(true)
                .build();

            let addr: u16 = 0x8000;
            for y in 0..24 {
                for x in 0..16 {
                    display_tile(
                        &draw_list,
                        origin,
                        interconnect,
                        addr,
                        tile_num,
                        x_draw + x * TILE_SCALE,
                        y_draw + y * TILE_SCALE,
                    );
                    x_draw += 8 * TILE_SCALE;
                    tile_num += 1;
                }
                y_draw += 8 * TILE_SCALE;
                x_draw = 0;
            }
        });
}

fn display_tile(
    draw_list: &DrawListMut,
    origin: [f32; 2],
    interconnect: &Interconnect,
    start_loc: u16,
    tile_num: u16,
    x: i32,
    y: i32,
) {
    for tile_y in (0..16).step_by(2) {
        let addr: u16 = start_loc + (tile_num * 16) + tile_y;

        let second_byte = interconnect.read_mem(addr);
        let first_byte = interconnect.read_mem(addr + 1);

        let mut color: u8;

        for bit in (0..8).rev() {
            let first_bit = (first_byte >> bit) & 1;
            let second_bit = (second_byte >> bit) & 1;

            if first_bit == 0 && second_bit == 0 {
                color = 0;
            } else if first_bit == 0 && second_bit == 1 {
                color = 1;
            } else if first_bit == 1 && second_bit == 0 {
                color = 2;
            } else {
                color = 3;
            }

            let new_x = (x + ((7 - bit) * TILE_SCALE)) as f32;
            let new_y = (y + ((tile_y as i32) / 2 * TILE_SCALE)) as f32;

            let width = TILE_SCALE as f32;
            let height = TILE_SCALE as f32;

            let (r, g, b) = TILE_COLORS[color as usize].get_rgb();
            let mut top_left = [new_x, new_y];
            top_left[0] += origin[0];
            top_left[1] += origin[1];
            let bottom_right = [top_left[0] + width, top_left[1] + height];
            let color = ImColor32::from_rgb(r, g, b);
            draw_list
                .add_rect(top_left, bottom_right, color)
                .filled(true)
                .build();
        }
    }
}
