use super::*;

fn base_crement(opcode: u8, crement_mode: &CrementMode, diff: u8) {
    let mut cpu = Cpu::new();

    let initial = 0x0A;
    let expected = match crement_mode {
        CrementMode::Increment => initial + 1,
        CrementMode::Decrement => initial - 1,
    };

    cpu.load_program(&[opcode, 0x42 - diff, 0x69]);
    cpu.registers.index_x = diff;
    cpu.memory.write(0x6942, initial);

    cpu.run();

    assert_eq!(cpu.memory.read(0x6942), expected);
}

pub fn crement_mem(opcode: u8, crement_mode: &CrementMode) {
    base_crement(opcode, crement_mode, 0x00);
}

pub fn x_crement_mem(opcode: u8, crement_mode: &CrementMode) {
    base_crement(opcode, crement_mode, 0x0A);
}
