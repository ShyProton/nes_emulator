use std::{collections::HashMap, iter::zip};

use super::{
    test_prep,
    AddressingMode::{
        self, Absolute, AbsoluteX, AbsoluteY, IndirectX, IndirectY, ZeroPage, ZeroPageX, ZeroPageY,
    },
    Cpu, RegisterAlias,
};

fn base_store(opcode: u8, addr_mode: &AddressingMode, source: &RegisterAlias) {
    let mut cpu = Cpu::new();

    test_prep::prepare(&mut cpu, opcode, addr_mode, 0x00);
    cpu.registers.set_register(source, 0x55);

    cpu.run();

    assert_eq!(cpu.memory.read(test_prep::get_addr(addr_mode)), 0x55);
}

#[test]
fn store() {
    let instruction_map: HashMap<RegisterAlias, (Vec<u8>, Vec<AddressingMode>)> = HashMap::from([
        (
            RegisterAlias::A,
            (
                vec![0x85, 0x95, 0x8D, 0x9D, 0x99, 0x81, 0x91],
                vec![
                    ZeroPage, ZeroPageX, Absolute, AbsoluteX, AbsoluteY, IndirectX, IndirectY,
                ],
            ),
        ),
        (
            RegisterAlias::X,
            (vec![0x86, 0x96, 0x8E], vec![ZeroPage, ZeroPageY, Absolute]),
        ),
        (
            RegisterAlias::Y,
            (vec![0x84, 0x94, 0x8C], vec![ZeroPage, ZeroPageX, Absolute]),
        ),
    ]);

    for (register, (opcodes, addr_modes)) in instruction_map {
        for (op, mode) in zip(opcodes, addr_modes) {
            base_store(op, &mode, &register);
        }
    }
}
