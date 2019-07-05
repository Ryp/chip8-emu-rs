use super::cpu;

pub enum MemoryUsage
{
    Read,
    Write,
    Execute,
}

pub fn is_valid_memory_range(baseAddress: u16, sizeInBytes: usize, usage: MemoryUsage) -> bool
{
    assert!(sizeInBytes > 0); // Invalid address range size

    let baseAddress = baseAddress as usize;
    let endAddress = baseAddress + (sizeInBytes - 1);

    if endAddress < baseAddress {
        return false; // Overflow
    }

    match usage {
        MemoryUsage::Read => (endAddress <= cpu::MAX_PROGRAM_ADDRESS),
        MemoryUsage::Write | MemoryUsage::Execute => (baseAddress >= cpu::MIN_PROGRAM_ADDRESS && endAddress <= cpu::MAX_PROGRAM_ADDRESS),
    }
}
