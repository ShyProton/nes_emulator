use super::*;

pub fn from_memory(opcode: u8, target: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x10]);

    cpu.memory.write(0x0010, 0x55);

    cpu.run();

    assert_eq!(cpu.registers.get_register(target), 0x55);
}

pub fn x_from_memory(opcode: u8, target: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x06]);

    cpu.memory.write(0x0010, 0x55);
    cpu.registers.index_x = 0x0A;

    cpu.run();

    assert_eq!(cpu.registers.get_register(target), 0x55);
}

pub fn y_from_memory(opcode: u8, target: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x06]);

    cpu.memory.write(0x0010, 0x55);
    cpu.registers.index_y = 0x0A;

    cpu.run();

    assert_eq!(cpu.registers.get_register(target), 0x55);
}
