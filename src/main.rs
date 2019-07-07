use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod chip8;
mod sdl2;

fn main() {
    let args: Vec<_> = env::args().collect();
    assert_eq!(args.len(), 2);

    let mut state: chip8::CPUState = chip8::create_chip8_state();

    // Load program in chip8 memory
    {
        let rom_file: String = args[1].to_string();
        println!("[INFO] loading program: {}", rom_file);

        let mut file = File::open(&Path::new(&rom_file)).expect("Could not open file");
        let mut rom_content = Vec::<u8>::new();
        file.read_to_end(&mut rom_content).expect("Unable to read the file");

        println!("[INFO] program loaded");

        chip8::load_program(&mut state, rom_content);
    }

    let config = chip8::EmuConfig {
        debug_mode: true,
        palette: chip8::Palette {
            primary: chip8::Color { r: 1.0, g: 1.0, b: 1.0 },
            secondary: chip8::Color { r: 0.14, g: 0.14, b: 0.14 }
        },
        screen_scale: 16
    };

    sdl2::execute_main_loop(&mut state, &config).unwrap();
}
