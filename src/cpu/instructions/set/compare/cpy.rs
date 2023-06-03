use super::{AddressingMode, Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates::{absolute, immediate, zero};

impl Cpu {
    /// CPY - Compare Y Register.
    /// This instruction compares the contents of the Y register with another memory held value and
    /// sets the zero and carry flags as appropriate.
    pub fn cpy(&mut self, addr_mode: &AddressingMode) {
        self.compare(&RegisterAlias::Y, addr_mode);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REGISTER_ALIAS: RegisterAlias = RegisterAlias::Y;

    #[test]
    fn imm_compare() {
        immediate::compare(0xC0, &REGISTER_ALIAS);
    }

    #[test]
    fn zero_compare() {
        zero::compare(0xC4, &REGISTER_ALIAS);
    }

    #[test]
    fn abs_compare() {
        absolute::compare(0xCC, &REGISTER_ALIAS);
    }
}
