use super::cpu;
use super::memory;
use super::config::EmuConfig;
use super::instruction::*;

use std::cmp::max;

pub fn load_program(state: &mut cpu::CPUState, program: Vec<u8>)
{
    let programSize = program.len();

    assert!((programSize & 0x0001) == 0); // Unaligned size
    assert!(memory::is_valid_memory_range(cpu::MinProgramAddress as u16, programSize as u16, memory::MemoryUsage::Write));

    let rangeBegin = cpu::MinProgramAddress;
    let rangeEnd = cpu::MinProgramAddress + programSize;

    state.memory[rangeBegin..rangeEnd].clone_from_slice(&program[..]);
}

pub fn load_next_instruction(state: &cpu::CPUState) -> u16
{
    let pc = state.pc as usize;
    let byte0 = state.memory[pc + 0] as u32;
    let byte1 = state.memory[pc + 1] as u32;

    // Load big endian
    let instruction = byte0 | byte1 << 8;

    instruction as u16
}

pub fn execute_step(config: &EmuConfig, state: &mut cpu::CPUState, deltaTimeMs: u32)
{
    let mut instructionsToExecute: u32 = 0;

    update_timers(state, &mut instructionsToExecute, deltaTimeMs);

    for i in 0..instructionsToExecute
    {
        // Simulate logic
        let nextInstruction: u16 = load_next_instruction(state);
        execute_instruction(config, state, nextInstruction);
    }
}

fn update_timers(state: &mut cpu::CPUState, executionCounter: &mut u32, deltaTimeMs: u32)
{
    // Update delay timer
    state.delayTimerAccumulator += deltaTimeMs;

    let delayTimerDecrement: u32 = state.delayTimerAccumulator / cpu::DelayTimerPeriodMs;
    state.delayTimer = max(0, state.delayTimer as i32 - delayTimerDecrement as i32) as u8; // TODO maybe there's a cast error here

    // Remove accumulated ticks
    state.delayTimerAccumulator = state.delayTimerAccumulator % cpu::DelayTimerPeriodMs;

    // Update execution counter
    state.executionTimerAccumulator += deltaTimeMs;

    *executionCounter = state.executionTimerAccumulator / cpu::InstructionExecutionPeriodMs;
    state.executionTimerAccumulator = state.executionTimerAccumulator % cpu::InstructionExecutionPeriodMs;

    // TODO Handle sound
    if state.soundTimer > 0 {
        state.soundTimer += 1;
    }
}

