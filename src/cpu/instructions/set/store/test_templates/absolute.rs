use super::{Cpu, RegisterAlias};

pub fn store_mem(opcode: u8, source: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x42, 0x69]);

    *cpu.registers.by_alias(source) = 0x55;

    cpu.run();

    assert_eq!(cpu.memory.read(0x6942), 0x55);
}

pub fn x_store_mem(opcode: u8, source: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x40, 0x69]);

    cpu.registers.index_x = 0x02;
    *cpu.registers.by_alias(source) = 0x55;

    cpu.run();

    assert_eq!(cpu.memory.read(0x6942), 0x55);
}

pub fn y_store_mem(opcode: u8, source: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x40, 0x69]);

    cpu.registers.index_y = 0x02;
    *cpu.registers.by_alias(source) = 0x55;

    cpu.run();

    assert_eq!(cpu.memory.read(0x6942), 0x55);
}
