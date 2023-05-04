use super::{registers::Register, Cpu};

impl Cpu {
    /// Loads a byte of memory into the accumulator setting the zero and negative flags as appropriate.
    pub fn lda(&mut self, program: &[u8]) {
        let param = self.get_param(program);

        self.registers.accumulator = param;
        self.update_zero_and_negative_flags(self.registers.accumulator);
    }

    /// Loads a byte of memory into the X register setting the zero and negative flags as appropriate.
    pub fn ldx(&mut self, program: &[u8]) {
        let param = self.get_param(program);

        self.registers.index_x = param;
        self.update_zero_and_negative_flags(self.registers.index_x);
    }

    /// Copies the current contents of the accumulator into the X register and sets the zero and
    /// negative flags as appropriate.
    pub fn tax(&mut self) {
        self.registers.index_x = self.registers.accumulator;
        self.update_zero_and_negative_flags(self.registers.index_x);
    }

    /// Adds one to the X register setting the zero and negative flags as appropriate.
    pub fn inx(&mut self) {
        self.registers.index_x += 1;
        self.update_zero_and_negative_flags(self.registers.index_x);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.registers.status.set_flag('Z', result == 0);
        self.registers.status.set_flag('N', result.get_nth_bit(7));
    }

    fn get_param(&mut self, program: &[u8]) -> u8 {
        let param = program[self.registers.program_counter as usize];
        self.registers.program_counter += 1;

        param
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_ops_1() {
        let mut cpu = Cpu::new();
        cpu.interpret(&[0xA9, 0xC0, 0xAA, 0xE8, 0x00]);

        assert_eq!(cpu.registers.index_x, 0xC1);
    }

    #[test]
    fn lda_0xa9_immediate_load_data() {
        let mut cpu = Cpu::new();
        cpu.interpret(&[0xA9, 0x05, 0x00]);

        assert_eq!(cpu.registers.accumulator, 0x05);
        assert!(!cpu.registers.status.get_flag('Z'));
        assert!(!cpu.registers.status.get_flag('N'));
    }

    #[test]
    fn lda_0xa9_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.interpret(&[0xA9, 0x00, 0x00]);

        assert!(cpu.registers.status.get_flag('Z'));
    }

    #[test]
    fn ldx_0xa2_immediate_load_data() {
        let mut cpu = Cpu::new();
        cpu.interpret(&[0xA2, 0x05, 0x00]);

        assert_eq!(cpu.registers.index_x, 0x05);
        assert!(!cpu.registers.status.get_flag('Z'));
        assert!(!cpu.registers.status.get_flag('N'));
    }

    #[test]
    fn ldx_0xa2_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.interpret(&[0xA2, 0x00, 0x00]);

        assert!(cpu.registers.status.get_flag('Z'));
    }

    #[test]
    fn tax_0xaa_move_a_to_x() {
        let mut cpu = Cpu::new();
        cpu.interpret(&[0xA9, 0x0A, 0xAA, 0x00]);

        assert_eq!(cpu.registers.index_x, 0x0A);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn inx_0xe8_overflow() {
        let mut cpu = Cpu::new();
        cpu.interpret(&[0xA2, 0xFF, 0xE8, 0x00]);

        assert_eq!(cpu.registers.index_x, 1);
    }
}
