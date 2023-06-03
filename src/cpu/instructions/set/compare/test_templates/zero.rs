use super::*;

pub fn compare(opcode: u8, target: &RegisterAlias) {
    let mut cpu = Cpu::new();

    // Equality case.
    cpu.load_program(&[opcode, 0x69]);
    cpu.registers.set_register(target, 0x50);
    cpu.memory.write(0x0069, 0x50);
    cpu.run();

    assert!(cpu.registers.status.get_flag(StatusFlagAlias::C));
    assert!(cpu.registers.status.get_flag(StatusFlagAlias::Z));

    // Greater case.
    cpu.load_program(&[opcode, 0x69]);
    cpu.registers.set_register(target, 0x50);
    cpu.memory.write(0x0069, 0x4F);
    cpu.run();

    assert!(cpu.registers.status.get_flag(StatusFlagAlias::C));
    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::Z));

    // Less case.
    cpu.load_program(&[opcode, 0x69]);
    cpu.registers.set_register(target, 0x50);
    cpu.memory.write(0x0069, 0x51);
    cpu.run();

    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::C));
    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::Z));
}

pub fn x_compare(opcode: u8, target: &RegisterAlias) {
    let mut cpu = Cpu::new();

    // Equality case.
    cpu.load_program(&[opcode, 0x60]);
    cpu.registers.set_register(&RegisterAlias::X, 0x09);
    cpu.registers.set_register(target, 0x50);
    cpu.memory.write(0x0069, 0x50);
    cpu.run();

    assert!(cpu.registers.status.get_flag(StatusFlagAlias::C));
    assert!(cpu.registers.status.get_flag(StatusFlagAlias::Z));

    // Greater case.
    cpu.load_program(&[opcode, 0x60]);
    cpu.registers.set_register(&RegisterAlias::X, 0x09);
    cpu.registers.set_register(target, 0x50);
    cpu.memory.write(0x0069, 0x4F);
    cpu.run();

    assert!(cpu.registers.status.get_flag(StatusFlagAlias::C));
    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::Z));

    // Less case.
    cpu.load_program(&[opcode, 0x60]);
    cpu.registers.set_register(&RegisterAlias::X, 0x09);
    cpu.registers.set_register(target, 0x50);
    cpu.memory.write(0x0069, 0x51);
    cpu.run();

    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::C));
    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::Z));
}
