use super::cpu;

pub enum MemoryUsage
{
    Read,
    Write,
    Execute,
}

pub fn is_valid_memory_range(baseAddress: u16, sizeInBytes: u16, usage: MemoryUsage) -> bool
{
    assert!(sizeInBytes > 0); // Invalid address range size

    let endAddress: u16 = baseAddress + (sizeInBytes - 1);

    if endAddress < baseAddress {
        return false; // Overflow
    }

    match usage {
        MemoryUsage::Read => (endAddress <= cpu::MaxProgramAddress),
        MemoryUsage::Write | MemoryUsage::Execute => (baseAddress >= cpu::MinProgramAddress as u16 && endAddress <= cpu::MaxProgramAddress),
    }
}
