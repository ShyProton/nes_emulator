use super::{AddressingMode, Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// LDA - Load Accumulator.
    /// Loads a byte of memory into the accumulator setting the zero and negative flags as
    /// appropriate.
    pub fn lda(&mut self, addr_mode: &AddressingMode) {
        self.load(addr_mode, &RegisterAlias::A);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        test_templates::{absolute, immediate, indirect, zero},
        RegisterAlias,
    };

    const REGISTER_ALIAS: RegisterAlias = RegisterAlias::A;

    #[test]
    fn imm_load_data() {
        immediate::load_data(0xA9, &REGISTER_ALIAS);
    }

    #[test]
    fn imm_zero_flag() {
        immediate::flag_check(0xA9);
    }

    #[test]
    fn zero_from_memory() {
        zero::from_memory(0xA5, &REGISTER_ALIAS);
    }

    #[test]
    fn zero_x_from_memory() {
        zero::x_from_memory(0xB5, &REGISTER_ALIAS);
    }

    #[test]
    fn abs_from_memory() {
        absolute::from_memory(0xAD, &REGISTER_ALIAS);
    }

    #[test]
    fn abs_x_from_memory() {
        absolute::x_from_memory(0xBD, &REGISTER_ALIAS);
    }

    #[test]
    fn abs_y_from_memory() {
        absolute::y_from_memory(0xB9, &REGISTER_ALIAS);
    }

    #[test]
    fn ind_x_from_memory() {
        indirect::x_from_memory(0xA1, &REGISTER_ALIAS);
    }

    #[test]
    fn ind_y_from_memory() {
        indirect::y_from_memory(0xB1, &REGISTER_ALIAS);
    }
}
