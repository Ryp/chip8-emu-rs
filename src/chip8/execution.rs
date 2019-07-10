use super::cpu;
use super::memory;
use super::instruction::*;

use std::cmp::max;

pub fn load_program(state: &mut cpu::CPUState, program: Vec<u8>)
{
    let program_size = program.len();

    assert!((program_size & 0x0001) == 0); // Unaligned size
    assert!(memory::is_valid_memory_range(cpu::MIN_PROGRAM_ADDRESS as u16, program_size, memory::MemoryUsage::Write));

    let range_begin = cpu::MIN_PROGRAM_ADDRESS;
    let range_end = cpu::MIN_PROGRAM_ADDRESS + program_size;

    state.memory[range_begin..range_end].clone_from_slice(&program[..]);
}

pub fn load_next_instruction(state: &cpu::CPUState) -> u16
{
    let pc = state.pc as usize;
    let byte0 = u32::from(state.memory[pc]);
    let byte1 = u32::from(state.memory[pc + 1]);

    // Load big endian
    let instruction = byte0 << 8 | byte1;

    instruction as u16
}

pub fn execute_step(state: &mut cpu::CPUState, delta_time_ms: u32)
{
    let mut instructions_to_execute: u32 = 0;

    update_timers(state, &mut instructions_to_execute, delta_time_ms);

    for _ in 0..instructions_to_execute
    {
        // Simulate logic
        let next_instruction: u16 = load_next_instruction(state);
        execute_instruction(state, next_instruction);
    }
}

fn update_timers(state: &mut cpu::CPUState, execution_counter: &mut u32, delta_time_ms: u32)
{
    // Update delay timer
    state.delay_timer_accumulator += delta_time_ms;

    let delay_timer_decrement: u32 = state.delay_timer_accumulator / cpu::DELAY_TIMER_PERIOD_MS;
    state.delay_timer = max(0, i32::from(state.delay_timer) - delay_timer_decrement as i32) as u8; // TODO maybe there's a cast error here

    // Remove accumulated ticks
    state.delay_timer_accumulator %= cpu::DELAY_TIMER_PERIOD_MS;

    // Update execution counter
    state.execution_timer_accumulator += delta_time_ms;

    *execution_counter = state.execution_timer_accumulator / cpu::INSTRUCTION_EXECUTION_PERIOD_MS;
    state.execution_timer_accumulator %= cpu::INSTRUCTION_EXECUTION_PERIOD_MS;

    // TODO Handle sound
    if state.sound_timer > 0 {
        state.sound_timer += 1;
    }
}

