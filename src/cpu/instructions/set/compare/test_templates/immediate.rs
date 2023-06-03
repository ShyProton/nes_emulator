use super::*;

pub fn compare(opcode: u8, target: &RegisterAlias) {
    let mut cpu = Cpu::new();

    // Equality case.
    cpu.load_program(&[opcode, 0x50]);
    cpu.registers.set_register(target, 0x50);
    cpu.run();

    assert!(cpu.registers.status.get_flag(StatusFlagAlias::C));
    assert!(cpu.registers.status.get_flag(StatusFlagAlias::Z));

    // Greater case.
    cpu.load_program(&[opcode, 0x4F]);
    cpu.registers.set_register(target, 0x50);
    cpu.run();

    assert!(cpu.registers.status.get_flag(StatusFlagAlias::C));
    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::Z));

    // Less case.
    cpu.load_program(&[opcode, 0x51]);
    cpu.registers.set_register(target, 0x50);
    cpu.run();

    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::C));
    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::Z));
}
