use super::{AddressingMode, Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// CMP - Compare Accumulator.
    /// This instruction compares the contents of the accumulator with another memory held value
    /// and sets the zero and carry flags as appropriate.
    pub fn cmp(&mut self, addr_mode: &AddressingMode) {
        self.compare(&RegisterAlias::A, addr_mode);
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
    fn imm_compare() {
        immediate::compare(0xC9, &REGISTER_ALIAS);
    }

    #[test]
    fn zero_compare() {
        zero::compare(0xC5, &REGISTER_ALIAS);
    }

    #[test]
    fn zero_x_compare() {
        zero::x_compare(0xD5, &REGISTER_ALIAS);
    }

    #[test]
    fn abs_compare() {
        absolute::compare(0xCD, &REGISTER_ALIAS);
    }

    #[test]
    fn abs_x_compare() {
        absolute::x_compare(0xDD, &REGISTER_ALIAS);
    }

    #[test]
    fn abs_y_compare() {
        absolute::y_compare(0xD9, &REGISTER_ALIAS);
    }

    #[test]
    fn ind_x_compare() {
        indirect::x_compare(0xC1, &REGISTER_ALIAS);
    }

    #[test]
    fn ind_y_compare() {
        indirect::y_compare(0xD1, &REGISTER_ALIAS);
    }
}
