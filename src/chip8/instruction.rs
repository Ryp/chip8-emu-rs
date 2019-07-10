use super::{
    cpu,
    cpu::CPUState,
    cpu::VRegisterName::*,
    display,
    keyboard,
    memory,
    memory::MemoryUsage,
    opcode::OpCode,
};

use rand::prelude::*;

pub fn execute_instruction_internal(state: &mut cpu::CPUState, instruction: OpCode)
{
    match instruction {
        OpCode::CLS => execute_cls(state),
        OpCode::RET => execute_ret(state),
        OpCode::SYS{addr} => execute_sys(state, addr),
        OpCode::JP{addr} => execute_jp(state, addr),
        OpCode::CALL{addr} => execute_call(state, addr),
        OpCode::SE{reg, value} => execute_se(state, reg, value),
        OpCode::SNE{reg, value} => execute_sne(state, reg, value),
        OpCode::SE2{reg_x, reg_y} => execute_se2(state, reg_x, reg_y),
        OpCode::LD{reg, value} => execute_ld(state, reg, value),
        OpCode::ADD{reg, value} => execute_add(state, reg, value),
        OpCode::LD2{reg_x, reg_y} => execute_ld2(state, reg_x, reg_y),
        OpCode::OR{reg_x, reg_y} => execute_or(state, reg_x, reg_y),
        OpCode::AND{reg_x, reg_y} => execute_and(state, reg_x, reg_y),
        OpCode::ADD2{reg_x, reg_y} => execute_add2(state, reg_x, reg_y),
        OpCode::SUB{reg_x, reg_y} => execute_sub(state, reg_x, reg_y),
        OpCode::XOR{reg_x, reg_y} => execute_xor(state, reg_x, reg_y),
        OpCode::SHR{reg_x, reg_y} => execute_shr(state, reg_x, reg_y),
        OpCode::SUBN{reg_x, reg_y} => execute_subn(state, reg_x, reg_y),
        OpCode::SHL{reg_x, reg_y} => execute_shl(state, reg_x, reg_y),
        OpCode::SNE2{reg_x, reg_y} => execute_sne2(state, reg_x, reg_y),
        OpCode::LDI{addr} => execute_ldi(state, addr),
        OpCode::JP2{addr} => execute_jp2(state, addr),
        OpCode::DRW{reg_x, reg_y, size} => execute_drw(state, reg_x, reg_y, size),
        OpCode::RND{reg, value} => execute_rnd(state, reg, value),
        OpCode::SKP{reg} => execute_skp(state, reg),
        OpCode::SKNP{reg} => execute_sknp(state, reg),
        OpCode::LDT{reg} => execute_ldt(state, reg),
        OpCode::LDK{reg} => execute_ldk(state, reg),
        OpCode::LDDT{reg} => execute_lddt(state, reg),
        OpCode::LDST{reg} => execute_ldst(state, reg),
        OpCode::ADDI{reg} => execute_addi(state, reg),
        OpCode::LDF{reg} => execute_ldf(state, reg),
        OpCode::LDB{reg} => execute_ldb(state, reg),
        OpCode::LDAI{reg} => execute_ldai(state, reg),
        OpCode::LDM{reg} => execute_ldm(state, reg),
    }
}

// Clear the display.
pub fn execute_cls(state: &mut CPUState)
{
    state.screen = vec![vec![0; cpu::SCREEN_LINE_SIZE_IN_BYTES]; cpu::SCREEN_HEIGHT];
}

// Return from a subroutine.
// The interpreter sets the program counter to the address at the top of the stack,
// then subtracts 1 from the stack pointer.
pub fn execute_ret(state: &mut CPUState)
{
    assert!(state.sp > 0); // Stack Underflow

    let next_pc_value: u16 = state.stack[state.sp as usize] + 2;
    assert!(memory::is_valid_memory_range(next_pc_value, 2, MemoryUsage::Execute));

    state.pc = next_pc_value;
    state.sp = if state.sp > 0 { state.sp - 1 } else { state.sp };
}

// Jump to a machine code routine at nnn.
// This instruction is only used on the old computers on which Chip-8 was originally implemented.
// NOTE: We choose to ignore it since we don't load any code into system memory.
pub fn execute_sys(_state: &mut CPUState, _address: u16)
{
    // noop
}

// Jump to location nnn.
// The interpreter sets the program counter to nnn.
pub fn execute_jp(state: &mut CPUState, address: u16)
{
    assert!((address & 0x0001) == 0); // Unaligned address
    assert!(memory::is_valid_memory_range(address, 2, MemoryUsage::Execute));

    state.pc = address;
}

// Call subroutine at nnn.
// The interpreter increments the stack pointer, then puts the current PC on the top of the stack.
// The PC is then set to nnn.
pub fn execute_call(state: &mut CPUState, address: u16)
{
    assert!((address & 0x0001) == 0); // Unaligned address
    assert!(memory::is_valid_memory_range(address, 2, MemoryUsage::Execute));

    assert!((state.sp as usize) < cpu::STACK_SIZE); // Stack overflow

    state.sp = if (state.sp as usize) < cpu::STACK_SIZE { state.sp + 1 } else { state.sp }; // Increment sp
    state.stack[state.sp as usize] = state.pc; // Put PC on top of the stack
    state.pc = address; // Set PC to new address
}

