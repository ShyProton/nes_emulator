use super::{Cpu, RegisterAlias};

pub mod implied {
    use super::{Cpu, RegisterAlias};

    pub fn transfer(opcode: u8, source: &RegisterAlias, target: &RegisterAlias) {
        let mut cpu = Cpu::new();
        cpu.load_program(&[opcode]);

        *cpu.registers.by_alias(source) = 0x55;

        cpu.run();

        assert_eq!(*cpu.registers.by_alias(target), 0x55);
        assert!(!cpu.registers.status.get_flag('Z'));
        assert!(!cpu.registers.status.get_flag('N'));
    }

    pub fn flag_check(opcode: u8, source: &RegisterAlias) {
        let mut cpu = Cpu::new();

        cpu.load_program(&[opcode]);
        *cpu.registers.by_alias(source) = 0x00;
        cpu.run();

        assert!(cpu.registers.status.get_flag('Z'));
        assert!(!cpu.registers.status.get_flag('N'));

        cpu.load_program(&[opcode]);
        *cpu.registers.by_alias(source) = 0b1000_0000;
        cpu.run();

        assert!(!cpu.registers.status.get_flag('Z'));
        assert!(cpu.registers.status.get_flag('N'));
    }
}
