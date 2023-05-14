mod addressing_mode;
mod opcode;

use super::{registers::Register, Cpu};
use addressing_mode::AddressingMode;

impl Cpu {
    /// Loads a byte of memory into the accumulator setting the zero and negative flags as appropriate.
    pub fn lda(&mut self, addr_mode: &AddressingMode) {
        let value = self.get_value(addr_mode);

        self.registers.accumulator = value;
        self.update_zero_and_negative_flags(self.registers.accumulator);
    }

    /// Loads a byte of memory into the X register setting the zero and negative flags as appropriate.
    pub fn ldx(&mut self, addr_mode: &AddressingMode) {
        let value = self.get_value(addr_mode);

        self.registers.index_x = value;
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

    /// Stores the contents of the accumulator into memory.
    pub fn sta(&mut self, addr_mode: &AddressingMode) {
        let addr = self.get_operand_address(addr_mode);
        self.memory.write(addr, self.registers.accumulator);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.registers.status.set_flag('Z', result == 0);
        self.registers.status.set_flag('N', result.get_nth_bit(7));
    }

    fn get_value(&mut self, addr_mode: &AddressingMode) -> u8 {
        let addr = self.get_operand_address(addr_mode);
        self.memory.read(addr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_ops_1() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA9, 0xC0, 0xAA, 0xE8, 0x00]);
        cpu.run();

        assert_eq!(cpu.registers.index_x, 0xC1);
    }

    // TODO: Rename tests in relation to their addressing modes.
    #[test]
    fn lda_imm_load_data() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA9, 0x05, 0x00]);
        cpu.run();

        assert_eq!(cpu.registers.accumulator, 0x05);
        assert!(!cpu.registers.status.get_flag('Z'));
        assert!(!cpu.registers.status.get_flag('N'));
    }

    #[test]
    fn lda_imm_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA9, 0x00, 0x00]);
        cpu.run();

        assert!(cpu.registers.status.get_flag('Z'));
    }

    #[test]
    fn lda_zero_from_memory() {
        let mut cpu = Cpu::new();
        cpu.memory.write(0x10, 0x55);

        cpu.load_program(&[0xA5, 0x10, 0x00]);
        cpu.run();

        assert_eq!(cpu.registers.accumulator, 0x55);
    }

    #[test]
    fn ldx_imm_load_data() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA2, 0x05, 0x00]);
        cpu.run();

        assert_eq!(cpu.registers.index_x, 0x05);
        assert!(!cpu.registers.status.get_flag('Z'));
        assert!(!cpu.registers.status.get_flag('N'));
    }

    #[test]
    fn ldx_imm_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA2, 0x00, 0x00]);
        cpu.run();

        assert!(cpu.registers.status.get_flag('Z'));
    }

    #[test]
    fn tax_move_a_to_x() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA9, 0x0A, 0xAA, 0x00]);
        cpu.run();

        assert_eq!(cpu.registers.index_x, 0x0A);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn inx_overflow() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA2, 0xFF, 0xE8, 0x00]);
        cpu.run();

        assert_eq!(cpu.registers.index_x, 1);
    }

    #[test]
    fn sta_zero_store_mem() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA9, 0x55, 0x85, 0x69]);
        cpu.run();

        assert_eq!(cpu.memory.read(0x69), 0x55);
    }
}
