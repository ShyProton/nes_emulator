use super::{AddressingMode, Cpu};

impl Cpu {
    /// STA - Store Accumulator.
    /// Stores the contents of the accumulator into memory.
    pub fn sta(&mut self, addr_mode: &AddressingMode) {
        let addr = self.get_operand_address(addr_mode);
        self.memory.write(addr, self.registers.accumulator);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_store_mem() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA9, 0x55, 0x85, 0x69]);
        cpu.run();

        assert_eq!(cpu.memory.read(0x69), 0x55);
    }
}
