use super::*;

pub fn wrapping(opcode: u8, target: &RegisterAlias, crement_mode: &CrementMode) {
    let mut cpu = Cpu::new();

    let (initial, expected) = match crement_mode {
        CrementMode::Increment => (0xFF, 0x00),
        CrementMode::Decrement => (0x00, 0xFF),
    };

    cpu.load_program(&[opcode]);
    cpu.registers.set_register(target, initial);

    cpu.run();

    assert_eq!(cpu.registers.get_register(target), expected);
}

pub fn crement(opcode: u8, target: &RegisterAlias, crement_mode: &CrementMode) {
    let mut cpu = Cpu::new();

    let initial = 0x0A;
    let expected = match crement_mode {
        CrementMode::Increment => initial + 1,
        CrementMode::Decrement => initial - 1,
    };

    cpu.load_program(&[opcode]);
    cpu.registers.set_register(target, initial);

    cpu.run();

    assert_eq!(cpu.registers.get_register(target), expected);
}
