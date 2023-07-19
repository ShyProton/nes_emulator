use super::{
    perform_logical_operation, test_prep,
    AddressingMode::{
        self, Absolute, AbsoluteX, AbsoluteY, Immediate, IndirectX, IndirectY, ZeroPage, ZeroPageX,
    },
    Cpu, LogicalOperation,
};
use std::{collections::HashMap, iter::zip};

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
    const MODE_COUNT: usize = 8;
    const ADDR_MODES: [AddressingMode; MODE_COUNT] = [
        Immediate, ZeroPage, ZeroPageX, Absolute, AbsoluteX, AbsoluteY, IndirectX, IndirectY,
    ];

    type OpCodes = [u8; MODE_COUNT];

    let instruction_map: HashMap<LogicalOperation, OpCodes> = HashMap::from([
        (
            LogicalOperation::And,
            [0x29, 0x25, 0x35, 0x2D, 0x3D, 0x39, 0x21, 0x31],
        ),
        (
            LogicalOperation::Eor,
            [0x49, 0x45, 0x55, 0x4D, 0x5D, 0x59, 0x41, 0x51],
        ),
        (
            LogicalOperation::Ora,
            [0x09, 0x05, 0x15, 0x0D, 0x1D, 0x19, 0x01, 0x11],
        ),
    ]);

    for (logical_op, opcodes) in instruction_map {
        for (code, mode) in zip(opcodes, ADDR_MODES) {
            base_logical(code, &mode, &logical_op);
        }
    }
}
