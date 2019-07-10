pub enum OpCode
{
    CLS, // 00E0 - CLS
    RET, // 00EE - RET
    SYS { addr: u16 }, // 0nnn - SYS addr
    JP { addr: u16 }, // 1nnn - JP addr
    CALL { addr: u16 }, // 2nnn - CALL addr
    SE { reg: u8, value: u8 }, // 3xkk - SE Vx, byte
    SNE { reg: u8, value: u8 }, // 4xkk - SNE Vx, byte
    SE2 { reg_x: u8, reg_y: u8 }, // 5xy0 - SE Vx, Vy
    LD { reg: u8, value: u8 }, // 6xkk - LD Vx, byte
    ADD { reg: u8, value: u8 }, // 7xkk - ADD Vx, byte
    LD2 { reg_x: u8, reg_y: u8 }, // 8xy0 - LD Vx, Vy
    OR { reg_x: u8, reg_y: u8 }, // 8xy1 - OR Vx, Vy
    AND { reg_x: u8, reg_y: u8 }, // 8xy2 - AND Vx, Vy
    XOR { reg_x: u8, reg_y: u8 }, // 8xy3 - XOR Vx, Vy
    ADD2 { reg_x: u8, reg_y: u8 }, // 8xy4 - ADD Vx, Vy
    SUB { reg_x: u8, reg_y: u8 }, // 8xy5 - SUB Vx, Vy
    SHR { reg_x: u8, reg_y: u8 }, // 8xy6 - SHR Vx {, Vy}
    SUBN { reg_x: u8, reg_y: u8 }, // 8xy7 - SUBN Vx, Vy
    SHL { reg_x: u8, reg_y: u8 }, // 8xyE - SHL Vx {, Vy}
    SNE2 { reg_x: u8, reg_y: u8 }, // 9xy0 - SNE Vx, Vy
    LDI { addr: u16 }, // Annn - LD I, addr
    JP2 { addr: u16 }, // Bnnn - JP V0, addr
    RND { reg: u8, value: u8 }, // Cxkk - RND Vx, byte
    DRW { reg_x: u8, reg_y: u8, size: u8 }, // Dxyn - DRW Vx, Vy, nibble
    SKP { reg: u8 }, // Ex9E - SKP Vx
    SKNP { reg: u8 }, // ExA1 - SKNP Vx
    LDT { reg: u8 }, // Fx07 - LD Vx, DT
    LDK { reg: u8 }, // Fx0A - LD Vx, K
    LDDT { reg: u8 }, // Fx15 - LD DT, Vx
    LDST { reg: u8 }, // Fx18 - LD ST, Vx
    ADDI { reg: u8 }, // Fx1E - ADD I, Vx
    LDF { reg: u8 }, // Fx29 - LD F, Vx
    LDB { reg: u8 }, // Fx33 - LD B, Vx
    LDAI { reg: u8 }, // Fx55 - LD [I], Vx
    LDM { reg: u8 }, // Fx65 - LD Vx, [I]
}

pub fn decode_instruction(instruction: u16) -> OpCode
{
    let first_nibble = instruction & 0xF000;

    match first_nibble {
        0x0000 => {
            match decode_0xxx(instruction) {
                0x00E0 => OpCode::CLS,
                0x00EE => OpCode::RET,
                _ => OpCode::SYS {addr: decode_0xxx(instruction)},
            }
        },
        0x1000 => OpCode::JP {addr: decode_0xxx(instruction)},
        0x2000 => OpCode::CALL {addr: decode_0xxx(instruction)},
        0x3000 => OpCode::SE {reg: decode_0x00(instruction), value: decode_00xx(instruction)},
        0x4000 => OpCode::SNE {reg: decode_0x00(instruction), value: decode_00xx(instruction)},
        0x5000 => OpCode::SE2 {reg_x: decode_0x00(instruction), reg_y: decode_00x0(instruction)},
        0x6000 => OpCode::LD {reg: decode_0x00(instruction), value: decode_00xx(instruction)},
        0x7000 => OpCode::ADD {reg: decode_0x00(instruction), value: decode_00xx(instruction)},
        0x8000 => {
            let reg_x = decode_0x00(instruction);
            let reg_y = decode_00x0(instruction);
            match decode_000x(instruction) {
                0x0 => OpCode::LD2 {reg_x, reg_y},
                0x1 => OpCode::OR {reg_x, reg_y},
                0x2 => OpCode::AND {reg_x, reg_y},
                0x3 => OpCode::XOR {reg_x, reg_y},
                0x4 => OpCode::ADD2 {reg_x, reg_y},
                0x5 => OpCode::SUB {reg_x, reg_y},
                0x6 => OpCode::SHR {reg_x, reg_y},
                0x7 => OpCode::SUBN {reg_x, reg_y},
                0xE => OpCode::SHL {reg_x, reg_y},
                _ => panic!("error: invalid opcode: 0x{:X}", instruction),
            }
        },
        0x9000 => OpCode::SNE2 {reg_x: decode_0x00(instruction), reg_y: decode_00x0(instruction)},
        0xA000 => OpCode::LDI {addr: decode_0xxx(instruction)},
        0xB000 => OpCode::JP2 {addr: decode_0xxx(instruction)},
        0xC000 => OpCode::RND {reg: decode_0x00(instruction), value: decode_00xx(instruction)},
        0xD000 => OpCode::DRW {reg_x: decode_0x00(instruction), reg_y: decode_00x0(instruction), size: decode_000x(instruction)},
        0xE000 => {
            let reg = decode_0x00(instruction);
            match decode_00xx(instruction) {
                0x9E => OpCode::SKP {reg},
                0xA1 => OpCode::SKNP {reg},
                _ => panic!("error: invalid opcode: 0x{:X}", instruction),
            }
        },
        0xF000 => {
            let reg = decode_0x00(instruction);
            match decode_00xx(instruction) {
                0x07 => OpCode::LDT {reg},
                0x0A => OpCode::LDK {reg},
                0x15 => OpCode::LDDT {reg},
                0x18 => OpCode::LDST {reg},
                0x1E => OpCode::ADDI {reg},
                0x29 => OpCode::LDF {reg},
                0x33 => OpCode::LDB {reg},
                0x55 => OpCode::LDAI {reg},
                0x65 => OpCode::LDM {reg},
                _ => panic!("error: invalid opcode: 0x{:X}", instruction),
            }
        },
        _ => panic!("error: invalid opcode: 0x{:X}", instruction),
    }
}

// Address
fn decode_0xxx(instruction: u16) -> u16
{
    instruction & 0x0FFF
}

// Whole byte
fn decode_00xx(instruction: u16) -> u8
{
    (instruction & 0x00FF) as u8
}

// One nibble
fn decode_0x00(instruction: u16) -> u8
{
    ((instruction & 0x0F00) >> 8) as u8
}

fn decode_00x0(instruction: u16) -> u8
{
    ((instruction & 0x00F0) >> 4) as u8
}

fn decode_000x(instruction: u16) -> u8
{
    (instruction & 0x000F) as u8
}
