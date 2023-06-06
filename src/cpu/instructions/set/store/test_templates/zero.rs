use super::{Cpu, RegisterAlias};

fn base_store_mem(opcode: u8, source: &RegisterAlias, mode_register: &RegisterAlias, diff: u8) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x69 - diff]);

    cpu.registers
        .set_register(mode_register, diff)
        .set_register(source, 0x55);

    cpu.run();

    assert_eq!(cpu.memory.read(0x0069), 0x55);
}

pub fn store_mem(opcode: u8, source: &RegisterAlias) {
    // NOTE: Difference of 0 causes the addressing mode's register to be ignored.
    base_store_mem(opcode, source, &RegisterAlias::X, 0x00);
}

pub fn x_store_mem(opcode: u8, source: &RegisterAlias) {
    base_store_mem(opcode, source, &RegisterAlias::X, 0x0A);
}

pub fn y_store_mem(opcode: u8, source: &RegisterAlias) {
    base_store_mem(opcode, source, &RegisterAlias::Y, 0x0A);
}