// Skip next instruction if Vx = kk.
// The interpreter compares register Vx to kk, and if they are equal,
// increments the program counter by 2.
pub fn execute_se(state: &mut CPUState, register_name: u8, value: u8)
{
    assert!((register_name & !0x0F) == 0); // Invalid register
    assert!(memory::is_valid_memory_range(state.pc, 6, MemoryUsage::Execute));

    let register_value: u8 = state.v_registers[register_name as usize];

    if register_value == value {
        state.pc += 4;
    }
}

// Skip next instruction if Vx != kk.
// The interpreter compares register Vx to kk, and if they are not equal,
// increments the program counter by 2.
pub fn execute_sne(state: &mut CPUState, register_name: u8, value: u8)
{
    let register_value: u8 = state.v_registers[register_name as usize];

    assert!((register_name & !0x0F) == 0); // Invalid register
    assert!(memory::is_valid_memory_range(state.pc, 6, MemoryUsage::Execute));

    if register_value != value {
        state.pc += 4;
    }
}

// Skip next instruction if Vx = Vy.
// The interpreter compares register Vx to register Vy, and if they are equal,
// increments the program counter by 2.
pub fn execute_se2(state: &mut CPUState, register_lhs: u8, register_rhs: u8)
{
    assert!((register_lhs & !0x0F) == 0); // Invalid register
    assert!((register_rhs & !0x0F) == 0); // Invalid register
    assert!(memory::is_valid_memory_range(state.pc, 6, MemoryUsage::Execute));

    let register_value_lhs: u8 = state.v_registers[register_lhs as usize];
    let register_value_rhs: u8 = state.v_registers[register_rhs as usize];

    if register_value_lhs == register_value_rhs {
        state.pc += 4;
    }
}

// Set Vx = kk.
// The interpreter puts the value kk into register Vx.
pub fn execute_ld(state: &mut CPUState, register_name: u8, value: u8)
{
    assert!((register_name & !0x0F) == 0); // Invalid register

    let register_name = register_name as usize;

    state.v_registers[register_name] = value;
}

// Set Vx = Vx + kk.
// Adds the value kk to the value of register Vx, then stores the result in Vx.
// NOTE: Carry in NOT set.
// NOTE: Overflows will just wrap the value around.
pub fn execute_add(state: &mut CPUState, register_name: u8, value: u8)
{
    assert!((register_name & !0x0F) == 0); // Invalid register

    let register_name = register_name as usize;

    let register_value: u8 = state.v_registers[register_name];
    let sum: u8 = register_value.wrapping_add(value);

    state.v_registers[register_name] = sum;
}

// Set Vx = Vy.
// Stores the value of register Vy in register Vx.
pub fn execute_ld2(state: &mut CPUState, register_lhs: u8, register_rhs: u8)
{
    assert!((register_lhs & !0x0F) == 0); // Invalid register
    assert!((register_rhs & !0x0F) == 0); // Invalid register

    let register_lhs = register_lhs as usize;
    let register_rhs = register_rhs as usize;

    state.v_registers[register_lhs] = state.v_registers[register_rhs];
}

// Set Vx = Vx OR Vy.
// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
// A bitwise OR compares the corrseponding bits from two values, and if either bit is 1,
// then the same bit in the result is also 1. Otherwise, it is 0.
pub fn execute_or(state: &mut CPUState, register_lhs: u8, register_rhs: u8)
{
    assert!((register_lhs & !0x0F) == 0); // Invalid register
    assert!((register_rhs & !0x0F) == 0); // Invalid register

    let register_lhs = register_lhs as usize;
    let register_rhs = register_rhs as usize;

    state.v_registers[register_lhs] |= state.v_registers[register_rhs];
}

// Set Vx = Vx AND Vy.
// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
// A bitwise AND compares the corrseponding bits from two values, and if both bits are 1,
// then the same bit in the result is also 1. Otherwise, it is 0.
pub fn execute_and(state: &mut CPUState, register_lhs: u8, register_rhs: u8)
{
    assert!((register_lhs & !0x0F) == 0); // Invalid register
    assert!((register_rhs & !0x0F) == 0); // Invalid register

    let register_lhs = register_lhs as usize;
    let register_rhs = register_rhs as usize;

    state.v_registers[register_lhs] &= state.v_registers[register_rhs];
}

// Set Vx = Vx XOR Vy.
// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
// An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same,
// then the corresponding bit in the result is set to 1.  Otherwise, it is 0.
pub fn execute_xor(state: &mut CPUState, register_lhs: u8, register_rhs: u8)
{
    assert!((register_lhs & !0x0F) == 0); // Invalid register
    assert!((register_rhs & !0x0F) == 0); // Invalid register

    let register_lhs = register_lhs as usize;
    let register_rhs = register_rhs as usize;

    state.v_registers[register_lhs] ^= state.v_registers[register_rhs];
}

