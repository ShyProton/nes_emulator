use super::{AddressingMode, Cpu};

impl Cpu {
    /// LDX - Load X Register.
    /// Loads a byte of memory into the X register setting the zero and negative flags as
    /// appropriate.
    pub fn ldx(&mut self, addr_mode: &AddressingMode) {
        self.registers.index_x = self.load(addr_mode);
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_templates;

    #[test]
    fn imm_load_data() {
        test_templates::imm::load_data(0xA2, 'X');
    }

    #[test]
    fn imm_zero_flag() {
        test_templates::imm::z_flag_set(0xA2);
    }

    #[test]
    fn zero_from_memory() {
        test_templates::zero::from_memory(0xA6, 'X');
    }

    #[test]
    fn zeroy_from_memory() {
        test_templates::zero::y_from_memory(0xB6, 'X');
    }

    #[test]
    fn abs_from_memory() {
        test_templates::abs::from_memory(0xAE, 'X');
    }

    #[test]
    fn absy_from_memory() {
        test_templates::abs::y_from_memory(0xBE, 'X');
    }
}
