use super::{aliases::RegisterAlias, Cpu};

#[cfg(test)]
use super::{aliases::StatusFlagAlias, test_prep, AddressingMode};

#[cfg(test)]
mod tests;

impl Cpu {
    /// T__ - Transfer Between Registers.
    /// Copies the current contents of a specified register into another specified register, setting
    /// the zero and negative flags as appropriate.
    fn transfer(&mut self, source: &RegisterAlias, target: &RegisterAlias) {
        let source = self.registers.get_register(source);

        self.registers.set_register(target, source);
        self.update_zero_and_negative_flags(source);
    }

    /// TAX - Transfer Accumulator to X.
    /// Copies the current contents of the accumulator into the X register, setting the zero and
    /// negative flags as appropriate.
    pub fn tax(&mut self) {
        self.transfer(&RegisterAlias::A, &RegisterAlias::X);
    }

    /// TAY - Transfer Accumulator to Y.
    /// Copies the current contents of the accumulator into the Y register, setting the zero and
    /// negative flags as appropriate.
    pub fn tay(&mut self) {
        self.transfer(&RegisterAlias::A, &RegisterAlias::Y);
    }

    /// TSX - Transfer Stack Pointer to X.
    /// Copies the current contents of the stack register into the X register, setting the zero and
    /// negative flags as appropriate.
    pub fn tsx(&mut self) {
        self.transfer(&RegisterAlias::S, &RegisterAlias::X);
    }

    /// TXA - Transfer X to Accumulator.
    /// Copies the current contents of the X register into the accumulator, setting the zero and
    /// negative flags as appropriate.
    pub fn txa(&mut self) {
        self.transfer(&RegisterAlias::X, &RegisterAlias::A);
    }

    /// TXS - Transfer X to Stack Pointer.
    /// Copies the current contents of the X register into the stack register, setting the zero and
    /// negative flags as appropriate.
    pub fn txs(&mut self) {
        self.transfer(&RegisterAlias::X, &RegisterAlias::S);
    }

    /// TYA - Transfer Y to Accumulator.
    /// Copies the current contents of the Y register into the accumulator, setting the zero and
    /// negative flags as appropriate.
    pub fn tya(&mut self) {
        self.transfer(&RegisterAlias::Y, &RegisterAlias::A);
    }
}
