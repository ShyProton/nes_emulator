use super::{test_prep, AddressingMode, Cpu, JumpType};
use crate::Memory;
use std::{collections::HashMap, iter::zip};

fn base_jump(opcode: u8, addr_mode: &AddressingMode, jump_type: &JumpType) {
    let mut cpu = Cpu::new();
    let indirect_val = 0xEA; // NOP

    test_prep::prepare(&mut cpu, opcode, addr_mode, indirect_val);

    let (expected_program_counter, nop_subtraction) = match addr_mode {
        AddressingMode::Absolute => (test_prep::ABS_ADDR, 1),
        AddressingMode::Indirect => (cpu.memory.read_u16(test_prep::ABS_ADDR), 0),
        _ => panic!("Unsupported addressing mode for jumping instruction."),
    };

    let (expected_stack, stack_addition) = match jump_type {
        JumpType::Jump => (0x0000, 0),
        JumpType::Subroutine => (Memory::PRG_ROM_START.wrapping_add(3), 1), // Initial PC + 3 bytes
    };

    cpu.run();

    assert_eq!(
        // -1 for (potentially) reading NOP, -1 for reading BRK...
        cpu.registers
            .program_counter
            .wrapping_sub(nop_subtraction)
            .wrapping_sub(1),
        expected_program_counter,
    );

    // Pulling from the stack.
    cpu.registers.program_counter = cpu.pull_stack_u16();

    assert_eq!(
        cpu.registers.program_counter.wrapping_add(stack_addition),
        expected_stack
    );
}

#[test]
fn jump() {
    type OpCodeModes = (Vec<u8>, Vec<AddressingMode>);

    let instruction_map: HashMap<JumpType, OpCodeModes> = HashMap::from([
        (
            JumpType::Jump, // JMP
            (
                vec![0x4C, 0x6C],
                vec![AddressingMode::Absolute, AddressingMode::Indirect],
            ),
        ),
        (
            JumpType::Subroutine, // JSR
            (vec![0x20], vec![AddressingMode::Absolute]),
        ),
    ]);

    for (jump_type, (opcodes, addr_modes)) in instruction_map {
        for (code, mode) in zip(opcodes, addr_modes) {
            base_jump(code, &mode, &jump_type);
        }
    }
}
