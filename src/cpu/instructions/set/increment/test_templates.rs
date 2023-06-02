use super::{Cpu, RegisterAlias, StatusFlagAlias};

pub mod immediate {
    use super::*;

    pub fn increment(opcode: u8, target: &RegisterAlias) {
        let mut cpu = Cpu::new();

        cpu.load_program(&[opcode]);
        *cpu.registers.by_alias(target) = 0x01;

        cpu.run();

        assert_eq!(*cpu.registers.by_alias(target), 0x02);
    }

    pub fn overflow(opcode: u8, target: &RegisterAlias) {
        let mut cpu = Cpu::new();

        cpu.load_program(&[opcode]);
        *cpu.registers.by_alias(target) = 0xFF;

        cpu.run();

        assert_eq!(*cpu.registers.by_alias(target), 0x00);
        assert!(cpu.registers.status.get_flag(StatusFlagAlias::Z));
    }
}
