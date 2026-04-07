use ::dioxus::logger::tracing::{info, Level};
use ::dioxus::prelude::*;
use crabboy::constants::BUFFER_SIZE;
use crabboy::interconnect::ppu::Rgb;
use crabboy::{
    gameboy::GameBoy,
    interconnect::{
        cartridge::cartridge_info::{ram_size, u8_to_cart_type},
        cartridge::Cartridge,
    },
};

static CSS: Asset = asset!("/assets/main.css");
static GB: GlobalSignal<WebGameBoy> = Signal::global(|| WebGameBoy::new());

#[derive(Clone, PartialEq)]
struct Buffers {
    prev_buffer: [Rgb; BUFFER_SIZE],
    current_buffer: [Rgb; BUFFER_SIZE],
}

impl Buffers {
    fn new() -> Buffers {
        Buffers {
            prev_buffer: [Rgb::new(0, 0, 0); BUFFER_SIZE],
            current_buffer: [Rgb::new(0, 0, 0); BUFFER_SIZE],
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct WebGameBoy {
    booted: bool,
    gb: GameBoy,
    prev_buffer: [Rgb; BUFFER_SIZE],
    current_buffer: [Rgb; BUFFER_SIZE],
}

impl WebGameBoy {
    pub fn new() -> WebGameBoy {
        WebGameBoy {
            booted: false,
            gb: GameBoy::new(),
            prev_buffer: [Rgb::new(0, 0, 0); BUFFER_SIZE],
            current_buffer: [Rgb::new(0, 0, 0); BUFFER_SIZE],
        }
    }

    pub fn run(&mut self) {
        /*
        let interconnect = &mut self.gb.interconnect;
        self.gb.cpu.run(interconnect);
        */
    }

    pub fn boot(&mut self, rom: &[u8]) {
        info!("BOOTING");
        self.booted = true;
        let game_rom = rom.to_vec();

        let cart_type_value = game_rom[0x147];
        let rom_size_value = game_rom[0x148];
        let ram_size_value = game_rom[0x149];
        let cart_type = u8_to_cart_type(cart_type_value);
        let ram = vec![0x00; ram_size(ram_size_value) as usize];

        self.gb.interconnect.cartridge = Cartridge::new(&game_rom, &ram, &cart_type);
        self.gb.cpu.pc = 0x100;
    }
}

fn main() {
    dioxus::logger::init(Level::INFO).expect("failed to init logger");
    ::dioxus::launch(App);
}

#[component]
pub fn FilePick() -> Element {
    rsx! {
        input {
            r#type: "file",
            accept: ".gb, .gbc",
            multiple: false,
            onchange: move |evt| {
                async move {
                    for file in evt.files() {
                        if let Ok(bytes) = file.read_bytes().await {
                            info!("BOOTING ROM");
                            GB.write().boot(bytes.iter().as_slice());
                        }
                    }
                }
            },
        
        }
    }
}

fn Start() {
    info!("STARTING: PC: {:#X}", GB.read().gb.cpu.pc);
    GB.write().run();
}

extern crate console_error_panic_hook;
use std::panic;

#[allow(non_snake_case)]
#[component]
pub fn App() -> Element {

    panic::set_hook(Box::new(console_error_panic_hook::hook));
    rsx! {
        document::Stylesheet { href: CSS }

        body {
            button {
                id: "load",
                onclick: move |event| {

                    info!("Clicked Load ROM");
                    Start();
                },
                "Load Rom"
            
            }
            h1 { "YO" }
            h1 { "CrabBoy" }
            p {
                "Booted: {GB.read().gb.booted}"
                "PC: {GB.read().gb.cpu.pc}"
            }
            div { class: "flex-container",
                div { class: "flex-child",
                    canvas {
                        id: "canvas",
                        width: "640",
                        height: "600",
                        style: "border:1px solid",
                    }
                }

                div { class: "flex-child",
                    p { "Hello World" }

                    div {
                        FilePick {}

                        button { id: "reset", "Reset" }
                    }
                    div { id: "fps" }
                
                }
            }
        }
    }
}

/*
}
*/

/*
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
*/
