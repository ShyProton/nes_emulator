use super::{aliases::StatusFlagAlias, AddressingMode, Cpu};

impl Cpu {
    /// BIT - Bit Test.
    /// This instructions is used to test if one or more bits are set in a target memory location.
    /// The mask pattern in A is ``ANDed`` with the value in memory to set or clear the zero flag,
    /// but the result is not kept. Bits 7 and 6 of the value from memory are copied into the N and
    /// V flags.
    pub fn bit(&mut self, addr_mode: &AddressingMode) {
        let addr = self.get_operand_address(addr_mode);
        let mem_value = self.memory.read(addr);

        let result = self.registers.accumulator & mem_value;

        self.registers
            .status
            .set_flag(StatusFlagAlias::Z, result != 0)
            .set_flag(StatusFlagAlias::V, mem_value & (0b0100_0001 << 6) != 0)
            .set_flag(StatusFlagAlias::N, mem_value & (0b1000_0001 << 7) != 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_bit_test() {
        let mut cpu = Cpu::new();

        cpu.load_program(&[0x24, 0x69]);

        cpu.registers.accumulator = 0b1010_1010;
        cpu.memory.write(0x0069, 0b0101_0101);

        cpu.run();

        assert!(!cpu.registers.status.get_flag(StatusFlagAlias::Z));
        assert!(cpu.registers.status.get_flag(StatusFlagAlias::V));
        assert!(!cpu.registers.status.get_flag(StatusFlagAlias::N));
    }

    #[test]
    fn abs_bit_test() {
        let mut cpu = Cpu::new();

        cpu.load_program(&[0x2C, 0x42, 0x69]);

        cpu.registers.accumulator = 0b0101_0111;
        cpu.memory.write(0x6942, 0b1010_1010);

        cpu.run();

        assert!(cpu.registers.status.get_flag(StatusFlagAlias::Z));
        assert!(!cpu.registers.status.get_flag(StatusFlagAlias::V));
        assert!(cpu.registers.status.get_flag(StatusFlagAlias::N));
    }
}
