use super::*;

pub fn logical(opcode: u8, logical_op: &LogicalOperation) {
    let mut cpu = Cpu::new();

    let mut value = 0b1010_1010;
    let argument = 0b1100_1100;

    cpu.load_program(&[opcode, argument]);
    cpu.registers.accumulator = value;
    cpu.run();

    perform_logical_operation(&mut value, argument, logical_op);

    assert_eq!(cpu.registers.accumulator, value);
}
