use super::{
    AddressingMode::{
        Absolute, AbsoluteX, AbsoluteY, Immediate, Implied, IndirectX, IndirectY, ZeroPage,
        ZeroPageX, ZeroPageY,
    },
    Instruction,
    InstructionAlias::{
        ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS, CLC, CLD, CLI, CLV, CMP,
        CPX, CPY, DEC, DEX, DEY, EOR, INC, INX, INY, JMP, JSR, LDA, LDX, LDY, LSR, NOP, ORA, PHA,
        PHP, PLA, PLP, ROL, ROR, RTI, RTS, SBC, SEC, SED, SEI, STA, STX, STY, TAX, TAY, TSX, TXA,
        TXS, TYA,
    },
};

use phf::phf_map;

pub static INSTRUCTION_LOOKUP: phf::Map<[u8; 1], Instruction> = phf_map! {
    [0x69] => Instruction::new(ADC, 2, 2, Implied),
    [0x65] => Instruction::new(ADC, 2, 3, ZeroPage),
    [0x75] => Instruction::new(ADC, 2, 4, ZeroPageX),
    [0x6D] => Instruction::new(ADC, 3, 4, Absolute),
    [0x7D] => Instruction::new(ADC, 3, 4, AbsoluteX),
    [0x79] => Instruction::new(ADC, 3, 4, AbsoluteY),
    [0x61] => Instruction::new(ADC, 2, 6, IndirectX),
    [0x71] => Instruction::new(ADC, 2, 5, IndirectY),

    [0x29] => Instruction::new(AND, 2, 2, Implied),
    [0x25] => Instruction::new(AND, 2, 3, ZeroPage),
    [0x35] => Instruction::new(AND, 2, 4, ZeroPageX),
    [0x2D] => Instruction::new(AND, 3, 4, Absolute),
    [0x3D] => Instruction::new(AND, 3, 4, AbsoluteX),
    [0x39] => Instruction::new(AND, 3, 4, AbsoluteY),
    [0x21] => Instruction::new(AND, 2, 6, IndirectX),
    [0x31] => Instruction::new(AND, 2, 5, IndirectY),

    [0x0A] => Instruction::new(ASL, 1, 2, Implied), // Accumulator
    [0x06] => Instruction::new(ASL, 2, 5, ZeroPage),
    [0x16] => Instruction::new(ASL, 2, 6, ZeroPageX),
    [0x0E] => Instruction::new(ASL, 3, 6, Absolute),
    [0x1E] => Instruction::new(ASL, 3, 7, AbsoluteX),

    [0x90] => Instruction::new(BCC, 2, 2, Implied), // Relative

    [0xB0] => Instruction::new(BCS, 2, 2, Implied), // Relative

    [0xF0] => Instruction::new(BEQ, 2, 2, Implied), // Relative

    [0x24] => Instruction::new(BIT, 2, 3, ZeroPage),
    [0x2C] => Instruction::new(BIT, 3, 4, Absolute),

    [0x30] => Instruction::new(BMI, 2, 2, Implied), // Relative

    [0xD0] => Instruction::new(BNE, 2, 2, Implied), // Relative

    [0x10] => Instruction::new(BPL, 2, 2, Implied), // Relative

    [0x00] => Instruction::new(BRK, 1, 7, Implied),

    [0x50] => Instruction::new(BVC, 2, 2, Implied), // Relative

    [0x70] => Instruction::new(BVS, 2, 2, Implied), // Relative

    [0x18] => Instruction::new(CLC, 1, 2, Implied),

    [0xD8] => Instruction::new(CLD, 1, 2, Implied),

    [0x58] => Instruction::new(CLI, 1, 2, Implied),

    [0xB8] => Instruction::new(CLV, 1, 2, Implied),

    [0xC9] => Instruction::new(CMP, 2, 2, Implied),
    [0xC5] => Instruction::new(CMP, 2, 3, ZeroPage),
    [0xD5] => Instruction::new(CMP, 2, 4, ZeroPageX),
    [0xCD] => Instruction::new(CMP, 3, 4, Absolute),
    [0xDD] => Instruction::new(CMP, 3, 4, AbsoluteX),
    [0xD9] => Instruction::new(CMP, 3, 4, AbsoluteY),
    [0xC1] => Instruction::new(CMP, 2, 6, IndirectX),
    [0xD1] => Instruction::new(CMP, 2, 5, IndirectY),

    [0xE0] => Instruction::new(CPX, 2, 2, Immediate),
    [0xE4] => Instruction::new(CPX, 2, 3, ZeroPage),
    [0xEC] => Instruction::new(CPX, 3, 4, Absolute),

    [0xC0] => Instruction::new(CPY, 2, 2, Immediate),
    [0xC4] => Instruction::new(CPY, 2, 3, ZeroPage),
    [0xCC] => Instruction::new(CPY, 3, 4, Absolute),

    [0xC6] => Instruction::new(DEC, 2, 5, ZeroPage),
    [0xD6] => Instruction::new(DEC, 2, 6, ZeroPageX),
    [0xCE] => Instruction::new(DEC, 3, 6, Absolute),
    [0xDE] => Instruction::new(DEC, 3, 7, AbsoluteX),

    [0xCA] => Instruction::new(DEX, 1, 2, Implied),

    [0x88] => Instruction::new(DEY, 1, 2, Implied),

    [0x49] => Instruction::new(EOR, 2, 2, Implied),
    [0x45] => Instruction::new(EOR, 2, 3, ZeroPage),
    [0x55] => Instruction::new(EOR, 2, 4, ZeroPageX),
    [0x4D] => Instruction::new(EOR, 3, 4, Absolute),
    [0x5D] => Instruction::new(EOR, 3, 4, AbsoluteX),
    [0x59] => Instruction::new(EOR, 3, 4, AbsoluteY),
    [0x41] => Instruction::new(EOR, 2, 6, IndirectX),
    [0x51] => Instruction::new(EOR, 2, 5, IndirectY),

    [0xE6] => Instruction::new(INC, 2, 5, ZeroPage),
    [0xF6] => Instruction::new(INC, 2, 5, ZeroPageX),
    [0xEE] => Instruction::new(INC, 2, 5, Absolute),
    [0xFE] => Instruction::new(INC, 2, 5, AbsoluteX),

    [0xE8] => Instruction::new(INX, 1, 2, Implied),

    [0xC8] => Instruction::new(INY, 1, 2, Implied),

    [0x4C] => Instruction::new(JMP, 3, 3, Absolute),
    [0x6C] => Instruction::new(JMP, 3, 5, Implied), // Indirect

    [0x20] => Instruction::new(JSR, 3, 6, Absolute),

    [0xA9] => Instruction::new(LDA, 2, 2, Immediate),
    [0xA5] => Instruction::new(LDA, 2, 3, ZeroPage),
    [0xB5] => Instruction::new(LDA, 2, 4, ZeroPageX),
    [0xAD] => Instruction::new(LDA, 3, 4, Absolute),
    [0xBD] => Instruction::new(LDA, 3, 4, AbsoluteX),
    [0xB9] => Instruction::new(LDA, 3, 4, AbsoluteY),
    [0xA1] => Instruction::new(LDA, 2, 6, IndirectX),
    [0xB1] => Instruction::new(LDA, 2, 5, IndirectY),

    [0xA2] => Instruction::new(LDX, 2, 2, Immediate),
    [0xA6] => Instruction::new(LDX, 2, 3, ZeroPage),
    [0xB6] => Instruction::new(LDX, 2, 4, ZeroPageY),
    [0xAE] => Instruction::new(LDX, 3, 4, Absolute),
    [0xBE] => Instruction::new(LDX, 3, 4, AbsoluteY),

    [0xA0] => Instruction::new(LDY, 2, 2, Immediate),
    [0xA4] => Instruction::new(LDY, 2, 3, ZeroPage),
    [0xB4] => Instruction::new(LDY, 2, 4, ZeroPageX),
    [0xAC] => Instruction::new(LDY, 3, 4, Absolute),
    [0xBC] => Instruction::new(LDY, 3, 4, AbsoluteX),

    [0x4A] => Instruction::new(LSR, 1, 2, Implied), // Accumulator
    [0x46] => Instruction::new(LSR, 2, 5, ZeroPage),
    [0x56] => Instruction::new(LSR, 2, 6, ZeroPageX),
    [0x4E] => Instruction::new(LSR, 3, 6, Absolute),
    [0x5E] => Instruction::new(LSR, 3, 7, AbsoluteX),

    [0xEA] => Instruction::new(NOP, 1, 2, Implied),

    [0x09] => Instruction::new(ORA, 2, 2, Immediate),
    [0x05] => Instruction::new(ORA, 2, 3, ZeroPage),
    [0x15] => Instruction::new(ORA, 2, 4, ZeroPageX),
    [0x0D] => Instruction::new(ORA, 3, 4, Absolute),
    [0x1D] => Instruction::new(ORA, 3, 4, AbsoluteX),
    [0x19] => Instruction::new(ORA, 3, 4, AbsoluteY),
    [0x01] => Instruction::new(ORA, 2, 6, IndirectX),
    [0x11] => Instruction::new(ORA, 2, 5, IndirectY),

    [0x48] => Instruction::new(PHA, 1, 3, Implied),

    [0x08] => Instruction::new(PHP, 1, 3, Implied),

    [0x68] => Instruction::new(PLA, 1, 4, Implied),

    [0x28] => Instruction::new(PLP, 1, 4, Implied),

    [0x2A] => Instruction::new(ROL, 1, 2, Implied), // Accumulator
    [0x26] => Instruction::new(ROL, 2, 5, ZeroPage),
    [0x36] => Instruction::new(ROL, 2, 6, ZeroPageX),
    [0x2E] => Instruction::new(ROL, 3, 6, Absolute),
    [0x3E] => Instruction::new(ROL, 3, 7, AbsoluteX),

    [0x6A] => Instruction::new(ROR, 1, 2, Implied), // Accumulator
    [0x66] => Instruction::new(ROR, 2, 5, ZeroPage),
    [0x76] => Instruction::new(ROR, 2, 6, ZeroPageX),
    [0x6E] => Instruction::new(ROR, 3, 6, Absolute),
    [0x7E] => Instruction::new(ROR, 3, 7, AbsoluteX),

    [0x40] => Instruction::new(RTI, 1, 6, Implied),

    [0x60] => Instruction::new(RTS, 1, 6, Implied),

    [0xE9] => Instruction::new(SBC, 2, 2, Immediate),
    [0xE5] => Instruction::new(SBC, 2, 3, ZeroPage),
    [0xF5] => Instruction::new(SBC, 2, 4, ZeroPageX),
    [0xED] => Instruction::new(SBC, 3, 4, Absolute),
    [0xFD] => Instruction::new(SBC, 3, 4, AbsoluteX),
    [0xF9] => Instruction::new(SBC, 3, 4, AbsoluteY),
    [0xE1] => Instruction::new(SBC, 2, 6, IndirectX),
    [0xF1] => Instruction::new(SBC, 2, 5, IndirectY),

    [0x38] => Instruction::new(SEC, 1, 2, Implied),

    [0xF8] => Instruction::new(SED, 1, 2, Implied),

    [0x78] => Instruction::new(SEI, 1, 2, Implied),

    [0x85] => Instruction::new(STA, 2, 3, ZeroPage),
    [0x95] => Instruction::new(STA, 2, 4, ZeroPageX),
    [0x8D] => Instruction::new(STA, 3, 4, Absolute),
    [0x9D] => Instruction::new(STA, 3, 5, AbsoluteX),
    [0x99] => Instruction::new(STA, 3, 5, AbsoluteY),
    [0x81] => Instruction::new(STA, 2, 6, IndirectX),
    [0x91] => Instruction::new(STA, 2, 6, IndirectY),

    [0x86] => Instruction::new(STX, 2, 3, ZeroPage),
    [0x96] => Instruction::new(STX, 2, 4, ZeroPageY),
    [0x8E] => Instruction::new(STX, 3, 4, Absolute),

    [0x84] => Instruction::new(STY, 2, 3, ZeroPage),
    [0x94] => Instruction::new(STY, 2, 4, ZeroPageX),
    [0x8C] => Instruction::new(STY, 3, 4, Absolute),

    [0xAA] => Instruction::new(TAX, 1, 2, Implied),

    [0xA8] => Instruction::new(TAY, 1, 2, Implied),

    [0xBA] => Instruction::new(TSX, 1, 2, Implied),

    [0x8A] => Instruction::new(TXA, 1, 2, Implied),

    [0x9A] => Instruction::new(TXS, 1, 2, Implied),

    [0x98] => Instruction::new(TYA, 1, 2, Implied),
};
