use super::{AddressingMode, Instruction};

use AddressingMode::{
    Absolute, AbsoluteX, AbsoluteY, Immediate, Implied, IndirectX, IndirectY, ZeroPage, ZeroPageX,
    ZeroPageY,
};

use phf::phf_map;

// TODO: Implement the rest of the CPU instructions.
pub static INSTRUCTION_LOOKUP: phf::Map<[u8; 1], Instruction> = phf_map! {
    [0x00] => Instruction::new("BRK", 1, 7, Implied),

    [0xA9] => Instruction::new("LDA", 2, 2, Immediate),
    [0xA5] => Instruction::new("LDA", 2, 3, ZeroPage),
    [0xB5] => Instruction::new("LDA", 2, 4, ZeroPageX),
    [0xAD] => Instruction::new("LDA", 3, 4, Absolute),
    [0xBD] => Instruction::new("LDA", 3, 4, AbsoluteX),
    [0xB9] => Instruction::new("LDA", 3, 4, AbsoluteY),
    [0xA1] => Instruction::new("LDA", 2, 6, IndirectX),
    [0xB1] => Instruction::new("LDA", 2, 5, IndirectY),

    [0xA2] => Instruction::new("LDX", 2, 2, Immediate),
    [0xA6] => Instruction::new("LDX", 2, 3, ZeroPage),
    [0xB6] => Instruction::new("LDX", 2, 4, ZeroPageY),
    [0xAE] => Instruction::new("LDX", 3, 4, Absolute),
    [0xBE] => Instruction::new("LDX", 3, 4, AbsoluteY),

    [0xA0] => Instruction::new("LDY", 2, 2, Immediate),
    [0xA4] => Instruction::new("LDY", 2, 3, ZeroPage),
    [0xB4] => Instruction::new("LDY", 2, 4, ZeroPageX),
    [0xAC] => Instruction::new("LDY", 3, 4, Absolute),
    [0xBC] => Instruction::new("LDY", 3, 4, AbsoluteX),

    [0x85] => Instruction::new("STA", 2, 3, ZeroPage),
    [0x95] => Instruction::new("STA", 2, 4, ZeroPageX),
    [0x8D] => Instruction::new("STA", 3, 4, Absolute),
    [0x9D] => Instruction::new("STA", 3, 5, AbsoluteX),
    [0x99] => Instruction::new("STA", 3, 5, AbsoluteY),
    [0x81] => Instruction::new("STA", 2, 6, IndirectX),
    [0x91] => Instruction::new("STA", 2, 6, IndirectY),

    [0xAA] => Instruction::new("TAX", 1, 2, Implied),
    [0xE8] => Instruction::new("INX", 1, 2, Implied)
};
