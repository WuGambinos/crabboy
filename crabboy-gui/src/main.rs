//! Example showing the same functionality as
//! `imgui-examples/examples/custom_textures.rs`
//!
//! Not that the texture uses the internal format `glow::SRGB`, so that
//! OpenGL automatically converts colors to linear space before the shaders.
//! The renderer assumes you set this internal format correctly like this.

use std::{io::Cursor, num::NonZeroU32, time::Instant};

use glow::HasContext;
use glutin::surface::GlSurface;
use imgui::{Condition, DrawListMut, ImColor32, Ui};

use imgui_glow_renderer::Renderer;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::keyboard::{Key, KeyCode, PhysicalKey};

use crabboy::gameboy::*;
use crabboy::interconnect::*;

use crabboy::constants::{TILE_COLORS, X_RESOLUTION, Y_RESOLUTION};
#[allow(dead_code)]
mod utils;

mod constants;
mod gui;

fn keycode_to_key(keycode: KeyCode) -> Option<joypad::Key> {
    match keycode {
        KeyCode::ArrowRight | KeyCode::KeyD => Some(joypad::Key::Right),
        KeyCode::ArrowLeft | KeyCode::KeyA => Some(joypad::Key::Left),
        KeyCode::ArrowUp | KeyCode::KeyW => Some(joypad::Key::Up),
        KeyCode::ArrowDown | KeyCode::KeyS => Some(joypad::Key::Down),
        KeyCode::KeyZ => Some(joypad::Key::A),
        KeyCode::KeyX => Some(joypad::Key::B),
        KeyCode::Space => Some(joypad::Key::Select),
        KeyCode::Enter => Some(joypad::Key::Start),
        _ => None,
    }
}

fn main() {
    let (event_loop, window, surface, context) = utils::create_window("Custom textures", None);
    let (mut winit_platform, mut imgui_context) = utils::imgui_init(&window);
    let gl = utils::glow_context(&context);
    // This time, we tell OpenGL this is an sRGB framebuffer and OpenGL will
    // do the conversion to sSGB space for us after the fragment shader.
    //unsafe { gl.enable(glow::FRAMEBUFFER_SRGB) };

    let mut textures = imgui::Textures::<glow::Texture>::default();
    // Note that `output_srgb` is `false`. This is because we set
    // `glow::FRAMEBUFFER_SRGB` so we don't have to manually do the conversion
    // in the shader.
    let mut ig_renderer = Renderer::new(&gl, &mut imgui_context, &mut textures, false)
        .expect("failed to create renderer");

    let path = std::env::current_dir().unwrap();
    let file_picker: rfd::FileDialog = rfd::FileDialog::new()
        .add_filter("gameboy", &["gb"])
        .add_filter("gameboy saves", &["sav"])
        .set_directory(&path);

    let mut gameboy = GameBoy::new();
    let mut textures_ui = TexturesUi::new(&gl, &mut gameboy, &mut textures);

    let mut last_frame = Instant::now();
    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| {
            // Note we can potentially make the loop more efficient by
            // changing the `Poll` (default) value to `ControlFlow::Wait`
            // but be careful to test on all target platforms!
            window_target.set_control_flow(ControlFlow::Poll);

            match event {
                winit::event::Event::WindowEvent {
                    event:
                        winit::event::WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    physical_key: actual_key,
                                    state: ElementState::Pressed,
                                    ..
                                },
                            ..
                        },
                    ..
                } => match actual_key {
                    PhysicalKey::Code(code) => {
                        if let Some(k) = keycode_to_key(code) {
                            gameboy.interconnect.joypad.key_down(k);
                        }
                    }
                    _ => (),
                },

                winit::event::Event::WindowEvent {
                    event:
                        winit::event::WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    physical_key: actual_key,
                                    state: ElementState::Released,
                                    ..
                                },
                            ..
                        },
                    ..
                } => match actual_key {
                    PhysicalKey::Code(code) => {
                        if let Some(k) = keycode_to_key(code) {
                            gameboy.interconnect.joypad.key_up(k);
                        }
                    }
                    _ => (),
                },

                winit::event::Event::NewEvents(_) => {
                    let now = Instant::now();
                    imgui_context
                        .io_mut()
                        .update_delta_time(now.duration_since(last_frame));
                    last_frame = now;
                }

                winit::event::Event::AboutToWait => {
                    if gameboy.booted {
                        gameboy.cpu.run(&mut gameboy.interconnect);
                    }
                    winit_platform
                        .prepare_frame(imgui_context.io_mut(), &window)
                        .unwrap();

                    window.request_redraw();
                }

                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::RedrawRequested,
                    ..
                } => {
                    unsafe { gl.clear(glow::COLOR_BUFFER_BIT) };

                    let ui = imgui_context.frame();
                    let framerate = ui.io().framerate;

                    textures_ui.generated_texture =
                        TexturesUi::generate(&gl, &mut gameboy, &mut textures);

                    textures_ui.show(ui);

                    ui.window("FPS")
                        .size(
                            [200.0, 200.0],
                            Condition::FirstUseEver,
                        )
                        .position([0.0, 0.0], Condition::FirstUseEver)
                        .collapsed(false, Condition::FirstUseEver)
                        .build(|| {
                            ui.text(format!("FPS:{}", framerate));
                        });

                    gui::menu(ui, &file_picker, &mut gameboy);
                    /*
                    gui::display_emulator(ui, &gameboy);
                    */

                    winit_platform.prepare_render(ui, &window);
                    let draw_data = imgui_context.render();
                    ig_renderer
                        .render(&gl, &textures, draw_data)
                        .expect("error rendering imgui");

                    surface
                        .swap_buffers(&context)
                        .expect("Failed to swap buffers");
                }

                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::Resized(new_size),
                    ..
                } => {
                    if new_size.width > 0 && new_size.height > 0 {
                        surface.resize(
                            &context,
                            NonZeroU32::new(new_size.width).unwrap(),
                            NonZeroU32::new(new_size.height).unwrap(),
                        );
                    }
                    winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
                }

                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    window_target.exit();
                }

                winit::event::Event::LoopExiting => {
                    ig_renderer.destroy(&gl);
                }

                event => {
                    winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
                }
            }
        })
        .expect("EventLoop error");
}

