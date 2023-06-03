use super::{Cpu, RegisterAlias};

pub fn store_mem(opcode: u8, source: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x69]);

    cpu.registers.set_register(source, 0x55);

    cpu.run();

    assert_eq!(cpu.memory.read(0x0069), 0x55);
}

pub fn x_store_mem(opcode: u8, source: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x60]);

    cpu.registers
        .set_register(&RegisterAlias::X, 0x09)
        .set_register(source, 0x55);

    cpu.run();

    assert_eq!(cpu.memory.read(0x0069), 0x55);
}

pub fn y_store_mem(opcode: u8, source: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x60]);

    cpu.registers
        .set_register(&RegisterAlias::Y, 0x09)
        .set_register(source, 0x55);

    cpu.run();

    assert_eq!(cpu.memory.read(0x0069), 0x55);
}
