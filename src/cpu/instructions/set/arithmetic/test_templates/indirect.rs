use super::*;

pub fn x_arithmetic(opcode: u8, arithmetic_mode: &ArithmeticMode) {
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

    cpu.load_program(&[opcode, 0x30]);
    cpu.registers.index_x = 0x04;
    cpu.memory.write(0x0034, 0x0069);
    cpu.memory.write(0x0069, operand_2);

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

pub fn y_arithmetic(opcode: u8, arithmetic_mode: &ArithmeticMode) {
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

    cpu.load_program(&[opcode, 0x42]);
    cpu.registers.index_y = 0x09;
    cpu.memory.write(0x0042, 0x60);
    cpu.memory.write(0x0069, operand_2);

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
