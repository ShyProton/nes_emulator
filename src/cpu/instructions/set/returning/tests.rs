use super::{test_prep, AddressingMode, Cpu, ReturnMode};
use std::collections::HashMap;

fn base_return(opcode: u8, return_mode: &ReturnMode) {
    let mut cpu = Cpu::new();

    test_prep::prepare(&mut cpu, opcode, &AddressingMode::Implied, 0x00);

    let expected_program_counter = 0x6942;

    cpu.memory
        .write_u16(cpu.get_stack_addr(), expected_program_counter);
    cpu.registers.stack_pointer = cpu.registers.stack_pointer.wrapping_sub(0x01);

    let addition = match return_mode {
        ReturnMode::Interrupt => {
            cpu.php();
            0x0000
        }
        ReturnMode::Subroutine => 0x0001,
    };

    cpu.run();

    assert_eq!(
        cpu.registers.program_counter.wrapping_sub(0x0001),
        expected_program_counter.wrapping_add(addition),
    );
}

#[test]
fn return_from() {
    let instruction_map: HashMap<ReturnMode, u8> = HashMap::from([
        (ReturnMode::Interrupt, 0x40),  // RTI
        (ReturnMode::Subroutine, 0x60), // RTS
    ]);

    for (mode, code) in instruction_map {
        base_return(code, &mode);
    }
}
