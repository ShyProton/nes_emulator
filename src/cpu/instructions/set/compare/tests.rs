use super::{
    test_prep,
    AddressingMode::{
        self, Absolute, AbsoluteX, AbsoluteY, Immediate, IndirectX, IndirectY, ZeroPage, ZeroPageX,
    },
    Cpu, RegisterAlias, StatusFlagAlias,
};
use std::{collections::HashMap, iter::zip};

fn base_compare(opcode: u8, addr_mode: &AddressingMode, target: &RegisterAlias) {
    let load_data_and_run = |cpu: &mut Cpu, register_val, memory_val| {
        test_prep::prepare(cpu, opcode, addr_mode, memory_val);
        cpu.registers.set_register(target, register_val);

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

#[test]
fn compare() {
    type OpCodeModes = (Vec<u8>, Vec<AddressingMode>);

    let instruction_map: HashMap<RegisterAlias, OpCodeModes> = HashMap::from([
        (
            RegisterAlias::A, // CMP
            (
                vec![0xC9, 0xC5, 0xD5, 0xCD, 0xDD, 0xD9, 0xC1, 0xD1],
                vec![
                    Immediate, ZeroPage, ZeroPageX, Absolute, AbsoluteX, AbsoluteY, IndirectX,
                    IndirectY,
                ],
            ),
        ),
        (
            RegisterAlias::X, // CPX
            (vec![0xE0, 0xE4, 0xEC], vec![Immediate, ZeroPage, Absolute]),
        ),
        (
            RegisterAlias::Y, // CPY
            (vec![0xC0, 0xC4, 0xCC], vec![Immediate, ZeroPage, Absolute]),
        ),
    ]);

    for (register, (opcodes, addr_modes)) in instruction_map {
        for (code, mode) in zip(opcodes, addr_modes) {
            base_compare(code, &mode, &register);
        }
    }
}
