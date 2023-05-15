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
    use super::super::{
        test_templates::{absolute, immediate, zero},
        RegisterAlias,
    };

    #[test]
    fn imm_load_data() {
        immediate::load_data(0xA0, &RegisterAlias::Y);
    }

    #[test]
    fn imm_zero_flag() {
        immediate::z_flag_set(0xA0);
    }

    #[test]
    fn zero_from_memory() {
        zero::from_memory(0xA4, &RegisterAlias::Y);
    }

    #[test]
    fn zerox_from_memory() {
        zero::x_from_memory(0xB4, &RegisterAlias::Y);
    }

    #[test]
    fn abs_from_memory() {
        absolute::from_memory(0xAC, &RegisterAlias::Y);
    }

    #[test]
    fn absx_from_memory() {
        absolute::x_from_memory(0xBC, &RegisterAlias::Y);
    }
}
