use super::{
    test_prep,
    AddressingMode::{
        self, Absolute, AbsoluteX, AbsoluteY, Immediate, IndirectX, IndirectY, ZeroPage, ZeroPageX,
        ZeroPageY,
    },
    Cpu, RegisterAlias, StatusFlagAlias,
};
use std::{collections::HashMap, iter::zip};

type OpCodeModes = (Vec<u8>, Vec<AddressingMode>);

fn base_load(opcode: u8, addr_mode: &AddressingMode, target: &RegisterAlias) {
    let mut cpu = Cpu::new();

    test_prep::prepare(&mut cpu, opcode, addr_mode, 0x55);
    cpu.run();

    assert_eq!(cpu.registers.get_register(target), 0x55);
    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::Z));
    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::N));
}

fn base_flag_check(opcode: u8, addr_mode: &AddressingMode) {
    let mut cpu = Cpu::new();

    test_prep::prepare(&mut cpu, opcode, addr_mode, 0x00);
    cpu.run();

    assert!(cpu.registers.status.get_flag(StatusFlagAlias::Z));
    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::N));

    test_prep::prepare(&mut cpu, opcode, addr_mode, 0b1000_0000);
    cpu.run();

    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::Z));
    assert!(cpu.registers.status.get_flag(StatusFlagAlias::N));
}

fn get_instruction_map() -> HashMap<RegisterAlias, OpCodeModes> {
    HashMap::from([
        (
            RegisterAlias::A, // LDA
            (
                vec![0xA9, 0xA5, 0xB5, 0xAD, 0xBD, 0xB9, 0xA1, 0xB1],
                vec![
                    Immediate, ZeroPage, ZeroPageX, Absolute, AbsoluteX, AbsoluteY, IndirectX,
                    IndirectY,
                ],
            ),
        ),
        (
            RegisterAlias::X, // LDX
            (
                vec![0xA2, 0xA6, 0xB6, 0xAE, 0xBE],
                vec![Immediate, ZeroPage, ZeroPageY, Absolute, AbsoluteY],
            ),
        ),
        (
            RegisterAlias::Y, // LDY
            (
                vec![0xA0, 0xA4, 0xB4, 0xAC, 0xBC],
                vec![Immediate, ZeroPage, ZeroPageX, Absolute, AbsoluteX],
            ),
        ),
    ])
}

#[test]
fn load() {
    for (register, (opcodes, addr_modes)) in get_instruction_map() {
        for (op, mode) in zip(opcodes, addr_modes) {
            base_load(op, &mode, &register);
        }
    }
}

#[test]
fn flag_check() {
    for (_, (opcodes, addr_modes)) in get_instruction_map() {
        for (op, mode) in zip(opcodes, addr_modes) {
            base_flag_check(op, &mode);
        }
    }
}
