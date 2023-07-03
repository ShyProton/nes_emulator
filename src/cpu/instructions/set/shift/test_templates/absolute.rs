use super::*;

fn base_shift(opcode: u8, shift_type: &ShiftType, shift_direction: &ShiftDirection, diff: u8) {
    let mut cpu = Cpu::new();
    let initial_carry = 1;

    cpu.load_program(&[opcode, 0x42 - diff, 0x69]);
    cpu.registers
        .status
        .set_flag(StatusFlagAlias::C, initial_carry != 0);

    cpu.registers.index_x = diff;
    cpu.memory.write(0x6942, 0b1001_1010);

    cpu.run();

    let carry_set = match shift_type {
        ShiftType::Shift => 0,
        ShiftType::Rotate => initial_carry,
    };

    let (memory, carry) = match shift_direction {
        ShiftDirection::Left => (0b0011_0100 | carry_set, true),
        ShiftDirection::Right => (0b0100_1101 | carry_set << (8 - 1), false),
    };

    assert_eq!(cpu.memory.read(0x6942), memory);
    assert_eq!(cpu.registers.status.get_flag(StatusFlagAlias::C), carry);
}

pub fn shift_mem(opcode: u8, shift_type: &ShiftType, shift_direction: &ShiftDirection) {
    base_shift(opcode, shift_type, shift_direction, 0x00);
}

pub fn x_shift_mem(opcode: u8, shift_type: &ShiftType, shift_direction: &ShiftDirection) {
    base_shift(opcode, shift_type, shift_direction, 0x0A);
}