// Set Vx = Vx + Vy, set VF = carry.
// The values of Vx and Vy are added together.
// If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
// Only the lowest 8 bits of the result are kept, and stored in Vx.
pub fn execute_add2(state: &mut CPUState, register_lhs: u8, register_rhs: u8)
{
    assert!((register_lhs & !0x0F) == 0); // Invalid register
    assert!((register_rhs & !0x0F) == 0); // Invalid register

    let register_lhs = register_lhs as usize;
    let register_rhs = register_rhs as usize;

    let value_lhs: u8 = state.v_registers[register_lhs];
    let value_rhs: u8 = state.v_registers[register_rhs];
    let result: u8 = value_lhs.wrapping_add(value_rhs);

    state.v_registers[register_lhs] = result;
    state.v_registers[VF as usize] = if result > value_lhs { 0 } else { 1 }; // Set carry
}

// Set Vx = Vx - Vy, set VF = NOT borrow.
// If Vx > Vy, then VF is set to 1, otherwise 0.
// Then Vy is subtracted from Vx, and the results stored in Vx.
pub fn execute_sub(state: &mut CPUState, register_lhs: u8, register_rhs: u8)
{
    assert!((register_lhs & !0x0F) == 0); // Invalid register
    assert!((register_rhs & !0x0F) == 0); // Invalid register

    let register_lhs = register_lhs as usize;
    let register_rhs = register_rhs as usize;

    let value_lhs: u8 = state.v_registers[register_lhs];
    let value_rhs: u8 = state.v_registers[register_rhs];
    let result: u8 = value_lhs.wrapping_sub(value_rhs);

    state.v_registers[register_lhs] = result;
    state.v_registers[VF as usize] = if value_lhs > value_rhs { 1 } else { 0 }; // Set carry
}

// Set Vx = Vx SHR 1.
// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
// Then Vx is divided by 2.
// NOTE: register_rhs is just ignored apparently.
pub fn execute_shr(state: &mut CPUState, register_lhs: u8, _register_rhs: u8)
{
    assert!((register_lhs & !0x0F) == 0); // Invalid register

    let register_lhs = register_lhs as usize;

    let value_lhs: u8 = state.v_registers[register_lhs];

    state.v_registers[register_lhs] = value_lhs >> 1;
    state.v_registers[VF as usize] = value_lhs & 0x01; // Set carry
}

// Set Vx = Vy - Vx, set VF = NOT borrow.
// If Vy > Vx, then VF is set to 1, otherwise 0.
// Then Vx is subtracted from Vy, and the results stored in Vx.
pub fn execute_subn(state: &mut CPUState, register_lhs: u8, register_rhs: u8)
{
    assert!((register_lhs & !0x0F) == 0); // Invalid register
    assert!((register_rhs & !0x0F) == 0); // Invalid register

    let register_lhs = register_lhs as usize;
    let register_rhs = register_rhs as usize;

    let value_lhs: u8 = state.v_registers[register_lhs];
    let value_rhs: u8 = state.v_registers[register_rhs];
    let result: u8 = value_rhs.wrapping_sub(value_lhs);

    state.v_registers[register_lhs] = result;
    state.v_registers[VF as usize] = if value_rhs > value_lhs { 1 } else { 0 }; // Set carry
}

// Set Vx = Vx SHL 1.
// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
// NOTE: register_rhs is just ignored apparently.
pub fn execute_shl(state: &mut CPUState, register_lhs: u8, _register_rhs: u8)
{
    assert!((register_lhs & !0x0F) == 0); // Invalid register

    let register_lhs = register_lhs as usize;

    let value_lhs: u8 = state.v_registers[register_lhs];

    state.v_registers[register_lhs] = value_lhs << 1;
    state.v_registers[VF as usize] = if (value_lhs & 0x80) != 0 { 1 } else { 0 }; // Set carry
}

// Skip next instruction if Vx != Vy.
// The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
pub fn execute_sne2(state: &mut CPUState, register_lhs: u8, register_rhs: u8)
{
    assert!((register_lhs & !0x0F) == 0); // Invalid register
    assert!((register_rhs & !0x0F) == 0); // Invalid register
    assert!(memory::is_valid_memory_range(state.pc, 6, MemoryUsage::Execute));

    let register_lhs = register_lhs as usize;
    let register_rhs = register_rhs as usize;

    let value_lhs: u8 = state.v_registers[register_lhs];
    let value_rhs: u8 = state.v_registers[register_rhs];

    if value_lhs != value_rhs {
        state.pc += 4;
    }
}

// Set I = nnn.
// The value of register I is set to nnn.
pub fn execute_ldi(state: &mut CPUState, address: u16)
{
    state.i = address;
}

