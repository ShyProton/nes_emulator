use super::{aliases::RegisterAlias, AddressingMode, Cpu};

#[cfg(test)]
use super::aliases::StatusFlagAlias;

mod inc;
mod inx;
mod iny;

#[cfg(test)]
mod test_templates;

impl Cpu {
    /// IN_ - Increment a Register.
    /// Adds one to some specified register setting the zero and negative flags as appropriate.
    pub fn increment(&mut self, target: &RegisterAlias) {
        let result = self.registers.by_alias(target).wrapping_add(1);
        self.update_zero_and_negative_flags(result);

        *self.registers.by_alias(target) = result;
    }
}
