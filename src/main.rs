mod chip8;
mod sdl2;

#[macro_use]
extern crate clap;
use clap::{Arg, App};

fn main() {
    // Argument parsing
    let matches = App::new("CHIP-8 Emulator")
        .arg(Arg::with_name("rom_path")
             .help("path of the CHIP-8 ROM to load")
             .required(true)
             .index(1))
        .arg(Arg::with_name("debug")
             .short("d")
             .help("enable debug mode"))
        .arg(Arg::with_name("scale")
             .short("s")
             .takes_value(true)
             .help("screen upscale factor"))
        .get_matches();

    let rom_path = matches.value_of("rom_path").unwrap();

    let config = chip8::EmuConfig {
        debug_mode: matches.is_present("debug"),
        palette: chip8::Palette {
            primary: chip8::Color { r: 1.0, g: 1.0, b: 1.0 },
            secondary: chip8::Color { r: 0.14, g: 0.14, b: 0.14 }
        },
        screen_scale: value_t!(matches, "scale", u32).unwrap_or(16),
    };

    let mut state: chip8::CPUState = chip8::create_chip8_state();

    let rom_content = std::fs::read(&rom_path).expect("Unable to read file");

    chip8::load_program(&mut state, rom_content);

    sdl2::execute_main_loop(&mut state, &config).unwrap();
}
