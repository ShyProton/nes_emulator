use super::{Cpu, RegisterAlias};

pub fn load_data(opcode: u8, target_alias: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x55]);
    cpu.run();

    assert_eq!(*cpu.registers.by_alias(target_alias), 0x55);
    assert!(!cpu.registers.status.get_flag('Z'));
    assert!(!cpu.registers.status.get_flag('N'));
}

pub fn z_flag_set(opcode: u8) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x00]);
    cpu.run();

    assert!(cpu.registers.status.get_flag('Z'));
}
