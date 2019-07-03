use super::cpu::CPUState;

pub fn read_screen_pixel(state: &CPUState, x: usize, y: usize) -> u8
{
    let screenOffsetByte = x / 8;
    let screenOffsetBit = x % 8;

    let value = (state.screen[y][screenOffsetByte] >> screenOffsetBit) & 0x1;

    assert!(value == 0 || value == 1);

    value as u8
}

pub fn write_screen_pixel(state: &mut CPUState, x: usize, y: usize, value: u8)
{
    assert!(value == 0 || value == 1);

    let screenOffsetByte = x / 8;
    let screenOffsetBit = x % 8;

    let mask = (1 << screenOffsetBit) as u8;
    let screenByteValue = state.screen[y][screenOffsetByte];

    state.screen[y][screenOffsetByte] = screenByteValue & !mask | value << screenOffsetBit as u8;
}
