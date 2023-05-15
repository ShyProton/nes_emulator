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
    use super::super::test_templates;

    #[test]
    fn imm_load_data() {
        test_templates::imm::load_data(0xA9, 'A');
    }

    #[test]
    fn imm_zero_flag() {
        test_templates::imm::z_flag_set(0xA9);
    }

    #[test]
    fn zero_from_memory() {
        test_templates::zero::from_memory(0xA5, 'A');
    }

    #[test]
    fn zerox_from_memory() {
        test_templates::zero::x_from_memory(0xB5, 'A');
    }

    #[test]
    fn abs_from_memory() {
        test_templates::abs::from_memory(0xAD, 'A');
    }

    #[test]
    fn absx_from_memory() {
        test_templates::abs::x_from_memory(0xBD, 'A');
    }

    #[test]
    fn absy_from_memory() {
        test_templates::abs::y_from_memory(0xB9, 'A');
    }

    #[test]
    fn indx_from_memory() {
        test_templates::ind::x_from_memory(0xA1, 'A');
    }

    #[test]
    fn indy_from_memory() {
        test_templates::ind::y_from_memory(0xB1, 'A');
    }
}
