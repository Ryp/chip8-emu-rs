use super::cpu::CPUState;

pub fn read_screen_pixel(state: &CPUState, x: usize, y: usize) -> u8
{
    let screen_offset_byte = x / 8;
    let screen_offset_bit = x % 8;

    let value = (state.screen[y][screen_offset_byte] >> screen_offset_bit) & 0x1;

    assert!(value == 0 || value == 1);

    value as u8
}

pub fn write_screen_pixel(state: &mut CPUState, x: usize, y: usize, value: u8)
{
    assert!(value == 0 || value == 1);

    let screen_offset_byte = x / 8;
    let screen_offset_bit = x % 8;

    let mask = (1 << screen_offset_bit) as u8;
    let screen_byte_value = state.screen[y][screen_offset_byte];

    state.screen[y][screen_offset_byte] = screen_byte_value & !mask | value << screen_offset_bit as u8;
}
