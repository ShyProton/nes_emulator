use super::{Cpu, RegisterAlias};

pub fn x_store_mem(opcode: u8, source: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x30]);

    cpu.registers.index_x = 0x04;
    cpu.registers.set_register(source, 0x55);
    cpu.memory.write(0x0034, 0x0069);

    cpu.run();

    assert_eq!(cpu.memory.read(0x0069), 0x55);
}

pub fn y_store_mem(opcode: u8, source: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x34]);

    cpu.registers.index_y = 0x60;
    cpu.registers.set_register(source, 0x55);
    cpu.memory.write(0x0034, 0x09);

    cpu.run();

    assert_eq!(cpu.memory.read(0x0069), 0x55);
}
