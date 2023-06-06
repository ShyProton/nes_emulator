use super::*;

fn base_logical(
    opcode: u8,
    logical_op: &LogicalOperation,
    mode_register: &RegisterAlias,
    diff: u8,
) {
    let mut cpu = Cpu::new();

    let mut value = 0b1010_1010;
    let argument = 0b1100_1100;

    cpu.load_program(&[opcode, 0x34 - diff, 0x12]);
    cpu.memory.write(0x1234, argument);

    cpu.registers.set_register(mode_register, diff);
    cpu.registers.accumulator = value;

    cpu.run();

    perform_logical_operation(&mut value, argument, logical_op);
    assert_eq!(cpu.registers.accumulator, value);
}

pub fn logical(opcode: u8, logical_op: &LogicalOperation) {
    // NOTE: Difference of 0 causes the addressing mode's register to be ignored.
    base_logical(opcode, logical_op, &RegisterAlias::X, 0x00);
}

pub fn x_logical(opcode: u8, logical_op: &LogicalOperation) {
    base_logical(opcode, logical_op, &RegisterAlias::X, 0x0A);
}

pub fn y_logical(opcode: u8, logical_op: &LogicalOperation) {
    base_logical(opcode, logical_op, &RegisterAlias::Y, 0x0A);
}
