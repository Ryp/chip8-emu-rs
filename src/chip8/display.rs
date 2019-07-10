use super::cpu::CPUState;

pub fn read_screen_pixel(state: &CPUState, x: usize, y: usize) -> bool
{
    let screen_offset_byte = x / 8;
    let screen_offset_bit = x % 8;

    ((state.screen[y][screen_offset_byte] >> screen_offset_bit) & 0x1) != 0
}

pub fn write_screen_pixel(state: &mut CPUState, x: usize, y: usize, value: bool)
{
    let screen_offset_byte = x / 8;
    let screen_offset_bit = x % 8;

    let mask = (1 << screen_offset_bit) as u8;
    let screen_byte_value = state.screen[y][screen_offset_byte];

    state.screen[y][screen_offset_byte] = screen_byte_value & !mask | (value as u8) << screen_offset_bit as u8;
}
