use std::collections::HashMap;

use super::{test_prep, AddressingMode, Cpu, RegisterAlias, StatusFlagAlias};

fn base_push(opcode: u8, target: &RegisterAlias) {
    let mut cpu = Cpu::new();
    let expected = 0b1100_1011;

    test_prep::prepare(&mut cpu, opcode, &AddressingMode::Implied, 0x00);
    cpu.registers.set_register(target, expected);

    cpu.run();

    assert_eq!(cpu.pull_stack(), expected);
}

fn base_pull(opcode: u8, target: &RegisterAlias) {
    let mut cpu = Cpu::new();
    let expected = 0b1100_1011;

    test_prep::prepare(&mut cpu, opcode, &AddressingMode::Implied, 0x00);

    cpu.push_stack(expected);
    cpu.run();

    cpu.registers.status.set_flag(StatusFlagAlias::B, false);
    assert_eq!(cpu.registers.get_register(target), expected);
}

#[test]
fn push() {
    let instruction_map: HashMap<RegisterAlias, u8> = HashMap::from([
        (RegisterAlias::A, 0x48), // PHA
        (RegisterAlias::P, 0x08), // PHP
    ]);

    for (register, opcode) in instruction_map {
        base_push(opcode, &register);
    }
}

#[test]
fn pull() {
    let instruction_map: HashMap<RegisterAlias, u8> = HashMap::from([
        (RegisterAlias::A, 0x68), // PLA
        (RegisterAlias::P, 0x28), // PLP
    ]);

    for (register, opcode) in instruction_map {
        base_pull(opcode, &register);
    }
}
