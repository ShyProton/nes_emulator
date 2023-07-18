use std::collections::HashMap;

use super::{test_prep, AddressingMode, Cpu, RegisterAlias, StatusFlagAlias};

fn base_transfer(opcode: u8, source: &RegisterAlias, target: &RegisterAlias) {
    let mut cpu = Cpu::new();

    test_prep::prepare(&mut cpu, opcode, &AddressingMode::Implied, 0x00);
    cpu.registers.set_register(source, 0x55);

    cpu.run();

    assert_eq!(cpu.registers.get_register(target), 0x55);
    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::Z));
    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::N));
}

fn base_flag_check(opcode: u8, source: &RegisterAlias) {
    let mut cpu = Cpu::new();

    test_prep::prepare(&mut cpu, opcode, &AddressingMode::Implied, 0x00);
    cpu.registers.set_register(source, 0x00);
    cpu.run();

    assert!(cpu.registers.status.get_flag(StatusFlagAlias::Z));
    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::N));

    test_prep::prepare(&mut cpu, opcode, &AddressingMode::Implied, 0x00);
    cpu.registers.set_register(source, 0b1000_0000);
    cpu.run();

    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::Z));
    assert!(cpu.registers.status.get_flag(StatusFlagAlias::N));
}

fn get_instruction_map() -> HashMap<(RegisterAlias, RegisterAlias), u8> {
    HashMap::from([
        ((RegisterAlias::A, RegisterAlias::X), 0xAA),
        ((RegisterAlias::A, RegisterAlias::Y), 0xA8),
        ((RegisterAlias::S, RegisterAlias::X), 0xBA),
        ((RegisterAlias::X, RegisterAlias::A), 0x8A),
        ((RegisterAlias::X, RegisterAlias::S), 0x9A),
        ((RegisterAlias::Y, RegisterAlias::A), 0x98),
    ])
}

#[test]
fn transfer() {
    for ((source, target), opcode) in get_instruction_map() {
        base_transfer(opcode, &source, &target);
    }
}

#[test]
fn flag_check() {
    for ((source, _), opcode) in get_instruction_map() {
        base_flag_check(opcode, &source);
    }
}
