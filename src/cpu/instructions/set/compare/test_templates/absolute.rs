use super::*;

fn base_compare(opcode: u8, target: &RegisterAlias, mode_register: &RegisterAlias, difference: u8) {
    let load_data_and_run = |cpu: &mut Cpu, register_val, memory_val| {
        cpu.load_program(&[opcode, 0x42 - difference, 0x69]);

        cpu.registers.set_register(mode_register, difference);
        cpu.registers.set_register(target, register_val);
        cpu.memory.write(0x6942, memory_val);

        cpu.run();
    };

    let mut cpu = Cpu::new();

    // Equality case.
    load_data_and_run(&mut cpu, 0x50, 0x50);

    assert!(cpu.registers.status.get_flag(StatusFlagAlias::C));
    assert!(cpu.registers.status.get_flag(StatusFlagAlias::Z));

    // Greater case.
    load_data_and_run(&mut cpu, 0x50, 0x4F);

    assert!(cpu.registers.status.get_flag(StatusFlagAlias::C));
    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::Z));

    // Less case.
    load_data_and_run(&mut cpu, 0x50, 0x51);

    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::C));
    assert!(!cpu.registers.status.get_flag(StatusFlagAlias::Z));
}

pub fn compare(opcode: u8, target: &RegisterAlias) {
    // NOTE: Difference of 0 causes the addressing mode's register to be ignored.
    base_compare(opcode, target, &RegisterAlias::X, 0x00);
}

pub fn x_compare(opcode: u8, target: &RegisterAlias) {
    base_compare(opcode, target, &RegisterAlias::X, 0x02);
}

pub fn y_compare(opcode: u8, target: &RegisterAlias) {
    base_compare(opcode, target, &RegisterAlias::Y, 0x02);
}
