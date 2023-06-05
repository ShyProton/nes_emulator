use super::*;

fn base_crement(opcode: u8, crement_mode: &CrementMode, diff: u8) {
    let mut cpu = Cpu::new();

    let initial = 0x0A;
    let expected = match crement_mode {
        CrementMode::Increment => initial + 1,
        CrementMode::Decrement => initial - 1,
    };

    cpu.load_program(&[opcode, 0x69 - diff]);

    cpu.registers.set_register(&RegisterAlias::X, diff);
    cpu.memory.write(0x0069, initial);

    cpu.run();

    assert_eq!(cpu.memory.read(0x0069), expected);
}

pub fn wrapping(opcode: u8, crement_mode: &CrementMode) {
    let mut cpu = Cpu::new();

    let (initial, expected) = match crement_mode {
        CrementMode::Increment => (0xFF, 0x00),
        CrementMode::Decrement => (0x00, 0xFF),
    };

    cpu.load_program(&[opcode, 0x69]);
    cpu.memory.write(0x0069, initial);

    cpu.run();

    assert_eq!(cpu.memory.read(0x0069), expected);
}

pub fn crement_mem(opcode: u8, crement_mode: &CrementMode) {
    base_crement(opcode, crement_mode, 0x00);
}

pub fn x_crement_mem(opcode: u8, crement_mode: &CrementMode) {
    base_crement(opcode, crement_mode, 0x09);
}
