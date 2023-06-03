use super::*;

pub fn x_from_memory(opcode: u8, target: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x0A]);

    cpu.memory.write(0x0010, 0x69);
    cpu.memory.write(0x0069, 0x55);
    cpu.registers.set_register(&RegisterAlias::X, 0x06);

    cpu.run();

    assert_eq!(cpu.registers.get_register(target), 0x55);
}

pub fn y_from_memory(opcode: u8, target: &RegisterAlias) {
    let mut cpu = Cpu::new();
    cpu.load_program(&[opcode, 0x10]);

    cpu.memory.write(0x0010, 0x60);
    cpu.memory.write(0x0069, 0x55);
    cpu.registers.set_register(&RegisterAlias::Y, 0x09);

    cpu.run();

    assert_eq!(cpu.registers.get_register(target), 0x55);
}
