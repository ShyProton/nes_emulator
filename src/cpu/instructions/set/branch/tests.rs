use super::{test_prep, u8_to_u16, AddressingMode, Cpu, StatusFlagAlias};
use std::collections::HashMap;

// Number of bytes the branching instructions have.
const BYTES: u8 = 2;

fn base_branch(opcode: u8, flag: StatusFlagAlias, setting: bool) {
    let perform_test = |initial_setting: bool| {
        let mut cpu = Cpu::new();
        let displacement = 0xF8;

        test_prep::prepare(&mut cpu, opcode, &AddressingMode::Immediate, displacement);
        cpu.registers.status.set_flag(flag, initial_setting);

        let expected = cpu
            .registers
            .program_counter
            .wrapping_add(if setting == initial_setting {
                u8_to_u16(displacement)
            } else {
                0
            })
            .wrapping_add(BYTES.into())
            .wrapping_add(1); // PC stops on the third byte, after the bytes for opcode and operand.

        cpu.run();

        assert_eq!(cpu.registers.program_counter, expected,);
    };

    perform_test(false);
    perform_test(true);
}

#[test]
fn branch() {
    type FlagSetting = (StatusFlagAlias, bool);

    let instruction_map: HashMap<FlagSetting, u8> = HashMap::from([
        ((StatusFlagAlias::C, false), 0x90),
        ((StatusFlagAlias::C, true), 0xB0),
        ((StatusFlagAlias::Z, true), 0xF0),
        ((StatusFlagAlias::N, true), 0x30),
        ((StatusFlagAlias::Z, false), 0xD0),
        ((StatusFlagAlias::N, false), 0x10),
        ((StatusFlagAlias::V, false), 0x50),
        ((StatusFlagAlias::V, true), 0x70),
    ]);

    for ((flag, setting), code) in instruction_map {
        base_branch(code, flag, setting);
    }
}
