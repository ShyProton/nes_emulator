use super::{Cpu, RegisterAlias, StatusFlagAlias};

pub mod implied {
    use super::*;

    pub fn push_stack(opcode: u8, target: &RegisterAlias) {
        let mut cpu = Cpu::new();
        let expected = 0b1100_1011;

        cpu.load_program(&[opcode]);
        cpu.registers.set_register(target, expected);

        cpu.run();

        cpu.registers.stack_pointer += 1;
        assert_eq!(cpu.memory.read(cpu.get_stack_addr()), expected);
    }

    pub fn pull_stack(opcode: u8, target: &RegisterAlias) {
        let mut cpu = Cpu::new();
        let expected = 0b1100_1011;

        cpu.load_program(&[opcode]);

        cpu.memory.write(cpu.get_stack_addr(), expected);
        cpu.registers.stack_pointer -= 1;

        cpu.run();

        cpu.registers.status.set_flag(StatusFlagAlias::B, false);
        assert_eq!(cpu.registers.get_register(target), expected);
    }
}
