use super::Cpu;

impl Cpu {
    /// Copies the current contents of the accumulator into the X register and sets the zero and
    /// negative flags as appropriate.
    pub fn tax(&mut self) {
        self.registers.index_x = self.registers.accumulator;
        self.update_zero_and_negative_flags(self.registers.index_x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_a_to_x() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA9, 0x0A, 0xAA, 0x00]);
        cpu.run();

        assert_eq!(cpu.registers.index_x, 0x0A);
    }
}
