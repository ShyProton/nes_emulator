use super::{aliases::RegisterAlias, Cpu};

#[cfg(test)]
use super::aliases::StatusFlagAlias;

mod tax;
mod tay;
mod tsx;
mod txa;
mod txs;
mod tya;

#[cfg(test)]
mod test_templates;

impl Cpu {
    /// T__ - Transfer Between Registers.
    /// Copies the current contents of a specified register into another specified register, setting
    /// the zero and negative flags as appropriate.
    fn transfer(&mut self, source: &RegisterAlias, target: &RegisterAlias) {
        let source = self.registers.get_register(source);

        self.registers.set_register(target, source);
        self.update_zero_and_negative_flags(source);
    }
}
