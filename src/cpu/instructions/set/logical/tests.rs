use super::{
    perform_logical_operation, test_prep,
    AddressingMode::{
        self, Absolute, AbsoluteX, AbsoluteY, Immediate, IndirectX, IndirectY, ZeroPage, ZeroPageX,
    },
    Cpu, LogicalOperation,
};
use std::collections::HashMap;

fn base_logical(opcode: u8, addr_mode: &AddressingMode, logical_op: &LogicalOperation) {
    let mut cpu = Cpu::new();

    let mut value = 0b1010_1010;
    let argument = 0b1100_1100;

    test_prep::prepare(&mut cpu, opcode, addr_mode, argument);
    cpu.registers.accumulator = value;

    cpu.run();

    perform_logical_operation(&mut value, argument, logical_op);
    assert_eq!(cpu.registers.accumulator, value);
}

#[test]
fn logical() {
    let addr_modes = vec![
        Immediate, ZeroPage, ZeroPageX, Absolute, AbsoluteX, AbsoluteY, IndirectX, IndirectY,
    ];

    let instruction_map: HashMap<LogicalOperation, Vec<u8>> = HashMap::from([
        (
            LogicalOperation::And,
            vec![0x29, 0x25, 0x35, 0x2D, 0x3D, 0x39, 0x21, 0x31],
        ),
        (
            LogicalOperation::Eor,
            vec![0x49, 0x45, 0x55, 0x4D, 0x5D, 0x59, 0x41, 0x51],
        ),
        (
            LogicalOperation::Ora,
            vec![0x09, 0x05, 0x15, 0x0D, 0x1D, 0x19, 0x01, 0x11],
        ),
    ]);

    for (logical_op, opcodes) in instruction_map {
        for (i, code) in opcodes.iter().enumerate() {
            base_logical(*code, &addr_modes[i], &logical_op);
        }
    }
}
