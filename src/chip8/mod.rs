pub mod config;
pub mod cpu;
pub mod execution;

pub use self::config::*;
pub use self::cpu::*;
pub use self::execution::*;

mod instruction;
mod memory;

//use VRegisterName::*;
