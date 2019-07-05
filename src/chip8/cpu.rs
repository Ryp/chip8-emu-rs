pub const V_REGISTER_COUNT: usize = 16;
pub const STACK_SIZE: usize = 16;
pub const MEMORY_SIZE_IN_BYTES: usize = 0x1000;

// Display
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_LINE_SIZE_IN_BYTES: usize = SCREEN_WIDTH / 8;

// Memory
pub const MIN_PROGRAM_ADDRESS: usize = 0x0200;
pub const MAX_PROGRAM_ADDRESS: usize = 0x0FFF;

// Timings
pub const DELAY_TIMER_FREQUENCY: u32 = 60;
pub const DELAY_TIMER_PERIOD_MS: u32 = 1000 / DELAY_TIMER_FREQUENCY;
pub const INSTRUCTION_EXECUTION_FREQUENCY: u32 = 500;
pub const INSTRUCTION_EXECUTION_PERIOD_MS: u32 = 1000 / INSTRUCTION_EXECUTION_FREQUENCY;

// Fonts
const FONT_TABLE_GLYPH_COUNT: usize = 16;
const GLYPH_SIZE_IN_BYTES: usize = 5;

#[allow(dead_code)]
pub enum VRegisterName
{
    V0, V1, V2, V3,
    V4, V5, V6, V7,
    V8, V9, VA, VB,
    VC, VD, VE, VF
}

#[derive(Default)]
pub struct CPUState
{
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; STACK_SIZE],
    pub vRegisters: [u8; V_REGISTER_COUNT],
    pub i: u16,

    pub delayTimer: u8,
    pub soundTimer: u8,

    // Implementation detail
    pub delayTimerAccumulator: u32,
    pub executionTimerAccumulator: u32,

    pub memory: Vec<u8>,

    pub key_state: u16,

    pub key_state_prev: u16,
    pub isWaitingForKey: bool,

    pub fontTableOffsets: [u16; FONT_TABLE_GLYPH_COUNT],
    pub screen: Vec<Vec<u8>>,
}

const FONT_TABLE_OFFSET_IN_BYTES: usize = 0x0000;
const FONT_TABLE: [u8; GLYPH_SIZE_IN_BYTES * FONT_TABLE_GLYPH_COUNT] =
[
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // etc...
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    0x90, 0x90, 0xF0, 0x10, 0x10,
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    0xF0, 0x10, 0x20, 0x40, 0x40,
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    0xF0, 0x90, 0xF0, 0x90, 0x90,
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    0xF0, 0x80, 0xF0, 0x80, 0x80
];

fn load_font_table(state: &mut CPUState)
{
    let tableOffset = FONT_TABLE_OFFSET_IN_BYTES;
    let tableSize = FONT_TABLE_GLYPH_COUNT * GLYPH_SIZE_IN_BYTES;

    // Make sure we don't spill in program addressable space.
    assert!((tableOffset + tableSize - 1) < MIN_PROGRAM_ADDRESS);

    let fontRangeBegin = FONT_TABLE_OFFSET_IN_BYTES;
    let fontRangeEnd = FONT_TABLE_OFFSET_IN_BYTES + tableSize;
    state.memory[fontRangeBegin..fontRangeEnd].clone_from_slice(&FONT_TABLE[..]);

    // Assing font table addresses in memory
    for tableIndex in 0..FONT_TABLE_GLYPH_COUNT {
        state.fontTableOffsets[tableIndex] = (tableOffset + GLYPH_SIZE_IN_BYTES * tableIndex) as u16;
    }
}

pub fn createCPUState() -> CPUState
{
    let mut state: CPUState = Default::default();

    // Set PC to first address
    state.pc = MIN_PROGRAM_ADDRESS as u16;

    // Clear memory
    state.memory = vec![0; MEMORY_SIZE_IN_BYTES];

    // Clear screen
    state.screen = vec![vec![0; SCREEN_LINE_SIZE_IN_BYTES]; SCREEN_HEIGHT];

    load_font_table(&mut state);

    state
}
