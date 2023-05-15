use super::{AddressingMode, Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates::{absolute, immediate, zero};

impl Cpu {
    /// LDX - Load X Register.
    /// Loads a byte of memory into the X register setting the zero and negative flags as
    /// appropriate.
    pub fn ldx(&mut self, addr_mode: &AddressingMode) {
        self.load(addr_mode, &RegisterAlias::X);
    }
}

#[cfg(test)]
mod tests {
    use super::{absolute, immediate, zero, RegisterAlias};

    #[test]
    fn imm_load_data() {
        immediate::load_data(0xA2, &RegisterAlias::X);
    }

    #[test]
    fn imm_zero_flag() {
        immediate::flag_check(0xA2);
    }

    #[test]
    fn zero_from_memory() {
        zero::from_memory(0xA6, &RegisterAlias::X);
    }

    #[test]
    fn zero_y_from_memory() {
        zero::y_from_memory(0xB6, &RegisterAlias::X);
    }

    #[test]
    fn abs_from_memory() {
        absolute::from_memory(0xAE, &RegisterAlias::X);
    }

    #[test]
    fn abs_y_from_memory() {
        absolute::y_from_memory(0xBE, &RegisterAlias::X);
    }
}
