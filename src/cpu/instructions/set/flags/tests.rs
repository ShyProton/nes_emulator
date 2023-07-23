use super::{test_prep, AddressingMode, Cpu, StatusFlagAlias};
use std::collections::HashMap;

fn base_check_flag(opcode: u8, flag: StatusFlagAlias, setting: bool) {
    let mut cpu = Cpu::new();

    test_prep::prepare(&mut cpu, opcode, &AddressingMode::Implied, 0x00);
    cpu.registers.status.set_flag(flag, !setting);
    cpu.run();

    assert_eq!(cpu.registers.status.get_flag(flag), setting);
}

#[test]
fn check_flag() {
    type FlagSetting = (StatusFlagAlias, bool);

    let instruction_map: HashMap<FlagSetting, u8> = HashMap::from([
        ((StatusFlagAlias::C, false), 0x18), // CLC
        ((StatusFlagAlias::D, false), 0xD8), // CLD
        ((StatusFlagAlias::I, false), 0x58), // CLI
        ((StatusFlagAlias::V, false), 0xB8), // CLV
        ((StatusFlagAlias::C, true), 0x38),  // SEC
        ((StatusFlagAlias::D, true), 0xF8),  // SED
        ((StatusFlagAlias::I, true), 0x78),  // SEI
    ]);

    for ((flag, setting), code) in instruction_map {
        base_check_flag(code, flag, setting);
    }
}
