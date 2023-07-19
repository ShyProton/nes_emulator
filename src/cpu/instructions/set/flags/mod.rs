use super::{aliases::StatusFlagAlias, Cpu};

#[cfg(test)]
use super::{test_prep, AddressingMode};

#[cfg(test)]
mod tests;

impl Cpu {
    /// CLC - Clear Carry Flag.
    /// Set the carry flag to zero.
    pub fn clc(&mut self) {
        self.registers.status.set_flag(StatusFlagAlias::C, false);
    }

    /// CLD - Clear Decimal Mode.
    /// Set the decimal mode flag to zero.
    pub fn cld(&mut self) {
        self.registers.status.set_flag(StatusFlagAlias::D, false);
    }

    /// CLI - Clear Interrupt Disable.
    /// Clears the interrupt disable flag allowing normal interrupt requests to be serviced.
    pub fn cli(&mut self) {
        self.registers.status.set_flag(StatusFlagAlias::I, false);
    }

    /// CLV - Clear Overflow Flag.
    /// Clears the overflow flag.
    pub fn clv(&mut self) {
        self.registers.status.set_flag(StatusFlagAlias::V, false);
    }

    /// SEC - Set Carry Flag.
    /// Set the carry flag to one.
    pub fn sec(&mut self) {
        self.registers.status.set_flag(StatusFlagAlias::C, true);
    }

    /// SED - Set Decimal Flag.
    /// Set the decimal mode flag to one.
    pub fn sed(&mut self) {
        self.registers.status.set_flag(StatusFlagAlias::D, true);
    }

    /// SEI - Set Interrupt Disable.
    /// Set the interrupt disable flag to one.
    pub fn sei(&mut self) {
        self.registers.status.set_flag(StatusFlagAlias::I, true);
    }
}
