use super::{Cpu, StatusFlagAlias};

pub mod implied {
    use super::*;

    pub fn check_flag(opcode: u8, flag: StatusFlagAlias, setting: bool) {
        let mut cpu = Cpu::new();

        cpu.load_program(&[opcode]);
        cpu.run();

        assert!(cpu.registers.status.get_flag(flag) == setting);
    }
}
