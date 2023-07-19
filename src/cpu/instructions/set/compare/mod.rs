use std::cmp::Ordering;

use super::{
    aliases::{RegisterAlias, StatusFlagAlias},
    AddressingMode, Cpu,
};

#[cfg(test)]
use super::test_prep;

#[cfg(test)]
mod tests;

impl Cpu {
    /// CP_ - Compare a register.
    /// This instruction compares the contents of a given register with another memory held value
    /// and sets the zero and carry flags as appropriate.
    pub fn compare(&mut self, target: &RegisterAlias, addr_mode: &AddressingMode) {
        let addr = self.get_operand_address(addr_mode);
        let register_value = self.registers.get_register(target);

        let comparison = register_value.cmp(&self.memory.read(addr));

        self.update_zero_and_negative_flags(register_value);

        self.registers
            .status
            .set_flag(StatusFlagAlias::C, comparison != Ordering::Less)
            .set_flag(StatusFlagAlias::Z, comparison == Ordering::Equal);
    }

    /// CMP - Compare Accumulator.
    /// This instruction compares the contents of the accumulator with another memory held value
    /// and sets the zero and carry flags as appropriate.
    pub fn cmp(&mut self, addr_mode: &AddressingMode) {
        self.compare(&RegisterAlias::A, addr_mode);
    }

    /// CPX - Compare X Register.
    /// This instruction compares the contents of the X register with another memory held value and
    /// sets the zero and carry flags as appropriate.
    pub fn cpx(&mut self, addr_mode: &AddressingMode) {
        self.compare(&RegisterAlias::X, addr_mode);
    }

    /// CPY - Compare Y Register.
    /// This instruction compares the contents of the Y register with another memory held value and
    /// sets the zero and carry flags as appropriate.
    pub fn cpy(&mut self, addr_mode: &AddressingMode) {
        self.compare(&RegisterAlias::Y, addr_mode);
    }
}
