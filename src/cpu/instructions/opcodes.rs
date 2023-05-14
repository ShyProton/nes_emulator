use super::{AddressingMode, Instruction};

use phf::phf_map;

// TODO: Implement the rest of the CPU instructions.
pub static INSTRUCTION_LOOKUP: phf::Map<[u8; 1], Instruction> = phf_map! {
    [0x00] => Instruction::new("BRK", 1, 7, AddressingMode::Implied),

    [0xA9] => Instruction::new("LDA", 2, 2, AddressingMode::Immediate),
    [0xA5] => Instruction::new("LDA", 2, 3, AddressingMode::ZeroPage),

    [0xA2] => Instruction::new("LDX", 2, 2, AddressingMode::Immediate),

    [0x85] => Instruction::new("STA", 2, 3, AddressingMode::ZeroPage),

    [0xAA] => Instruction::new("TAX", 1, 2, AddressingMode::Implied),
    [0xE8] => Instruction::new("INX", 1, 2, AddressingMode::Implied)
};
