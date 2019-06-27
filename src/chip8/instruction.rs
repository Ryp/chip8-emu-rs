use super::cpu::CPUState;

pub fn execute_cls(state: &mut CPUState){}
pub fn execute_ret(state: &mut CPUState){}
pub fn execute_sys(state: &mut CPUState, address: u16){}
pub fn execute_jp(state: &mut CPUState, address: u16){}
pub fn execute_call(state: &mut CPUState, address: u16){}
pub fn execute_se(state: &mut CPUState, registerName: u8, value: u8){}
pub fn execute_sne(state: &mut CPUState, registerName: u8, value: u8){}
pub fn execute_se2(state: &mut CPUState, registerLHS: u8, registerRHS: u8){}
pub fn execute_ld(state: &mut CPUState, registerName: u8, value: u8){}
pub fn execute_add(state: &mut CPUState, registerName: u8, value: u8){}
pub fn execute_ld2(state: &mut CPUState, registerLHS: u8, registerRHS: u8){}
pub fn execute_or(state: &mut CPUState, registerLHS: u8, registerRHS: u8){}
pub fn execute_and(state: &mut CPUState, registerLHS: u8, registerRHS: u8){}
pub fn execute_xor(state: &mut CPUState, registerLHS: u8, registerRHS: u8){}
pub fn execute_add2(state: &mut CPUState, registerLHS: u8, registerRHS: u8){}
pub fn execute_sub(state: &mut CPUState, registerLHS: u8, registerRHS: u8){}
pub fn execute_shr1(state: &mut CPUState, registerLHS: u8, registerRHS: u8){}
pub fn execute_subn(state: &mut CPUState, registerLHS: u8, registerRHS: u8){}
pub fn execute_shl1(state: &mut CPUState, registerLHS: u8, registerRHS: u8){}
pub fn execute_sne2(state: &mut CPUState, registerLHS: u8, registerRHS: u8){}
pub fn execute_ldi(state: &mut CPUState, address: u16){}
pub fn execute_jp2(state: &mut CPUState, baseAddress: u16){}
pub fn execute_rnd(state: &mut CPUState, registerName: u8, value: u8){}
pub fn execute_drw(state: &mut CPUState, registerLHS: u8, registerRHS: u8, size: u8){}
pub fn execute_skp(state: &mut CPUState, registerName: u8){}
pub fn execute_sknp(state: &mut CPUState, registerName: u8){}
pub fn execute_ldt(state: &mut CPUState, registerName: u8){}
pub fn execute_ldk(state: &mut CPUState, registerName: u8){}
pub fn execute_lddt(state: &mut CPUState, registerName: u8){}
pub fn execute_ldst(state: &mut CPUState, registerName: u8){}
pub fn execute_addi(state: &mut CPUState, registerName: u8){}
pub fn execute_ldf(state: &mut CPUState, registerName: u8){}
pub fn execute_ldb(state: &mut CPUState, registerName: u8){}
pub fn execute_ldai(state: &mut CPUState, registerName: u8){}
pub fn execute_ldm(state: &mut CPUState, registerName: u8){}