fn execute_instruction(config: &EmuConfig, state: &mut cpu::CPUState, instruction: u16)
{
    // Save PC for later
    let pcSave = state.pc;

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
        let registerName = ((instruction & 0x0F00) >> 8) as u8;
        let value = (instruction & 0x00FF) as u8;

        execute_se(state, registerName, value);
    }
    else if (instruction & !0x0FFF) == 0x4000 {
        // 4xkk - SNE Vx, byte
        let registerName = ((instruction & 0x0F00) >> 8) as u8;
        let value = (instruction & 0x00FF) as u8;

        execute_sne(state, registerName, value);
    }
    else if (instruction & !0x0FF0) == 0x5000 {
        // 5xy0 - SE Vx, Vy
        let registerLHS = ((instruction & 0x0F00) >> 8) as u8;
        let registerRHS = ((instruction & 0x00F0) >> 4) as u8;

        execute_se2(state, registerLHS, registerRHS);
    }
    else if (instruction & !0x0FFF) == 0x6000 {
        // 6xkk - LD Vx, byte
        let registerName = ((instruction & 0x0F00) >> 8) as u8;
        let value = (instruction & 0x00FF) as u8;

        execute_ld(state, registerName, value);
    }
    else if (instruction & !0x0FFF) == 0x7000 {
        // 7xkk - ADD Vx, byte
        let registerName = ((instruction & 0x0F00) >> 8) as u8;
        let value = (instruction & 0x00FF) as u8;

        execute_add(state, registerName, value);
    }
    else if (instruction & !0x0FF0) == 0x8000 {
        // 8xy0 - LD Vx, Vy
        let registerLHS = ((instruction & 0x0F00) >> 8) as u8;
        let registerRHS = ((instruction & 0x00F0) >> 4) as u8;

        execute_ld2(state, registerLHS, registerRHS);
    }
    else if (instruction & !0x0FF0) == 0x8001 {
        // 8xy1 - OR Vx, Vy
        let registerLHS = ((instruction & 0x0F00) >> 8) as u8;
        let registerRHS = ((instruction & 0x00F0) >> 4) as u8;

        execute_or(state, registerLHS, registerRHS);
    }
    else if (instruction & !0x0FF0) == 0x8002 {
        // 8xy2 - AND Vx, Vy
        let registerLHS = ((instruction & 0x0F00) >> 8) as u8;
        let registerRHS = ((instruction & 0x00F0) >> 4) as u8;

        execute_and(state, registerLHS, registerRHS);
    }
    else if (instruction & !0x0FF0) == 0x8003 {
        // 8xy3 - XOR Vx, Vy
        let registerLHS = ((instruction & 0x0F00) >> 8) as u8;
        let registerRHS = ((instruction & 0x00F0) >> 4) as u8;

        execute_xor(state, registerLHS, registerRHS);
    }
    else if (instruction & !0x0FF0) == 0x8004 {
        // 8xy4 - ADD Vx, Vy
        let registerLHS = ((instruction & 0x0F00) >> 8) as u8;
        let registerRHS = ((instruction & 0x00F0) >> 4) as u8;

        execute_add2(state, registerLHS, registerRHS);
    }
    else if (instruction & !0x0FF0) == 0x8005 {
        // 8xy5 - SUB Vx, Vy
        let registerLHS = ((instruction & 0x0F00) >> 8) as u8;
        let registerRHS = ((instruction & 0x00F0) >> 4) as u8;

        execute_sub(state, registerLHS, registerRHS);
    }
    else if (instruction & !0x0FF0) == 0x8006 {
        // 8xy6 - SHR Vx {, Vy}
        let registerLHS = ((instruction & 0x0F00) >> 8) as u8;
        let registerRHS = ((instruction & 0x00F0) >> 4) as u8;

        execute_shr1(state, registerLHS, registerRHS);
    }
    else if (instruction & !0x0FF0) == 0x8007 {
        // 8xy7 - SUBN Vx, Vy
        let registerLHS = ((instruction & 0x0F00) >> 8) as u8;
        let registerRHS = ((instruction & 0x00F0) >> 4) as u8;

        execute_subn(state, registerLHS, registerRHS);
    }
    else if (instruction & !0x0FF0) == 0x800E {
        // 8xyE - SHL Vx {, Vy}
        let registerLHS = ((instruction & 0x0F00) >> 8) as u8;
        let registerRHS = ((instruction & 0x00F0) >> 4) as u8;

        execute_shl1(state, registerLHS, registerRHS);
    }
    else if (instruction & !0x0FF0) == 0x9000 {
        // 9xy0 - SNE Vx, Vy
        let registerLHS = ((instruction & 0x0F00) >> 8) as u8;
        let registerRHS = ((instruction & 0x00F0) >> 4) as u8;

        execute_sne2(state, registerLHS, registerRHS);
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
        let registerName = ((instruction & 0x0F00) >> 8) as u8;
        let value = (instruction & 0x00FF) as u8;

        execute_rnd(state, registerName, value);
    }
    else if (instruction & !0x0FFF) == 0xD000 {
        // Dxyn - DRW Vx, Vy, nibble
        let registerLHS = ((instruction & 0x0F00) >> 8) as u8;
        let registerRHS = ((instruction & 0x00F0) >> 4) as u8;
        let size = (instruction & 0x000F) as u8;

        execute_drw(state, registerLHS, registerRHS, size);
    }
    else if (instruction & !0x0F00) == 0xE09E {
        // Ex9E - SKP Vx
        let registerName = ((instruction & 0x0F00) >> 8) as u8;

        execute_skp(state, registerName);
    }
    else if (instruction & !0x0F00) == 0xE0A1 {
        // ExA1 - SKNP Vx
        let registerName = ((instruction & 0x0F00) >> 8) as u8;

        execute_sknp(state, registerName);
    }
    else if (instruction & !0x0F00) == 0xF007 {
        // Fx07 - LD Vx, DT
        let registerName = ((instruction & 0x0F00) >> 8) as u8;

        execute_ldt(state, registerName);
    }
    else if (instruction & !0x0F00) == 0xF00A {
        // Fx0A - LD Vx, K
        let registerName = ((instruction & 0x0F00) >> 8) as u8;

        execute_ldk(state, registerName);
    }
    else if (instruction & !0x0F00) == 0xF015 {
        // Fx15 - LD DT, Vx
        let registerName = ((instruction & 0x0F00) >> 8) as u8;

        execute_lddt(state, registerName);
    }
    else if (instruction & !0x0F00) == 0xF018 {
        // Fx18 - LD ST, Vx
        let registerName = ((instruction & 0x0F00) >> 8) as u8;

        execute_ldst(state, registerName);
    }
    else if (instruction & !0x0F00) == 0xF01E {
        // Fx1E - ADD I, Vx
        let registerName = ((instruction & 0x0F00) >> 8) as u8;

        execute_addi(state, registerName);
    }
    else if (instruction & !0x0F00) == 0xF029 {
        // Fx29 - LD F, Vx
        let registerName = ((instruction & 0x0F00) >> 8) as u8;

        execute_ldf(state, registerName);
    }
    else if (instruction & !0x0F00) == 0xF033 {
        // Fx33 - LD B, Vx
        let registerName = ((instruction & 0x0F00) >> 8) as u8;

        execute_ldb(state, registerName);
    }
    else if (instruction & !0x0F00) == 0xF055 {
        // Fx55 - LD [I], Vx
        let registerName = ((instruction & 0x0F00) >> 8) as u8;

        execute_ldai(state, registerName);
    }
    else if (instruction & !0x0F00) == 0xF065 {
        // Fx65 - LD Vx, [I]
        let registerName = ((instruction & 0x0F00) >> 8) as u8;

        execute_ldm(state, registerName);
    }
    else
    {
        assert!(false); // Unknown instruction
    }

    // Increment PC only if it was NOT overriden by an instruction,
    // or if we are waiting for user input.
    if pcSave == state.pc && !state.isWaitingForKey {
        state.pc += 2;
    }

    // Save previous key state
    state.keyStatePrev = state.keyState;
}
