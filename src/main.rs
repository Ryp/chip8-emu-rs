use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod chip8;
mod sdl2;

#[macro_use]
extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("chip8-emu-rs")
        .arg(Arg::with_name("rom_path")
             .help("path of the CHIP-8 ROM to load")
             .required(true)
             .index(1))
        .arg(Arg::with_name("debug")
             .short("d")
             .help("enable debug mode"))
        .arg(Arg::with_name("scale")
             .takes_value(true)
             .short("s")
             .help("screen upscale factor"))
        .get_matches();

    let rom_path = matches.value_of("rom_path").unwrap();
    let screen_scale = value_t!(matches, "scale", u32).unwrap_or(16);
    let debug_mode = matches.is_present("debug");

    let mut state: chip8::CPUState = chip8::create_chip8_state();

    // Load program in chip8 memory
    {
        println!("[INFO] loading program: {:?}", rom_path);

        let mut rom_file = File::open(&Path::new(&rom_path)).expect("Could not open file");
        let mut rom_content = Vec::<u8>::new();
        rom_file.read_to_end(&mut rom_content).expect("Unable to read the file");

        println!("[INFO] program loaded");

        chip8::load_program(&mut state, rom_content);
    }

    let config = chip8::EmuConfig {
        debug_mode: debug_mode,
        palette: chip8::Palette {
            primary: chip8::Color { r: 1.0, g: 1.0, b: 1.0 },
            secondary: chip8::Color { r: 0.14, g: 0.14, b: 0.14 }
        },
        screen_scale: screen_scale
    };

    sdl2::execute_main_loop(&mut state, &config).unwrap();
}
