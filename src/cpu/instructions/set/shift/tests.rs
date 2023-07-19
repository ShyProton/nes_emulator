use super::{
    test_prep,
    AddressingMode::{self, Absolute, AbsoluteX, Implied, ZeroPage, ZeroPageX},
    Cpu, ShiftDirection, ShiftType, StatusFlagAlias,
};
use std::{collections::HashMap, iter::zip};

fn base_shift(
    opcode: u8,
    addr_mode: &AddressingMode,
    shift_type: &ShiftType,
    shift_direction: &ShiftDirection,
) {
    let mut cpu = Cpu::new();
    let initial_carry = 1;

    test_prep::prepare(&mut cpu, opcode, addr_mode, 0b1001_1010);
    cpu.registers.accumulator = 0b1001_1010; // For Implied (Accumulator) mode.
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

    let expected = match addr_mode {
        Implied => cpu.registers.accumulator,
        _ => cpu.memory.read(test_prep::get_addr(addr_mode)),
    };

    assert_eq!(expected, accumulator, "{opcode}, {addr_mode:?}");
    assert_eq!(cpu.registers.status.get_flag(StatusFlagAlias::C), carry);
}

#[test]
pub fn shift() {
    type ShiftSpecs = (ShiftType, ShiftDirection);
    type OpCodeModes = (Vec<u8>, Vec<AddressingMode>);

    let instruction_map: HashMap<ShiftSpecs, OpCodeModes> = HashMap::from([
        (
            (ShiftType::Shift, ShiftDirection::Left),
            (
                vec![0x0A, 0x06, 0x16, 0x0E, 0x1E],
                vec![Implied, ZeroPage, ZeroPageX, Absolute, AbsoluteX],
            ),
        ),
        (
            (ShiftType::Shift, ShiftDirection::Right),
            (
                vec![0x4A, 0x46, 0x56, 0x4E, 0x5E],
                vec![Implied, ZeroPage, ZeroPageX, Absolute, AbsoluteX],
            ),
        ),
        (
            (ShiftType::Rotate, ShiftDirection::Left),
            (
                vec![0x2A, 0x26, 0x36, 0x2E, 0x3E],
                vec![Implied, ZeroPage, ZeroPageX, Absolute, AbsoluteX],
            ),
        ),
        (
            (ShiftType::Rotate, ShiftDirection::Right),
            (
                vec![0x6A, 0x66, 0x76, 0x6E, 0x7E],
                vec![Implied, ZeroPage, ZeroPageX, Absolute, AbsoluteX],
            ),
        ),
    ]);

    for ((shift_type, shift_direction), (opcodes, addr_modes)) in instruction_map {
        for (op, mode) in zip(opcodes, addr_modes) {
            base_shift(op, &mode, &shift_type, &shift_direction);
        }
    }
}