pub fn execute_instruction(state: &mut cpu::CPUState, instruction: u16)
{
    // Save PC for later
    let pc_save = state.pc;

    // Decode and execute
    if instruction == 0x00E0 {
        // 00E0 - CLS
        execute_cls(state);
    }
    else if instruction == 0x00EE {
        // 00EE - RET
        execute_ret(state);
    }
    else if (instruction & !0x0FFF) == 0x0000 {
        // 0nnn - SYS addr
        let address = instruction & 0x0FFF;

        execute_sys(state, address);
    }
    else if (instruction & !0x0FFF) == 0x1000 {
        // 1nnn - JP addr
        let address = instruction & 0x0FFF;

        execute_jp(state, address);
    }
    else if (instruction & !0x0FFF) == 0x2000 {
        // 2nnn - CALL addr
        let address = instruction & 0x0FFF;

        execute_call(state, address);
    }
    else if (instruction & !0x0FFF) == 0x3000 {
        // 3xkk - SE Vx, byte
        let register_name = ((instruction & 0x0F00) >> 8) as u8;
        let value = (instruction & 0x00FF) as u8;

        execute_se(state, register_name, value);
    }
    else if (instruction & !0x0FFF) == 0x4000 {
        // 4xkk - SNE Vx, byte
        let register_name = ((instruction & 0x0F00) >> 8) as u8;
        let value = (instruction & 0x00FF) as u8;

        execute_sne(state, register_name, value);
    }
    else if (instruction & !0x0FF0) == 0x5000 {
        // 5xy0 - SE Vx, Vy
        let register_lhs = ((instruction & 0x0F00) >> 8) as u8;
        let register_rhs = ((instruction & 0x00F0) >> 4) as u8;

        execute_se2(state, register_lhs, register_rhs);
    }
    else if (instruction & !0x0FFF) == 0x6000 {
        // 6xkk - LD Vx, byte
        let register_name = ((instruction & 0x0F00) >> 8) as u8;
        let value = (instruction & 0x00FF) as u8;

        execute_ld(state, register_name, value);
    }
    else if (instruction & !0x0FFF) == 0x7000 {
        // 7xkk - ADD Vx, byte
        let register_name = ((instruction & 0x0F00) >> 8) as u8;
        let value = (instruction & 0x00FF) as u8;

        execute_add(state, register_name, value);
    }
    else if (instruction & !0x0FF0) == 0x8000 {
        // 8xy0 - LD Vx, Vy
        let register_lhs = ((instruction & 0x0F00) >> 8) as u8;
        let register_rhs = ((instruction & 0x00F0) >> 4) as u8;

        execute_ld2(state, register_lhs, register_rhs);
    }
    else if (instruction & !0x0FF0) == 0x8001 {
        // 8xy1 - OR Vx, Vy
        let register_lhs = ((instruction & 0x0F00) >> 8) as u8;
        let register_rhs = ((instruction & 0x00F0) >> 4) as u8;

        execute_or(state, register_lhs, register_rhs);
    }
    else if (instruction & !0x0FF0) == 0x8002 {
        // 8xy2 - AND Vx, Vy
        let register_lhs = ((instruction & 0x0F00) >> 8) as u8;
        let register_rhs = ((instruction & 0x00F0) >> 4) as u8;

        execute_and(state, register_lhs, register_rhs);
    }
    else if (instruction & !0x0FF0) == 0x8003 {
        // 8xy3 - XOR Vx, Vy
        let register_lhs = ((instruction & 0x0F00) >> 8) as u8;
        let register_rhs = ((instruction & 0x00F0) >> 4) as u8;

        execute_xor(state, register_lhs, register_rhs);
    }
    else if (instruction & !0x0FF0) == 0x8004 {
        // 8xy4 - ADD Vx, Vy
        let register_lhs = ((instruction & 0x0F00) >> 8) as u8;
        let register_rhs = ((instruction & 0x00F0) >> 4) as u8;

        execute_add2(state, register_lhs, register_rhs);
    }
    else if (instruction & !0x0FF0) == 0x8005 {
        // 8xy5 - SUB Vx, Vy
        let register_lhs = ((instruction & 0x0F00) >> 8) as u8;
        let register_rhs = ((instruction & 0x00F0) >> 4) as u8;

        execute_sub(state, register_lhs, register_rhs);
    }
    else if (instruction & !0x0FF0) == 0x8006 {
        // 8xy6 - SHR Vx {, Vy}
        let register_lhs = ((instruction & 0x0F00) >> 8) as u8;
        let register_rhs = ((instruction & 0x00F0) >> 4) as u8;

        execute_shr1(state, register_lhs, register_rhs);
    }
    else if (instruction & !0x0FF0) == 0x8007 {
        // 8xy7 - SUBN Vx, Vy
        let register_lhs = ((instruction & 0x0F00) >> 8) as u8;
        let register_rhs = ((instruction & 0x00F0) >> 4) as u8;

        execute_subn(state, register_lhs, register_rhs);
    }
    else if (instruction & !0x0FF0) == 0x800E {
        // 8xyE - SHL Vx {, Vy}
        let register_lhs = ((instruction & 0x0F00) >> 8) as u8;
        let register_rhs = ((instruction & 0x00F0) >> 4) as u8;

        execute_shl1(state, register_lhs, register_rhs);
    }
    else if (instruction & !0x0FF0) == 0x9000 {
        // 9xy0 - SNE Vx, Vy
        let register_lhs = ((instruction & 0x0F00) >> 8) as u8;
        let register_rhs = ((instruction & 0x00F0) >> 4) as u8;

        execute_sne2(state, register_lhs, register_rhs);
    }
    else if (instruction & !0x0FFF) == 0xA000 {
        // Annn - LD I, addr
        let address = instruction & 0x0FFF;

        execute_ldi(state, address);
    }
    else if (instruction & !0x0FFF) == 0xB000 {
        // Bnnn - JP V0, addr
        let address = instruction & 0x0FFF;

        execute_jp2(state, address);
    }
    else if (instruction & !0x0FFF) == 0xC000 {
        // Cxkk - RND Vx, byte
        let register_name = ((instruction & 0x0F00) >> 8) as u8;
        let value = (instruction & 0x00FF) as u8;

        execute_rnd(state, register_name, value);
    }
    else if (instruction & !0x0FFF) == 0xD000 {
        // Dxyn - DRW Vx, Vy, nibble
        let register_lhs = ((instruction & 0x0F00) >> 8) as u8;
        let register_rhs = ((instruction & 0x00F0) >> 4) as u8;
        let size = (instruction & 0x000F) as u8;

        execute_drw(state, register_lhs, register_rhs, size);
    }
    else if (instruction & !0x0F00) == 0xE09E {
        // Ex9E - SKP Vx
        let register_name = ((instruction & 0x0F00) >> 8) as u8;

        execute_skp(state, register_name);
    }
    else if (instruction & !0x0F00) == 0xE0A1 {
        // ExA1 - SKNP Vx
        let register_name = ((instruction & 0x0F00) >> 8) as u8;

        execute_sknp(state, register_name);
    }
    else if (instruction & !0x0F00) == 0xF007 {
        // Fx07 - LD Vx, DT
        let register_name = ((instruction & 0x0F00) >> 8) as u8;

        execute_ldt(state, register_name);
    }
    else if (instruction & !0x0F00) == 0xF00A {
        // Fx0A - LD Vx, K
        let register_name = ((instruction & 0x0F00) >> 8) as u8;

        execute_ldk(state, register_name);
    }
    else if (instruction & !0x0F00) == 0xF015 {
        // Fx15 - LD DT, Vx
        let register_name = ((instruction & 0x0F00) >> 8) as u8;

        execute_lddt(state, register_name);
    }
    else if (instruction & !0x0F00) == 0xF018 {
        // Fx18 - LD ST, Vx
        let register_name = ((instruction & 0x0F00) >> 8) as u8;

        execute_ldst(state, register_name);
    }
    else if (instruction & !0x0F00) == 0xF01E {
        // Fx1E - ADD I, Vx
        let register_name = ((instruction & 0x0F00) >> 8) as u8;

        execute_addi(state, register_name);
    }
    else if (instruction & !0x0F00) == 0xF029 {
        // Fx29 - LD F, Vx
        let register_name = ((instruction & 0x0F00) >> 8) as u8;

        execute_ldf(state, register_name);
    }
    else if (instruction & !0x0F00) == 0xF033 {
        // Fx33 - LD B, Vx
        let register_name = ((instruction & 0x0F00) >> 8) as u8;

        execute_ldb(state, register_name);
    }
    else if (instruction & !0x0F00) == 0xF055 {
        // Fx55 - LD [I], Vx
        let register_name = ((instruction & 0x0F00) >> 8) as u8;

        execute_ldai(state, register_name);
    }
    else if (instruction & !0x0F00) == 0xF065 {
        // Fx65 - LD Vx, [I]
        let register_name = ((instruction & 0x0F00) >> 8) as u8;

        execute_ldm(state, register_name);
    } else {
        unreachable!(); // Unknown instruction
    }

    // Increment PC only if it was NOT overriden by an instruction,
    // or if we are waiting for user input.
    if pc_save == state.pc && !state.is_waiting_for_key {
        state.pc += 2;
    }

    // Save previous key state
    state.key_state_prev = state.key_state;
}