// Jump to location nnn + V0.
// The program counter is set to nnn plus the value of V0.
pub fn execute_jp2(state: &mut CPUState, base_address: u16)
{
    let offset = u16::from(state.v_registers[V0 as usize]);
    let jump_address: u16 = base_address + offset;

    assert!((jump_address & 0x0001) == 0); // Unaligned address
    assert!(memory::is_valid_memory_range(jump_address, 2, MemoryUsage::Execute));

    state.pc = jump_address;
}

// Set Vx = random byte AND kk.
// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk.
// The results are stored in Vx. See instruction 8xy2 for more information on AND.
pub fn execute_rnd(state: &mut CPUState, register_name: u8, value: u8)
{
    assert!((register_name & !0x0F) == 0); // Invalid register

    let register_name = register_name as usize;

    let random_value: u8 = rand::thread_rng().gen();
    state.v_registers[register_name] = random_value & value;
}

// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
// The interpreter reads n bytes from memory, starting at the address stored in I.
// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
// Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1,
// otherwise it is set to 0.
// If the sprite is positioned so part of it is outside the coordinates of the display,
// it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR,
// and section 2.4, Display, for more information on the Chip-8 screen and sprites.
pub fn execute_drw(state: &mut CPUState, register_lhs: u8, register_rhs: u8, size: u8)
{
    assert!((register_lhs & !0x0F) == 0); // Invalid register
    assert!((register_rhs & !0x0F) == 0); // Invalid register
    assert!(memory::is_valid_memory_range(state.i, size as usize, MemoryUsage::Read));

    let register_lhs = register_lhs as usize;
    let register_rhs = register_rhs as usize;

    let sprite_start_x = state.v_registers[register_lhs] as usize;
    let sprite_start_y = state.v_registers[register_rhs] as usize;

    let mut collision: bool = false;

    // Sprites are made of rows of 1 byte each.
    for row_index in 0..size
    {
        let sprite_address = (state.i + u16::from(row_index)) as usize;
        let sprite_row: u8 = state.memory[sprite_address];
        let screen_y = (sprite_start_y + row_index as usize) % cpu::SCREEN_HEIGHT;

        for pixel_index in 0..8
        {
            let sprite_pixel_value = ((sprite_row >> (7 - pixel_index)) & 0x1) != 0;
            let screen_x: usize = (sprite_start_x + pixel_index) % cpu::SCREEN_WIDTH;

            let screen_pixel_value = display::read_screen_pixel(state, screen_x, screen_y);

            let result = screen_pixel_value ^ sprite_pixel_value;

            // A pixel was erased
            if screen_pixel_value && !result {
                collision = true;
            }

            display::write_screen_pixel(state, screen_x, screen_y, result);
        }
    }

    state.v_registers[VF as usize] = if collision { 1 } else { 0 };
}

// Skip next instruction if key with the value of Vx is pressed.
// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position,
// PC is increased by 2.
pub fn execute_skp(state: &mut CPUState, register_name: u8)
{
    assert!((register_name & !0x0F) == 0); // Invalid register
    assert!(memory::is_valid_memory_range(state.pc, 6, MemoryUsage::Execute));

    let key_id: u8 = state.v_registers[register_name as usize];

    if keyboard::is_key_pressed(state, key_id) {
        state.pc += 4;
    }
}

// Skip next instruction if key with the value of Vx is not pressed.
// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position,
// PC is increased by 2.
pub fn execute_sknp(state: &mut CPUState, register_name: u8)
{
    assert!((register_name & !0x0F) == 0); // Invalid register
    assert!(memory::is_valid_memory_range(state.pc, 6, MemoryUsage::Execute));

    let key: keyboard::KeyID = state.v_registers[register_name as usize];

    if !keyboard::is_key_pressed(state, key) {
        state.pc += 4;
    }
}

// Set Vx = delay timer value.
// The value of DT is placed into Vx.
pub fn execute_ldt(state: &mut CPUState, register_name: u8)
{
    assert!((register_name & !0x0F) == 0); // Invalid register

    state.v_registers[register_name as usize] = state.delay_timer;
}

// Wait for a key press, store the value of the key in Vx.
// All execution stops until a key is pressed, then the value of that key is stored in Vx.
pub fn execute_ldk(state: &mut CPUState, register_name: u8)
{
    assert!((register_name & !0x0F) == 0); // Invalid register

    // If we enter for the first time, set the waiting flag.
    if !state.is_waiting_for_key {
        state.is_waiting_for_key = true;
    } else {
        let key_state_press_mask: u16 = !state.key_state_prev & state.key_state;
        // When waiting, check the key states.
        if key_state_press_mask != 0 {
            state.v_registers[register_name as usize] = keyboard::get_key_pressed(key_state_press_mask);
            state.is_waiting_for_key = false;
        }
    }
}

// Set delay timer = Vx.
// DT is set equal to the value of Vx.
pub fn execute_lddt(state: &mut CPUState, register_name: u8)
{
    assert!((register_name & !0x0F) == 0); // Invalid register

    state.delay_timer = state.v_registers[register_name as usize];
}

