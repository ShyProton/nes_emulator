use super::{
    test_prep,
    AddressingMode::{self, Absolute, AbsoluteX, Implied, ZeroPage, ZeroPageX},
    Cpu, CrementMode, RegisterAlias,
};
use std::{collections::HashMap, iter::zip};

type IncrementSpecs = (CrementMode, Option<RegisterAlias>);
type OpCodeModes = (Vec<u8>, Vec<AddressingMode>);

type CrementTest = dyn Fn(u8, &AddressingMode, &CrementMode, &Option<RegisterAlias>);

fn perform_test(
    cpu: &mut Cpu,
    initial: u8,
    expected: u8,
    addr_mode: &AddressingMode,
    target_register: &Option<RegisterAlias>,
) {
    if let Some(register) = target_register {
        cpu.registers.set_register(register, initial);
    }

    cpu.run();

    let result = match target_register {
        Some(register) => cpu.registers.get_register(register),
        None => cpu.memory.read(test_prep::get_addr(addr_mode)),
    };

    assert_eq!(result, expected);
}

fn base_crement(
    opcode: u8,
    addr_mode: &AddressingMode,
    crement_mode: &CrementMode,
    target_register: &Option<RegisterAlias>,
) {
    let mut cpu = Cpu::new();

    let initial = 0x0A;
    let expected = match crement_mode {
        CrementMode::Increment => initial + 1,
        CrementMode::Decrement => initial - 1,
    };

    test_prep::prepare(&mut cpu, opcode, addr_mode, initial);
    perform_test(&mut cpu, initial, expected, addr_mode, target_register);
}

fn base_wrapping(
    opcode: u8,
    addr_mode: &AddressingMode,
    crement_mode: &CrementMode,
    target_register: &Option<RegisterAlias>,
) {
    let mut cpu = Cpu::new();

    let (initial, expected) = match crement_mode {
        CrementMode::Increment => (0xFF, 0x00),
        CrementMode::Decrement => (0x00, 0xFF),
    };

    test_prep::prepare(&mut cpu, opcode, addr_mode, initial);
    perform_test(&mut cpu, initial, expected, addr_mode, target_register);
}

fn get_instruction_map() -> HashMap<IncrementSpecs, OpCodeModes> {
    HashMap::from([
        (
            (CrementMode::Decrement, None), // DEC
            (
                vec![0xC6, 0xD6, 0xCE, 0xDE],
                vec![ZeroPage, ZeroPageX, Absolute, AbsoluteX],
            ),
        ),
        (
            (CrementMode::Increment, None), // INC
            (
                vec![0xE6, 0xF6, 0xEE, 0xFE],
                vec![ZeroPage, ZeroPageX, Absolute, AbsoluteX],
            ),
        ),
        (
            (CrementMode::Decrement, Some(RegisterAlias::X)), // DEX
            (vec![0xCA], vec![Implied]),
        ),
        (
            (CrementMode::Decrement, Some(RegisterAlias::Y)), // DEY
            (vec![0x88], vec![Implied]),
        ),
        (
            (CrementMode::Increment, Some(RegisterAlias::X)), // INX
            (vec![0xE8], vec![Implied]),
        ),
        (
            (CrementMode::Increment, Some(RegisterAlias::Y)), // INY
            (vec![0xC8], vec![Implied]),
        ),
    ])
}

fn test_loop(test: &CrementTest) {
    for ((crement_mode, register), (opcodes, addr_modes)) in get_instruction_map() {
        for (code, mode) in zip(opcodes, addr_modes) {
            test(code, &mode, &crement_mode, &register);
        }
    }
}

#[test]
fn crement() {
    test_loop(&base_crement);
}

#[test]
fn wrapping() {
    test_loop(&base_wrapping);
}
