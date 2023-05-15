use super::{AddressingMode, Cpu};

impl Cpu {
    /// LDA - Load Accumulator.
    /// Loads a byte of memory into the accumulator setting the zero and negative flags as
    /// appropriate.
    pub fn lda(&mut self, addr_mode: &AddressingMode) {
        self.registers.accumulator = self.load(addr_mode);
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_templates::{absolute, immediate, indirect, zero};

    #[test]
    fn imm_load_data() {
        immediate::load_data(0xA9, 'A');
    }

    #[test]
    fn imm_zero_flag() {
        immediate::z_flag_set(0xA9);
    }

    #[test]
    fn zero_from_memory() {
        zero::from_memory(0xA5, 'A');
    }

    #[test]
    fn zerox_from_memory() {
        zero::x_from_memory(0xB5, 'A');
    }

    #[test]
    fn abs_from_memory() {
        absolute::from_memory(0xAD, 'A');
    }

    #[test]
    fn absx_from_memory() {
        absolute::x_from_memory(0xBD, 'A');
    }

    #[test]
    fn absy_from_memory() {
        absolute::y_from_memory(0xB9, 'A');
    }

    #[test]
    fn indx_from_memory() {
        indirect::x_from_memory(0xA1, 'A');
    }

    #[test]
    fn indy_from_memory() {
        indirect::y_from_memory(0xB1, 'A');
    }
}