// Set sound timer = Vx.
// ST is set equal to the value of Vx.
pub fn execute_ldst(state: &mut CPUState, register_name: u8)
{
    assert!((register_name & !0x0F) == 0); // Invalid register

    state.sound_timer = state.v_registers[register_name as usize];
}

// Set I = I + Vx.
// The values of I and Vx are added, and the results are stored in I.
// NOTE: Carry in NOT set.
// NOTE: Overflows will just wrap the value around.
pub fn execute_addi(state: &mut CPUState, register_name: u8)
{
    assert!((register_name & !0x0F) == 0); // Invalid register

    let register_value = u16::from(state.v_registers[register_name as usize]);
    let i_value: u16 = state.i;
    let sum: u16 = i_value + register_value;

    assert!(sum >= i_value); // Overflow

    state.i = sum;
}

// Set I = location of sprite for digit Vx.
// The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
// See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
pub fn execute_ldf(state: &mut CPUState, register_name: u8)
{
    assert!((register_name & !0x0F) == 0); // Invalid register

    let glyph_index: u8 = state.v_registers[register_name as usize];

    assert!((glyph_index & !0x0F) == 0); // Invalid index

    state.i = state.font_table_offsets[glyph_index as usize];
}

// Store BCD representation of Vx in memory locations I, I+1, and I+2.
// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I,
// the tens digit at location I+1, and the ones digit at location I+2.
pub fn execute_ldb(state: &mut CPUState, register_name: u8)
{
    assert!((register_name & !0x0F) == 0); // Invalid register
    assert!(memory::is_valid_memory_range(state.i, 3, MemoryUsage::Write));

    let register_value: u8 = state.v_registers[register_name as usize];

    let ip = state.i as usize;
    state.memory[ip]     = (register_value / 100) % 10;
    state.memory[ip + 1] = (register_value / 10) % 10;
    state.memory[ip + 2] = (register_value) % 10;
}

// Store registers V0 through Vx in memory starting at location I.
// The interpreter copies the values of registers V0 through Vx into memory,
// starting at the address in I.
pub fn execute_ldai(state: &mut CPUState, register_name: u8)
{
    let register_index_max = register_name as usize;

    assert!((register_index_max & !0x0F) == 0); // Invalid register
    assert!(memory::is_valid_memory_range(state.i, register_index_max + 1, MemoryUsage::Write));

    for index in 0..=register_index_max {
        state.memory[state.i as usize + index] = state.v_registers[index];
    }
}

