use super::*;

pub fn x_compare(opcode: u8, target: &RegisterAlias) {
    let load_data_and_run = |cpu: &mut Cpu, register_val, memory_val| {
        cpu.load_program(&[opcode, 0x40]);

        cpu.registers.index_x = 0x02;
        cpu.registers.set_register(target, register_val);

        cpu.memory.write(0x0042, 0x69);
        cpu.memory.write(0x0069, memory_val);

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

pub fn y_compare(opcode: u8, target: &RegisterAlias) {
    let load_data_and_run = |cpu: &mut Cpu, register_val, memory_val| {
        cpu.load_program(&[opcode, 0x42]);

        cpu.registers.index_y = 0x09;
        cpu.registers.set_register(target, register_val);

        cpu.memory.write(0x0042, 0x60);
        cpu.memory.write(0x0069, memory_val);

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
