use super::{AddressingMode, Cpu};

impl Cpu {
    /// LDY - Load Y Register.
    /// Loads a byte of memory into the Y register setting the zero and negative flags as
    /// appropriate.
    pub fn ldy(&mut self, addr_mode: &AddressingMode) {
        self.registers.index_y = self.load(addr_mode);
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_templates;

    #[test]
    fn imm_load_data() {
        test_templates::imm::load_data(0xA0, 'Y');
    }

    #[test]
    fn imm_zero_flag() {
        test_templates::imm::z_flag_set(0xA0);
    }

    #[test]
    fn zero_from_memory() {
        test_templates::zero::from_memory(0xA4, 'Y');
    }

    #[test]
    fn zerox_from_memory() {
        test_templates::zero::x_from_memory(0xB4, 'Y');
    }

    #[test]
    fn abs_from_memory() {
        test_templates::abs::from_memory(0xAC, 'Y');
    }

    #[test]
    fn absx_from_memory() {
        test_templates::abs::x_from_memory(0xBC, 'Y');
    }
}
