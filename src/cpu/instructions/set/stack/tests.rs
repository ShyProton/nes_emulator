use std::collections::HashMap;

use super::{test_prep, AddressingMode, Cpu, RegisterAlias, StatusFlagAlias};

fn base_push(opcode: u8, target: &RegisterAlias) {
    let mut cpu = Cpu::new();
    let expected = 0b1100_1011;

    test_prep::prepare(&mut cpu, opcode, &AddressingMode::Implied, 0x00);
    cpu.registers.set_register(target, expected);

    cpu.run();

    cpu.registers.stack_pointer += 1;
    assert_eq!(cpu.memory.read(cpu.get_stack_addr()), expected);
}

fn base_pull(opcode: u8, target: &RegisterAlias) {
    let mut cpu = Cpu::new();
    let expected = 0b1100_1011;

    test_prep::prepare(&mut cpu, opcode, &AddressingMode::Implied, 0x00);

    cpu.memory.write(cpu.get_stack_addr(), expected);
    cpu.registers.stack_pointer -= 1;

    cpu.run();

    cpu.registers.status.set_flag(StatusFlagAlias::B, false);
    assert_eq!(cpu.registers.get_register(target), expected);
}

#[test]
fn push() {
    let instruction_map = HashMap::from([(RegisterAlias::A, 0x48), (RegisterAlias::P, 0x08)]);

    for (register, opcode) in instruction_map {
        base_push(opcode, &register);
    }
}

#[test]
fn pull() {
    let instruction_map = HashMap::from([(RegisterAlias::A, 0x68), (RegisterAlias::P, 0x28)]);

    for (register, opcode) in instruction_map {
        base_pull(opcode, &register);
    }
}
