use crate::chip8::cpu;
use crate::chip8::config;
use crate::chip8::keyboard;
use crate::chip8::execution;
use crate::chip8::cpu::CPUState;

extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;

pub fn execute_main_loop(state: &mut CPUState, config: &config::EmuConfig) -> Result<(), String>
{
    let scale = config.screen_scale as usize;
    let framebuffer_width = cpu::SCREEN_WIDTH * scale;
    let framebuffer_height = cpu::SCREEN_HEIGHT * scale;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut timer_subsystem = sdl_context.timer()?;

    let window = video_subsystem.window("CHIP-8 Emulator", framebuffer_width as u32, framebuffer_height as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let mut previous_time_ms: u32 = timer_subsystem.ticks();

    let mut event_pump = sdl_context.event_pump()?;

    'mainloop: loop {
        // Poll events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }

        let keyboard_state = event_pump.keyboard_state();

        // Get keyboard state
        keyboard::set_key_pressed(state, 0x1, keyboard_state.is_scancode_pressed(Scancode::Num1));
        keyboard::set_key_pressed(state, 0x2, keyboard_state.is_scancode_pressed(Scancode::Num2));
        keyboard::set_key_pressed(state, 0x3, keyboard_state.is_scancode_pressed(Scancode::Num3));
        keyboard::set_key_pressed(state, 0xC, keyboard_state.is_scancode_pressed(Scancode::Num4));
        keyboard::set_key_pressed(state, 0x4, keyboard_state.is_scancode_pressed(Scancode::Q));
        keyboard::set_key_pressed(state, 0x5, keyboard_state.is_scancode_pressed(Scancode::W));
        keyboard::set_key_pressed(state, 0x6, keyboard_state.is_scancode_pressed(Scancode::E));
        keyboard::set_key_pressed(state, 0xD, keyboard_state.is_scancode_pressed(Scancode::R));
        keyboard::set_key_pressed(state, 0x7, keyboard_state.is_scancode_pressed(Scancode::A));
        keyboard::set_key_pressed(state, 0x8, keyboard_state.is_scancode_pressed(Scancode::S));
        keyboard::set_key_pressed(state, 0x9, keyboard_state.is_scancode_pressed(Scancode::D));
        keyboard::set_key_pressed(state, 0xE, keyboard_state.is_scancode_pressed(Scancode::F));
        keyboard::set_key_pressed(state, 0xA, keyboard_state.is_scancode_pressed(Scancode::Z));
        keyboard::set_key_pressed(state, 0x0, keyboard_state.is_scancode_pressed(Scancode::X));
        keyboard::set_key_pressed(state, 0xB, keyboard_state.is_scancode_pressed(Scancode::C));
        keyboard::set_key_pressed(state, 0xF, keyboard_state.is_scancode_pressed(Scancode::V));

        let current_time_ms: u32 = timer_subsystem.ticks();
        let delta_time_ms: u32 = current_time_ms - previous_time_ms;

        execution::execute_step(state, delta_time_ms);

        // Draw
        let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::BGRA32, framebuffer_width as u32, framebuffer_height as u32)
            .map_err(|e| e.to_string())?;

        // Copy texture data
        texture.with_lock(None, |mapped_buffer: &mut [u8], mapped_buffer_pitch: usize| {
            let primary_color_bgra: [u8; 4] = [
                (255.0 * config.palette.primary.b) as u8,
                (255.0 * config.palette.primary.g) as u8,
                (255.0 * config.palette.primary.r) as u8,
                255
            ];
            let secondary_color_bgra: [u8; 4] = [
                (255.0 * config.palette.secondary.b) as u8,
                (255.0 * config.palette.secondary.g) as u8,
                (255.0 * config.palette.secondary.r) as u8,
                255
            ];

            let mut scanlines = mapped_buffer.chunks_mut(mapped_buffer_pitch);

            // Convert and upscale screen image
            for j in 0..cpu::SCREEN_HEIGHT {
                let current_scanline_slice = scanlines.next().unwrap();
                let mut scanline_pixels = current_scanline_slice.chunks_mut(4);

                for i in 0..cpu::SCREEN_LINE_SIZE_IN_BYTES {
                    let pixel_byte: u8 = state.screen[j][i];

                    for k in 0..8 {
                        let pixel_state = ((pixel_byte >> k) & 0x1) != 0;
                        let color_slice = if pixel_state { &primary_color_bgra[..] } else { &secondary_color_bgra[..] };

                        // Copy to upscale
                        for _ in 0..scale {
                            let dst_pixel = scanline_pixels.next().unwrap();
                            dst_pixel[..].clone_from_slice(color_slice);
                        }
                    }
                }

                // Copy scanlines
                for _ in 1..scale {
                    let dst_slice = scanlines.next().unwrap();
                    dst_slice[..].clone_from_slice(&current_scanline_slice[..]);
                }
            }
        })?;

        canvas.copy(&texture, None, None)?;
        canvas.present();

        previous_time_ms = current_time_ms;

        if config.debug_mode {
            println!("Frame time = {} ms", delta_time_ms);
        }
    }

    Ok(())
}
