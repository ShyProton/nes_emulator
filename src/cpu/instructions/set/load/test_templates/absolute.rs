use super::*;

pub fn from_memory(opcode: u8, target: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x34, 0x12]);

    cpu.memory.write(0x1234, 0x55);

    cpu.run();

    assert_eq!(cpu.registers.get_register(target), 0x55);
}

pub fn x_from_memory(opcode: u8, target: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x00, 0x12]);

    cpu.memory.write(0x1234, 0x55);
    cpu.registers.index_x = 0x34;

    cpu.run();

    assert_eq!(cpu.registers.get_register(target), 0x55);
}

pub fn y_from_memory(opcode: u8, target: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x00, 0x12]);

    cpu.memory.write(0x1234, 0x55);
    cpu.registers.index_y = 0x34;

    cpu.run();

    assert_eq!(cpu.registers.get_register(target), 0x55);
}
