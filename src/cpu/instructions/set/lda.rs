use super::{AddressingMode, Cpu};

impl Cpu {
    /// Loads a byte of memory into the accumulator setting the zero and negative flags as appropriate.
    pub fn lda(&mut self, addr_mode: &AddressingMode) {
        let value = self.get_value(addr_mode);

        self.registers.accumulator = value;
        self.update_zero_and_negative_flags(self.registers.accumulator);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imm_load_data() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA9, 0x05, 0x00]);
        cpu.run();

        assert_eq!(cpu.registers.accumulator, 0x05);
        assert!(!cpu.registers.status.get_flag('Z'));
        assert!(!cpu.registers.status.get_flag('N'));
    }

    #[test]
    fn imm_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA9, 0x00, 0x00]);
        cpu.run();

        assert!(cpu.registers.status.get_flag('Z'));
    }

    #[test]
    fn zero_from_memory() {
        let mut cpu = Cpu::new();
        cpu.memory.write(0x10, 0x55);

        cpu.load_program(&[0xA5, 0x10, 0x00]);
        cpu.run();

        assert_eq!(cpu.registers.accumulator, 0x55);
    }
}
