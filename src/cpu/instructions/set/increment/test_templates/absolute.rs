use super::*;

pub fn crement_mem(opcode: u8, crement_mode: &CrementMode) {
    let mut cpu = Cpu::new();

    let initial = 0x0A;
    let expected = match crement_mode {
        CrementMode::Increment => initial + 1,
        CrementMode::Decrement => initial - 1,
    };

    cpu.load_program(&[opcode, 0x42, 0x69]);
    cpu.memory.write(0x6942, initial);

    cpu.run();

    assert_eq!(cpu.memory.read(0x6942), expected);
}

pub fn x_crement_mem(opcode: u8, crement_mode: &CrementMode) {
    let mut cpu = Cpu::new();

    let initial = 0x0A;
    let expected = match crement_mode {
        CrementMode::Increment => initial + 1,
        CrementMode::Decrement => initial - 1,
    };

    cpu.load_program(&[opcode, 0x40, 0x69]);
    cpu.registers.index_x = 2;
    cpu.memory.write(0x6942, initial);

    cpu.run();

    assert_eq!(cpu.memory.read(0x6942), expected);
}
