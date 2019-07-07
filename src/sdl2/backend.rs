use crate::chip8::cpu;
use crate::chip8::config;
use crate::chip8::display;
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
    const PIXEL_FORMAT_BGRA_SIZE_IN_BYTES: usize = 4;

    let scale = config.screen_scale as usize;
    let width = cpu::SCREEN_WIDTH * scale;
    let height = cpu::SCREEN_HEIGHT * scale;
    let stride = width * PIXEL_FORMAT_BGRA_SIZE_IN_BYTES; // No extra space between lines
    let pitch = stride;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut timer_subsystem = sdl_context.timer()?;

    let window = video_subsystem.window("CHIP-8 Emulator", width as u32, height as u32)
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

        let framebuffer_width = cpu::SCREEN_WIDTH * scale;
        let framebuffer_height = cpu::SCREEN_HEIGHT * scale;

        // Draw
        let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::BGRA32, framebuffer_width as u32, framebuffer_height as u32)
            .map_err(|e| e.to_string())?;

        // Copy texture data
        texture.with_lock(None, |mapped_buffer: &mut [u8], mapped_buffer_pitch: usize| {
            assert_eq!(mapped_buffer_pitch, pitch);

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

            for j in 0..cpu::SCREEN_HEIGHT * scale {
                for i in 0..cpu::SCREEN_WIDTH * scale {
                    let pixel_index_flat_dst: usize = j * cpu::SCREEN_WIDTH * scale + i;
                    let pixel_output_offset_in_bytes: usize = pixel_index_flat_dst * PIXEL_FORMAT_BGRA_SIZE_IN_BYTES;
                    let pixel_value: u8 = display::read_screen_pixel(state, i / scale, j / scale);
                    let offset = pixel_output_offset_in_bytes;

                    if pixel_value != 0 {
                        mapped_buffer[offset..offset + 4].clone_from_slice(&primary_color_bgra[..]);
                    } else {
                        mapped_buffer[offset..offset + 4].clone_from_slice(&secondary_color_bgra[..]);
                    }
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
