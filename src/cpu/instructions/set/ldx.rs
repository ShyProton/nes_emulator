use super::{AddressingMode, Cpu};

impl Cpu {
    /// Loads a byte of memory into the X register setting the zero and negative flags as appropriate.
    pub fn ldx(&mut self, addr_mode: &AddressingMode) {
        let value = self.get_value(addr_mode);

        self.registers.index_x = value;
        self.update_zero_and_negative_flags(self.registers.index_x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imm_load_data() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA2, 0x05, 0x00]);
        cpu.run();

        assert_eq!(cpu.registers.index_x, 0x05);
        assert!(!cpu.registers.status.get_flag('Z'));
        assert!(!cpu.registers.status.get_flag('N'));
    }

    #[test]
    fn imm_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA2, 0x00, 0x00]);
        cpu.run();

        assert!(cpu.registers.status.get_flag('Z'));
    }
}
