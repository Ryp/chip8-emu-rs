pub const VRegisterCount: usize = 16;
pub const StackSize: usize = 16;
pub const MemorySizeInBytes: usize = 0x1000;

// Fonts
const FontTableGlyphCount: usize = 16;
const GlyphSizeInBytes: usize = 5;

// Display
pub const ScreenWidth: usize = 64;
pub const ScreenHeight: usize = 32;
pub const ScreenLineSizeInBytes: usize = ScreenWidth / 8;

// Memory
pub const MinProgramAddress: usize = 0x0200;
pub const MaxProgramAddress: u16 = 0x0FFF;

// Timings
pub const DelayTimerFrequency: u32 = 60;
pub const InstructionExecutionFrequency: u32 = 500;
pub const DelayTimerPeriodMs: u32 = 1000 / DelayTimerFrequency;
pub const InstructionExecutionPeriodMs: u32 = 1000 / InstructionExecutionFrequency;

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
    pub stack: [u16; StackSize],
    pub vRegisters: [u8; VRegisterCount],
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

    pub fontTableOffsets: [u16; FontTableGlyphCount],
    pub screen: Vec<Vec<u8>>,
}

const FontTableOffsetInBytes: usize = 0x0000;
const FontTable: [u8; GlyphSizeInBytes * FontTableGlyphCount] =
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
    let tableOffset = FontTableOffsetInBytes;
    let tableSize = FontTableGlyphCount * GlyphSizeInBytes;

    // Make sure we don't spill in program addressable space.
    assert!((tableOffset + tableSize - 1) < MinProgramAddress);

    let fontRangeBegin = FontTableOffsetInBytes;
    let fontRangeEnd = FontTableOffsetInBytes + tableSize;
    state.memory[fontRangeBegin..fontRangeEnd].clone_from_slice(&FontTable[..]);

    // Assing font table addresses in memory
    for tableIndex in 0..FontTableGlyphCount {
        state.fontTableOffsets[tableIndex] = (tableOffset + GlyphSizeInBytes * tableIndex) as u16;
    }
}

pub fn createCPUState() -> CPUState
{
    let mut state: CPUState = Default::default();

    // Set PC to first address
    state.pc = MinProgramAddress as u16;

    // Clear memory
    state.memory = vec![0; MemorySizeInBytes];

    // Clear screen
    state.screen = vec![vec![0; ScreenLineSizeInBytes]; ScreenHeight];

    load_font_table(&mut state);

    state
}
