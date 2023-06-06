use super::*;

pub fn x_logical(opcode: u8, logical_op: &LogicalOperation) {
    let mut cpu = Cpu::new();

    let mut value = 0b1010_1010;
    let argument = 0b1100_1100;

    cpu.load_program(&[opcode, 0x0A]);

    cpu.memory.write(0x0010, 0x69);
    cpu.memory.write(0x0069, argument);

    cpu.registers.index_x = 0x06;
    cpu.registers.accumulator = value;

    cpu.run();

    perform_logical_operation(&mut value, argument, logical_op);
    assert_eq!(cpu.registers.accumulator, value);
}

pub fn y_logical(opcode: u8, logical_op: &LogicalOperation) {
    let mut cpu = Cpu::new();

    let mut value = 0b1010_1010;
    let argument = 0b1100_1100;

    cpu.load_program(&[opcode, 0x10]);

    cpu.memory.write(0x0010, 0x60);
    cpu.memory.write(0x0069, argument);

    cpu.registers.index_y = 0x09;
    cpu.registers.accumulator = value;

    cpu.run();

    perform_logical_operation(&mut value, argument, logical_op);
    assert_eq!(cpu.registers.accumulator, value);
}
