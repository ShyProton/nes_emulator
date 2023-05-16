use super::{AddressingMode, Cpu, Instruction};

pub use aliases::InstructionAlias;
pub use lookup::INSTRUCTION_LOOKUP;

mod aliases;
mod lookup;
mod method_map;