// Read registers V0 through Vx from memory starting at location I.
// The interpreter reads values from memory starting at location I into registers V0 through Vx.
pub fn execute_ldm(state: &mut CPUState, register_name: u8)
{
    let register_index_max = register_name as usize;

    assert!((register_index_max & !0x0F) == 0); // Invalid register
    assert!(memory::is_valid_memory_range(state.i, register_index_max + 1, MemoryUsage::Read));

    for index in 0..=register_index_max {
        state.v_registers[index] = state.memory[state.i as usize + index];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::execution;

    #[test]
    fn instructions() {
        //SUBCASE("CLS")
        {
            let mut state = cpu::create_chip8_state();
            state.screen[0][0] = 0b11001100;
            state.screen[cpu::SCREEN_HEIGHT - 1][cpu::SCREEN_LINE_SIZE_IN_BYTES - 1] = 0b10101010;

            execution::execute_instruction(&mut state, 0x00E0);

            assert_eq!(state.screen[0][0], 0x00);
            assert_eq!(state.screen[cpu::SCREEN_HEIGHT - 1][cpu::SCREEN_LINE_SIZE_IN_BYTES - 1], 0x00);
        }

        //SUBCASE("JP")
        {
            let mut state = cpu::create_chip8_state();
            execution::execute_instruction(&mut state, 0x1240);

            assert_eq!(state.pc, 0x0240);

            execution::execute_instruction(&mut state, 0x1FFE);

            assert_eq!(state.pc, 0x0FFE);
        }

        //SUBCASE("CALL/RET")
        {
            let mut state = cpu::create_chip8_state();
            execution::execute_instruction(&mut state, 0x2F00);

            assert_eq!(state.sp, 1);
            assert_eq!(state.pc, 0x0F00);

            execution::execute_instruction(&mut state, 0x2A00);

            assert_eq!(state.sp, 2);
            assert_eq!(state.pc, 0x0A00);

            execution::execute_instruction(&mut state, 0x00EE);

            assert_eq!(state.sp, 1);
            assert_eq!(state.pc, 0x0F02);

            execution::execute_instruction(&mut state, 0x00EE);

            assert_eq!(state.sp, 0);
            assert_eq!(state.pc, cpu::MIN_PROGRAM_ADDRESS as u16 + 2);
        }

        //SUBCASE("SE")
        {
            let mut state = cpu::create_chip8_state();
            execution::execute_instruction(&mut state, 0x3000);

            assert_eq!(state.v_registers[V0 as usize], 0);
            assert_eq!(state.pc, cpu::MIN_PROGRAM_ADDRESS as u16 + 4);
        }

        //SUBCASE("SNE")
        {
            let mut state = cpu::create_chip8_state();
            execution::execute_instruction(&mut state, 0x40FF);

            assert_eq!(state.v_registers[V0 as usize], 0);
            assert_eq!(state.pc, cpu::MIN_PROGRAM_ADDRESS as u16 + 4);
        }

        //SUBCASE("SE2")
        {
            let mut state = cpu::create_chip8_state();
            execution::execute_instruction(&mut state, 0x5120);

            assert_eq!(state.v_registers[V0 as usize], 0);
            assert_eq!(state.v_registers[V1 as usize], 0);
            assert_eq!(state.pc, cpu::MIN_PROGRAM_ADDRESS as u16 + 4);
        }

        //SUBCASE("LD")
        {
            let mut state = cpu::create_chip8_state();
            execution::execute_instruction(&mut state, 0x06042);

            assert_eq!(state.v_registers[V0 as usize], 0x42);

            execution::execute_instruction(&mut state, 0x06A33);

            assert_eq!(state.v_registers[VA as usize], 0x33);
        }

        //SUBCASE("ADD")
        {
            let mut state = cpu::create_chip8_state();
            assert_eq!(state.v_registers[V2 as usize], 0x00);

            execution::execute_instruction(&mut state, 0x7203);

            assert_eq!(state.v_registers[V2 as usize], 0x03);

            execution::execute_instruction(&mut state, 0x7204);

            assert_eq!(state.v_registers[V2 as usize], 0x07);
        }

        //SUBCASE("LD2")
        {
            let mut state = cpu::create_chip8_state();

            state.v_registers[V3 as usize] = 32;

            execution::execute_instruction(&mut state, 0x8030);

            assert_eq!(state.v_registers[V0 as usize], 32);
        }

        //SUBCASE("OR")
        {
            let mut state = cpu::create_chip8_state();

            state.v_registers[VC as usize] = 0xF0;
            state.v_registers[VD as usize] = 0x0F;

            execution::execute_instruction(&mut state, 0x8CD1);

            assert_eq!(state.v_registers[VC as usize], 0xFF);
        }

        //SUBCASE("AND")
        {
            let mut state = cpu::create_chip8_state();

            state.v_registers[VC as usize] = 0xF0;
            state.v_registers[VD as usize] = 0x0F;

            execution::execute_instruction(&mut state, 0x8CD2);

            assert_eq!(state.v_registers[VC as usize], 0x00);

            state.v_registers[VC as usize] = 0xF0;
            state.v_registers[VD as usize] = 0xFF;

            execution::execute_instruction(&mut state, 0x8CD2);

            assert_eq!(state.v_registers[VC as usize], 0xF0);
        }

        //SUBCASE("XOR")
        {
            let mut state = cpu::create_chip8_state();

            state.v_registers[VC as usize] = 0x10;
            state.v_registers[VD as usize] = 0x1F;

            execution::execute_instruction(&mut state, 0x8CD3);

            assert_eq!(state.v_registers[VC as usize], 0x0F);
        }

        //SUBCASE("ADD")
        {
            let mut state = cpu::create_chip8_state();

            state.v_registers[V0 as usize] = 8;
            state.v_registers[V1 as usize] = 8;

            execution::execute_instruction(&mut state, 0x8014);

            assert_eq!(state.v_registers[V0 as usize], 16);
            assert_eq!(state.v_registers[VF as usize], 0);

            state.v_registers[V0 as usize] = 128;
            state.v_registers[V1 as usize] = 130;

            execution::execute_instruction(&mut state, 0x8014);

            assert_eq!(state.v_registers[V0 as usize], 2);
            assert_eq!(state.v_registers[VF as usize], 1);
        }

        //SUBCASE("SUB")
        {
            let mut state = cpu::create_chip8_state();

            state.v_registers[V0 as usize] = 8;
            state.v_registers[V1 as usize] = 7;

            execution::execute_instruction(&mut state, 0x8015);

            assert_eq!(state.v_registers[V0 as usize], 1);
            assert_eq!(state.v_registers[VF as usize], 1);

            state.v_registers[V0 as usize] = 8;
            state.v_registers[V1 as usize] = 9;

            execution::execute_instruction(&mut state, 0x8015);

            assert_eq!(state.v_registers[V0 as usize], 255);
            assert_eq!(state.v_registers[VF as usize], 0);
        }

        //SUBCASE("SHR")
        {
            let mut state = cpu::create_chip8_state();

            state.v_registers[V0 as usize] = 8;

            execution::execute_instruction(&mut state, 0x8016);

            assert_eq!(state.v_registers[V0 as usize], 4);
            assert_eq!(state.v_registers[VF as usize], 0);

            execution::execute_instruction(&mut state, 0x8026);

            assert_eq!(state.v_registers[V0 as usize], 2);
            assert_eq!(state.v_registers[VF as usize], 0);

            execution::execute_instruction(&mut state, 0x8026);

            assert_eq!(state.v_registers[V0 as usize], 1);
            assert_eq!(state.v_registers[VF as usize], 0);

            execution::execute_instruction(&mut state, 0x8026);

            assert_eq!(state.v_registers[V0 as usize], 0);
            assert_eq!(state.v_registers[VF as usize], 1);
        }

        //SUBCASE("SUBN")
        {
            let mut state = cpu::create_chip8_state();

            state.v_registers[V0 as usize] = 7;
            state.v_registers[V1 as usize] = 8;

            execution::execute_instruction(&mut state, 0x8017);

            assert_eq!(state.v_registers[V0 as usize], 1);
            assert_eq!(state.v_registers[VF as usize], 1);

            state.v_registers[V0 as usize] = 2;
            state.v_registers[V1 as usize] = 1;

            execution::execute_instruction(&mut state, 0x8017);

            assert_eq!(state.v_registers[V0 as usize], 255);
            assert_eq!(state.v_registers[VF as usize], 0);
        }

        //SUBCASE("SHL")
        {
            let mut state = cpu::create_chip8_state();

            state.v_registers[V0 as usize] = 64;

            execution::execute_instruction(&mut state, 0x801E);

            assert_eq!(state.v_registers[V0 as usize], 128);
            assert_eq!(state.v_registers[VF as usize], 0);

            execution::execute_instruction(&mut state, 0x801E);

            assert_eq!(state.v_registers[V0 as usize], 0);
            assert_eq!(state.v_registers[VF as usize], 1);
        }

        //SUBCASE("SNE2")
        {
            let mut state = cpu::create_chip8_state();

            state.v_registers[V9 as usize] = 64;
            state.v_registers[VA as usize] = 64;

            execution::execute_instruction(&mut state, 0x99A0);

            assert_eq!(state.pc, cpu::MIN_PROGRAM_ADDRESS as u16 + 2);

            state.v_registers[VA as usize] = 0;
            execution::execute_instruction(&mut state, 0x99A0);

            assert_eq!(state.pc, cpu::MIN_PROGRAM_ADDRESS as u16 + 6);
        }

        //SUBCASE("LDI")
        {
            let mut state = cpu::create_chip8_state();

            execution::execute_instruction(&mut state, 0xA242);

            assert_eq!(state.i, 0x0242);
        }

        //SUBCASE("JP2")
        {
            let mut state = cpu::create_chip8_state();

            state.v_registers[V0 as usize] = 0x02;

            execution::execute_instruction(&mut state, 0xB240);

            assert_eq!(state.pc, 0x0242);
        }

        //SUBCASE("RND")
        {
            let mut state = cpu::create_chip8_state();

            execution::execute_instruction(&mut state, 0xC10F);

            assert_eq!(state.v_registers[V1 as usize] & !0x0F, 0);

            execution::execute_instruction(&mut state, 0xC1F0);

            assert_eq!(state.v_registers[V1 as usize] & !0xF0, 0);
        }

        //SUBCASE("DRW")
        {
            // TODO
            // execution::execute_instruction(&mut state, 0x00E0); // Clear screen
            // state.v_registers[V0 as usize] = 0x0F; // Set digit to print
            // state.v_registers[V1 as usize] = 0x00; // Set digit to print
            // execution::execute_instruction(&mut state, 0xF029); // Load digit sprite address
            // execution::execute_instruction(&mut state, 0xD115); // Draw sprite
            // for (int i = 0; i < 10; i++)
            // {
            //     chip8::write_screen_pixel(state, chip8::SCREEN_WIDTH - i - 1, chip8::SCREEN_HEIGHT - i - 1, 1);
            // }
        }

        //SUBCASE("SKP")
        {
            let mut state = cpu::create_chip8_state();

            state.v_registers[VA as usize] = 0x0F;
            state.key_state = 0x8000;

            execution::execute_instruction(&mut state, 0xEA9E);

            assert_eq!(state.pc, cpu::MIN_PROGRAM_ADDRESS as u16 + 4); // Skipped

            execution::execute_instruction(&mut state, 0xEB9E);

            assert_eq!(state.pc, cpu::MIN_PROGRAM_ADDRESS as u16 + 6); // Did not skip

        }

        //SUBCASE("SKNP")
        {
            let mut state = cpu::create_chip8_state();

            state.v_registers[VA as usize] = 0xF;
            state.key_state = 0x8000;

            execution::execute_instruction(&mut state, 0xEBA1);

            assert_eq!(state.pc, cpu::MIN_PROGRAM_ADDRESS as u16 + 4); // Skipped

            execution::execute_instruction(&mut state, 0xEAA1);

            assert_eq!(state.pc, cpu::MIN_PROGRAM_ADDRESS as u16 + 6); // Did not skip
        }

        //SUBCASE("LDT")
        {
            let mut state = cpu::create_chip8_state();

            state.delay_timer = 42;
            state.v_registers[V4 as usize] = 0;

            execution::execute_instruction(&mut state, 0xF407);

            assert_eq!(state.v_registers[V4 as usize], 42);
        }

        //SUBCASE("LDK")
        {
            let mut state = cpu::create_chip8_state();

            assert!(!state.is_waiting_for_key);
            assert_eq!(state.v_registers[V1 as usize], 0);

            execution::execute_instruction(&mut state, 0xF10A);

            assert!(state.is_waiting_for_key);
            assert_eq!(state.v_registers[V1 as usize], 0);

            keyboard::set_key_pressed(&mut state, 0xA, true);

            execution::execute_instruction(&mut state, 0xF10A);

            assert!(!state.is_waiting_for_key);
            assert_eq!(state.v_registers[V1 as usize], 0xA);
        }

        //SUBCASE("LDDT")
        {
            let mut state = cpu::create_chip8_state();

            state.v_registers[V5 as usize] = 66;

            execution::execute_instruction(&mut state, 0xF515);

            assert_eq!(state.delay_timer, 66);
        }

        //SUBCASE("LDST")
        {
            let mut state = cpu::create_chip8_state();

            state.v_registers[V6 as usize] = 33;

            execution::execute_instruction(&mut state, 0xF618);

            assert_eq!(state.sound_timer, 33);
        }

        //SUBCASE("ADDI")
        {
            let mut state = cpu::create_chip8_state();

            state.v_registers[V9 as usize] = 10;
            state.i = cpu::MIN_PROGRAM_ADDRESS as u16;

            execution::execute_instruction(&mut state, 0xF91E);

            assert_eq!(state.i, cpu::MIN_PROGRAM_ADDRESS as u16 + 10);
        }

        //SUBCASE("LDF")
        {
            let mut state = cpu::create_chip8_state();

            state.v_registers[V0 as usize] = 9;

            execution::execute_instruction(&mut state, 0xF029);

            assert_eq!(state.i, state.font_table_offsets[9]);

            state.v_registers[V0 as usize] = 0xF;

            execution::execute_instruction(&mut state, 0xF029);

            assert_eq!(state.i, state.font_table_offsets[0xF]);
        }

        //SUBCASE("LDB")
        {
            let mut state = cpu::create_chip8_state();

            state.i = cpu::MIN_PROGRAM_ADDRESS as u16;
            state.v_registers[V7 as usize] = 109;

            execution::execute_instruction(&mut state, 0xF733);

            assert_eq!(state.memory[state.i as usize + 0], 1);
            assert_eq!(state.memory[state.i as usize + 1], 0);
            assert_eq!(state.memory[state.i as usize + 2], 9);

            state.v_registers[V7 as usize] = 255;

            execution::execute_instruction(&mut state, 0xF733);

            assert_eq!(state.memory[state.i as usize + 0], 2);
            assert_eq!(state.memory[state.i as usize + 1], 5);
            assert_eq!(state.memory[state.i as usize + 2], 5);
        }

        //SUBCASE("LDAI")
        {
            let mut state = cpu::create_chip8_state();

            state.i = cpu::MIN_PROGRAM_ADDRESS as u16;
            state.memory[state.i as usize + 0] = 0xF4;
            state.memory[state.i as usize + 1] = 0x33;
            state.memory[state.i as usize + 2] = 0x82;
            state.memory[state.i as usize + 3] = 0x73;

            state.v_registers[V0 as usize] = 0xE4;
            state.v_registers[V1 as usize] = 0x23;
            state.v_registers[V2 as usize] = 0x00;

            execute_instruction_internal(&mut state, OpCode::LDAI{reg: V1 as u8});

            assert_eq!(state.memory[state.i as usize + 0], 0xE4);
            assert_eq!(state.memory[state.i as usize + 1], 0x23);
            assert_eq!(state.memory[state.i as usize + 2], 0x82);
            assert_eq!(state.memory[state.i as usize + 3], 0x73);
        }

        //SUBCASE("LDM")
        {
            let mut state = cpu::create_chip8_state();

            state.i = cpu::MIN_PROGRAM_ADDRESS as u16;
            state.v_registers[V0 as usize] = 0xF4;
            state.v_registers[V1 as usize] = 0x33;
            state.v_registers[V2 as usize] = 0x82;
            state.v_registers[V3 as usize] = 0x73;

            state.memory[state.i as usize + 0] = 0xE4;
            state.memory[state.i as usize + 1] = 0x23;
            state.memory[state.i as usize + 2] = 0x00;

            execute_instruction_internal(&mut state, OpCode::LDM{reg: V1 as u8});

            assert_eq!(state.v_registers[V0 as usize], 0xE4);
            assert_eq!(state.v_registers[V1 as usize], 0x23);
            assert_eq!(state.v_registers[V2 as usize], 0x82);
            assert_eq!(state.v_registers[V3 as usize], 0x73);
        }
    }
}
