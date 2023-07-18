use super::{test_preperations, Cpu, RegisterAlias, StatusFlagAlias};

pub mod implied {
    use super::*;

    pub fn transfer(opcode: u8, source: &RegisterAlias, target: &RegisterAlias) {
        let mut cpu = Cpu::new();
        test_preperations::implied(&mut cpu, opcode);

        cpu.registers.set_register(source, 0x55);

        cpu.run();

        assert_eq!(cpu.registers.get_register(target), 0x55);
        assert!(!cpu.registers.status.get_flag(StatusFlagAlias::Z));
        assert!(!cpu.registers.status.get_flag(StatusFlagAlias::N));
    }

    pub fn flag_check(opcode: u8, source: &RegisterAlias) {
        let mut cpu = Cpu::new();

        test_preperations::implied(&mut cpu, opcode);
        cpu.registers.set_register(source, 0x00);
        cpu.run();

        assert!(cpu.registers.status.get_flag(StatusFlagAlias::Z));
        assert!(!cpu.registers.status.get_flag(StatusFlagAlias::N));

        test_preperations::implied(&mut cpu, opcode);
        cpu.registers.set_register(source, 0b1000_0000);
        cpu.run();

        assert!(!cpu.registers.status.get_flag(StatusFlagAlias::Z));
        assert!(cpu.registers.status.get_flag(StatusFlagAlias::N));
    }
}
