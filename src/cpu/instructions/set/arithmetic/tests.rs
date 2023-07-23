use super::{
    test_prep,
    AddressingMode::{
        self, Absolute, AbsoluteX, AbsoluteY, Immediate, IndirectX, IndirectY, ZeroPage, ZeroPageX,
    },
    ArithmeticMode, Cpu, StatusFlagAlias,
};
use std::{collections::HashMap, iter::zip};

fn base_arithmetic(opcode: u8, addr_mode: &AddressingMode, arithmetic_mode: &ArithmeticMode) {
    let mut cpu = Cpu::new();

    let operand_1: u8 = 0x85;
    let operand_2: u8 = 0x10;

    let carry = true;

    let (expected_result, expected_flags) = match arithmetic_mode {
        ArithmeticMode::Addition => (
            operand_1
                .wrapping_add(operand_2)
                .wrapping_add(u8::from(carry)),
            false,
        ),
        ArithmeticMode::Subtraction => (
            operand_1
                .wrapping_sub(operand_2)
                .wrapping_sub(u8::from(!carry)),
            true,
        ),
    };

    test_prep::prepare(&mut cpu, opcode, addr_mode, operand_2);
    cpu.registers.accumulator = operand_1;
    cpu.registers.status.set_flag(StatusFlagAlias::C, carry);

    cpu.run();

    assert_eq!(cpu.registers.accumulator, expected_result);
    assert_eq!(
        cpu.registers.status.get_flag(StatusFlagAlias::C),
        expected_flags
    );
    assert_eq!(
        cpu.registers.status.get_flag(StatusFlagAlias::V),
        expected_flags
    );
}

#[test]
fn arithmetic() {
    const MODE_COUNT: usize = 8;
    const ADDR_MODES: [AddressingMode; MODE_COUNT] = [
        Immediate, ZeroPage, ZeroPageX, Absolute, AbsoluteX, AbsoluteY, IndirectX, IndirectY,
    ];

    type OpCodes = [u8; MODE_COUNT];

    let instruction_map: HashMap<ArithmeticMode, OpCodes> = HashMap::from([
        (
            ArithmeticMode::Addition, // ADC
            [0x69, 0x65, 0x75, 0x6D, 0x7D, 0x79, 0x61, 0x71],
        ),
        (
            ArithmeticMode::Subtraction, // SBC
            [0xE9, 0xE5, 0xF5, 0xED, 0xFD, 0xF9, 0xE1, 0xF1],
        ),
    ]);

    for (arithmetic_mode, opcodes) in instruction_map {
        for (code, mode) in zip(opcodes, ADDR_MODES) {
            base_arithmetic(code, &mode, &arithmetic_mode);
        }
    }
}
