use crate::chip8::cpu;
use crate::chip8::config;
use crate::chip8::display;
use crate::chip8::keyboard;
use crate::chip8::execution;
use crate::chip8::cpu::CPUState;

const PixelFormatBGRASizeInBytes: usize = 4;

fn fill_image_buffer(imageOutput: &mut Vec<u8>, state: &CPUState, palette: &config::Palette, scale: u32)
{
    let primary_color_BGRA: [u8; 4] = [
        (255.0 * palette.primary.b) as u8,
        (255.0 * palette.primary.g) as u8,
        (255.0 * palette.primary.r) as u8,
        255
    ];
    let secondary_color_BGRA: [u8; 4] = [
        (255.0 * palette.secondary.b) as u8,
        (255.0 * palette.secondary.g) as u8,
        (255.0 * palette.secondary.r) as u8,
        255
    ];
    let scale = scale as usize;

    for j in 0..cpu::ScreenHeight * scale {
        for i in 0..cpu::ScreenWidth * scale {
            let pixelIndexFlatDst: usize = j * cpu::ScreenWidth * scale + i;
            let pixelOutputOffsetInBytes: usize = pixelIndexFlatDst * PixelFormatBGRASizeInBytes;
            let pixelValue: u8 = display::read_screen_pixel(state, i / scale, j / scale);

            if pixelValue != 0
            {
                imageOutput[pixelOutputOffsetInBytes + 0] = primary_color_BGRA[0];
                imageOutput[pixelOutputOffsetInBytes + 1] = primary_color_BGRA[1];
                imageOutput[pixelOutputOffsetInBytes + 2] = primary_color_BGRA[2];
                imageOutput[pixelOutputOffsetInBytes + 3] = primary_color_BGRA[3];
            }
            else
            {
                imageOutput[pixelOutputOffsetInBytes + 0] = secondary_color_BGRA[0];
                imageOutput[pixelOutputOffsetInBytes + 1] = secondary_color_BGRA[1];
                imageOutput[pixelOutputOffsetInBytes + 2] = secondary_color_BGRA[2];
                imageOutput[pixelOutputOffsetInBytes + 3] = secondary_color_BGRA[3];
            }
        }
    }
}

extern crate sdl2;

//use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;

pub fn execute_main_loop(state: &mut CPUState, config: &config::EmuConfig) -> Result<(), String>
{
    let scale = config.screen_scale as usize;
    let width = cpu::ScreenWidth * scale;
    let height = cpu::ScreenHeight * scale;
    let stride = width * PixelFormatBGRASizeInBytes; // No extra space between lines
    let size = stride * cpu::ScreenHeight * scale;

    let mut image = vec![0 as u8; size];

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut timer_subsystem = sdl_context.timer()?;

    let window = video_subsystem.window("CHIP-8 Emulator", width as u32, height as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .software() // TODO
        .build()
        //.present_vsync()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let rmask: u32;
    let gmask: u32;
    let bmask: u32;
    let amask: u32;

    // TODO
    //#if SDL_BYTEORDER == SDL_BIG_ENDIAN
    //    rmask = 0xff000000;
    //    gmask = 0x00ff0000;
    //    bmask = 0x0000ff00;
    //    amask = 0x000000ff;
    //#else // little endian, like x86
    rmask = 0x000000ff;
    gmask = 0x0000ff00;
    bmask = 0x00ff0000;
    amask = 0xff000000;
    //#endif

    let depth: usize = 32;
    let pitch = stride;

    //SDL_Surface* surf = SDL_CreateRGBSurfaceFrom(image, width, height, depth, pitch, rmask, gmask, bmask, amask);

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

        execution::execute_step(&config, state, delta_time_ms);

        fill_image_buffer(&mut image, state, &config.palette, scale as u32);

        // Draw
        let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
            .map_err(|e| e.to_string())?;

        // Create a red-green gradient
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..256 {
                for x in 0..256 {
                    let offset = y*pitch + x*3;
                    buffer[offset] = x as
                        u8;
                    buffer[offset
                        + 1]
                        = y
                        as
                        u8;
                    buffer[offset
                        +
                        2]
                        =
                        0;
                }
            }
        })?;

        canvas.clear();
        canvas.copy(&texture,
                    None,
                    Some(Rect::new(100,
                                   100,
                                   256,
                                   256)))?;
        canvas.copy_ex(&texture,
                       None,
                       Some(Rect::new(450,
                                      100,
                                      256,
                                      256)),
                                      30.0,
                                      None,
                                      false,
                                      false)?;
        canvas.present();
        //SDL_Texture* texture = SDL_CreateTextureFromSurface(ren, surf);

        //canvas.copy(&texture, None, None)?;
        //canvas.present();

        //SDL_DestroyTexture(texture);
        // TODO

        previous_time_ms = current_time_ms;
    }

    Ok(())
}
