use super::{Cpu, RegisterAlias};

pub fn store_mem(opcode: u8, target_alias: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x69]);

    *cpu.registers.by_alias(target_alias) = 0x55;

    cpu.run();

    assert_eq!(cpu.memory.read(0x0069), 0x55);
}

pub fn x_store_mem(opcode: u8, target_alias: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x60]);

    cpu.registers.index_x = 0x09;
    *cpu.registers.by_alias(target_alias) = 0x55;

    cpu.run();

    assert_eq!(cpu.memory.read(0x0069), 0x55);
}

pub fn y_store_mem(opcode: u8, target_alias: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x60]);

    cpu.registers.index_y = 0x09;
    *cpu.registers.by_alias(target_alias) = 0x55;

    cpu.run();

    assert_eq!(cpu.memory.read(0x0069), 0x55);
}
