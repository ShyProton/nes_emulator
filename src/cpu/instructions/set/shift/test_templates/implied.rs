use super::*;

pub fn shift(opcode: u8, shift_type: &ShiftType, shift_direction: &ShiftDirection) {
    let mut cpu = Cpu::new();
    let initial_carry = 1;

    cpu.load_program(&[opcode]);
    cpu.registers.accumulator = 0b1001_1010;
    cpu.registers
        .status
        .set_flag(StatusFlagAlias::C, initial_carry != 0);

    cpu.run();

    let carry_set = match shift_type {
        ShiftType::Shift => 0,
        ShiftType::Rotate => initial_carry,
    };

    let (accumulator, carry) = match shift_direction {
        ShiftDirection::Left => (0b0011_0100 | carry_set, true),
        ShiftDirection::Right => (0b0100_1101 | carry_set << 7, false),
    };

    assert_eq!(cpu.registers.accumulator, accumulator);
    assert_eq!(cpu.registers.status.get_flag(StatusFlagAlias::C), carry);
}
