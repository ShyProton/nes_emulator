use super::*;

fn base_from_memory(opcode: u8, target: &RegisterAlias, mode_register: &RegisterAlias, diff: u8) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x34 - diff, 0x12]);

    cpu.memory.write(0x1234, 0x55);
    cpu.registers.set_register(mode_register, diff);

    cpu.run();

    assert_eq!(cpu.registers.get_register(target), 0x55);
}

pub fn from_memory(opcode: u8, target: &RegisterAlias) {
    // NOTE: Difference of 0 causes the addressing mode's register to be ignored.
    base_from_memory(opcode, target, &RegisterAlias::X, 0x00);
}

pub fn x_from_memory(opcode: u8, target: &RegisterAlias) {
    base_from_memory(opcode, target, &RegisterAlias::X, 0x0A);
}

pub fn y_from_memory(opcode: u8, target: &RegisterAlias) {
    base_from_memory(opcode, target, &RegisterAlias::Y, 0x0A);
}
