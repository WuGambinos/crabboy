mod constants;
mod gui;
mod sdl_support;

use glow::HasContext;
use imgui::Context;
use imgui_glow_renderer::AutoRenderer;
use sdl2::{
    event::Event,
    video::{GLProfile, Window},
};
use sdl_support::SdlPlatform;

use crabboy::gameboy::*;
use crabboy::interconnect::joypad::Key;
use env_logger::*;

// Create a new glow context.
fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

fn keycode_to_key(keycode: sdl2::keyboard::Keycode) -> Option<Key> {
    match keycode {
        sdl2::keyboard::Keycode::Right | sdl2::keyboard::Keycode::D => Some(Key::Right),
        sdl2::keyboard::Keycode::Left | sdl2::keyboard::Keycode::A => Some(Key::Left),
        sdl2::keyboard::Keycode::Up | sdl2::keyboard::Keycode::W => Some(Key::Up),
        sdl2::keyboard::Keycode::Down | sdl2::keyboard::Keycode::S => Some(Key::Down),
        sdl2::keyboard::Keycode::Z => Some(Key::A),
        sdl2::keyboard::Keycode::X => Some(Key::B),
        sdl2::keyboard::Keycode::Space => Some(Key::Select),
        sdl2::keyboard::Keycode::Return => Some(Key::Start),
        _ => None,
    }
}

use sdl2::timer::Timer;
use std::time::{Duration, Instant};

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

const TARGET_FPS: u64 = 60;
const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / TARGET_FPS);

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);

    let window = video_subsystem
        .window("CrabBoy", WINDOW_WIDTH, WINDOW_HEIGHT)
        .allow_highdpi()
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();

    /* enable vsync to cap framerate */
    //window.subsystem().gl_set_swap_interval(1).unwrap();

    let gl: glow::Context = glow_context(&window);

    let mut imgui = Context::create();

    // disable creation of files on disc
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    let mut platform = SdlPlatform::init(&mut imgui);
    let mut renderer = AutoRenderer::initialize(gl, &mut imgui).expect("failed to create renderer");

    // start main loop
    let mut event_pump = sdl.event_pump().unwrap();

    let mut logger = Builder::from_default_env();
    logger.target(Target::Stdout);
    logger.init();

    // file dialog
    let path = std::env::current_dir().unwrap();
    let file_picker: rfd::FileDialog = rfd::FileDialog::new()
        .add_filter("gameboy", &["gb"])
        .add_filter("gameboy saves", &["sav"])
        .set_directory(&path);

    let mut gameboy = GameBoy::new();

    let mut last_frame = Instant::now();
    'main: loop {
        for event in event_pump.poll_iter() {
            /* pass all events to imgui platfrom */
            platform.handle_event(&mut imgui, &event);

            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = keycode.and_then(keycode_to_key) {
                        gameboy.interconnect.key_up(key)
                    }
                }

                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode.and_then(keycode_to_key) {
                        gameboy.interconnect.key_down(key)
                    }
                }

                _ => {}
            }

            if let Event::Quit { .. } = event {
                break 'main;
            }
        }

        /* call prepare_frame before calling imgui.new_frame() */
        platform.prepare_frame(&mut imgui, &window, &event_pump);

        let ui = imgui.new_frame();
        gui::menu(ui, &file_picker, &mut gameboy);
        gui::display_info(ui, &gameboy);
        gui::draw_tiles(ui, &gameboy.interconnect);
        gui::display_emulator(ui, &gameboy);
        gui::debug_window(ui, &gameboy);

        if gameboy.booted {
            gameboy.cpu.run(&mut gameboy.interconnect);
        }

        /* render */
        let draw_data = imgui.render();

        unsafe {renderer.gl_context().clear_color(0.14, 0.15, 0.16, 1.0);}

        unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        renderer.render(draw_data).unwrap();

        window.gl_swap_window();

        let elapsed = last_frame.elapsed();
        if elapsed < FRAME_DURATION {
            std::thread::sleep(FRAME_DURATION - elapsed);
        }
        last_frame = Instant::now();
    }
}
