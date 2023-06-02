use super::*;

pub fn load_data(opcode: u8, target: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x55]);
    cpu.run();

    assert_eq!(cpu.registers.get_register(target), 0x55);
    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::Z));
    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::N));
}

pub fn flag_check(opcode: u8) {
    let mut cpu = Cpu::new();

    cpu.load_program(&[opcode, 0x00]);
    cpu.run();

    assert!(cpu.registers.status.get_flag(StatusFlagAlias::Z));
    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::N));

    cpu.load_program(&[opcode, 0b1000_0000]);
    cpu.run();

    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::Z));
    assert!(cpu.registers.status.get_flag(StatusFlagAlias::N));
}
