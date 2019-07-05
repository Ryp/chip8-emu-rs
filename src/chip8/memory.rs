use super::cpu;

pub enum MemoryUsage
{
    Read,
    Write,
    Execute,
}

pub fn is_valid_memory_range(base_address: u16, size_in_bytes: usize, usage: MemoryUsage) -> bool
{
    assert!(size_in_bytes > 0); // Invalid address range size

    let base_address = base_address as usize;
    let end_address = base_address + (size_in_bytes - 1);

    if end_address < base_address {
        return false; // Overflow
    }

    match usage {
        MemoryUsage::Read => (end_address <= cpu::MAX_PROGRAM_ADDRESS),
        MemoryUsage::Write | MemoryUsage::Execute => (base_address >= cpu::MIN_PROGRAM_ADDRESS && end_address <= cpu::MAX_PROGRAM_ADDRESS),
    }
}
