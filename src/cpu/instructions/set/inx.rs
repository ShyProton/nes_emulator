use super::Cpu;

impl Cpu {
    /// INX - Increment X Register.
    /// Adds one to the X register setting the zero and negative flags as appropriate.
    pub fn inx(&mut self) {
        self.registers.index_x += 1;
        self.update_zero_and_negative_flags(self.registers.index_x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Normal use-case test w/out overflow.

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn overflow_check() {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA2, 0xFF, 0xE8, 0x00]);
        cpu.run();

        assert_eq!(cpu.registers.index_x, 1);
    }
}
