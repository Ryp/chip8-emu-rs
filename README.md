# Chip-8 Emulator

This is a Rust re-write of my existing C++ [CHIP-8 Emu](https://github.com/Ryp/chip8-emu). Only the specifics of this version will be detailed here.

## Building

This should get you going after cloning the repo:
```sh
$ cargo run --release -- <rom_file>
```

**Disclaimer** Debug perf is absolutely horrible. Use release if you're not digging into the code.

**Disclaimer:** I didn't spend too much effort making this portable/packaged at all.
