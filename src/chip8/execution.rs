use super::{
    cpu,
    instruction,
    memory,
    opcode,
};

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
        let next_instruction = load_next_instruction(state);
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

    let instruction = opcode::decode_instruction(instruction);

    instruction::execute_instruction_internal(state, instruction);

    // Increment PC only if it was NOT overriden by an instruction,
    // or if we are waiting for user input.
    if pc_save == state.pc && !state.is_waiting_for_key {
        state.pc += 2;
    }

    // Save previous key state
    state.key_state_prev = state.key_state;
}
