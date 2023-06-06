use super::{Cpu, RegisterAlias};

pub mod implied {
    use super::*;

    pub fn push_stack(opcode: u8, target: &RegisterAlias) {
        let mut cpu = Cpu::new();
        let expected = 0x55;

        cpu.load_program(&[opcode]);
        cpu.registers.set_register(target, expected);

        cpu.run();

        let stack_val = cpu.memory.read(cpu.get_stack_addr() + 0x01);
        assert_eq!(stack_val, expected);
    }

    pub fn pull_stack(opcode: u8, target: &RegisterAlias) {
        let mut cpu = Cpu::new();
        let expected = 0x55;

        cpu.load_program(&[opcode]);
        cpu.memory.write(cpu.get_stack_addr(), expected);

        cpu.run();

        assert_eq!(cpu.registers.get_register(target), expected);
    }
}