struct TexturesUi {
    generated_texture: imgui::TextureId,
}

impl TexturesUi {
    fn new(
        gl: &glow::Context,
        gameboy: &mut GameBoy,
        textures: &mut imgui::Textures<glow::Texture>,
    ) -> Self {
        Self {
            generated_texture: Self::generate(gl, gameboy, textures),
        }
    }

    /// Generate dummy texture
    fn generate(
        gl: &glow::Context,
        gameboy: &mut GameBoy,
        textures: &mut imgui::Textures<glow::Texture>,
    ) -> imgui::TextureId {
        let mut data =
            Vec::with_capacity((Y_RESOLUTION as usize * X_RESOLUTION as usize * 3).into());
        let video_buffer = gameboy.interconnect.ppu.video_buffer;

        for line_num in 0..Y_RESOLUTION {
            for x in 0..X_RESOLUTION {
                let index =
                    (u32::from(x) + (u32::from(line_num) * u32::from(X_RESOLUTION))) as usize;
                let color = video_buffer[index];
                let (r, g, b) = color.get_rgb();
                data.push(r);
                data.push(g);
                data.push(b);
            }
        }

        let gl_texture = unsafe { gl.create_texture() }.expect("unable to create GL texture");

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(gl_texture));
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as _,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as _,
            );
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGB as _, // When generating a texture like this, you're probably working in linear color space
                X_RESOLUTION as _,
                Y_RESOLUTION as _,
                0,
                glow::RGB,
                glow::UNSIGNED_BYTE,
                Some(&data),
            )
        }

        textures.insert(gl_texture)
    }

    fn show(&self, ui: &imgui::Ui) {
        ui.window("Hello textures")
            .size([400.0, 400.0], Condition::FirstUseEver)
            .build(|| {
                ui.text("Hello textures!");
                ui.text("Some generated texture");
                imgui::Image::new(self.generated_texture, [300.0, 300.0]).build(ui);

                ui.text("Say hello to Peppers");
            });
    }
}
