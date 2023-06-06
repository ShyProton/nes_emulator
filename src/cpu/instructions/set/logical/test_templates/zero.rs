use super::*;

fn base_logical(opcode: u8, logical_op: &Operation, diff: u8) {
    let mut cpu = Cpu::new();

    let mut value = 0b1010_1010;
    let argument = 0b1100_1100;

    cpu.load_program(&[opcode, 0x69 - diff]);
    cpu.memory.write(0x0069, argument);

    cpu.registers.index_x = diff;
    cpu.registers.accumulator = value;

    cpu.run();

    perform_logical_operation(&mut value, argument, logical_op);
    assert_eq!(cpu.registers.accumulator, value);
}

pub fn logical(opcode: u8, logical_op: &Operation) {
    base_logical(opcode, logical_op, 0x00);
}

pub fn x_logical(opcode: u8, logical_op: &Operation) {
    base_logical(opcode, logical_op, 0x0A);
}
