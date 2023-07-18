use super::{aliases::RegisterAlias, AddressingMode, Cpu};

#[cfg(test)]
use super::test_prep;

#[cfg(test)]
mod tests;

impl Cpu {
    /// ST_ - Store a Register.
    /// Stores the contents of a specified register into memory.
    fn store(&mut self, addr_mode: &AddressingMode, source: &RegisterAlias) {
        let addr = self.get_operand_address(addr_mode);
        self.memory.write(addr, self.registers.get_register(source));
    }

    /// STA - Store Accumulator.
    /// Stores the contents of the accumulator into memory.
    pub fn sta(&mut self, addr_mode: &AddressingMode) {
        self.store(addr_mode, &RegisterAlias::A);
    }

    /// STX - Store X register.
    /// Stores the contents of the X register into memory.
    pub fn stx(&mut self, addr_mode: &AddressingMode) {
        self.store(addr_mode, &RegisterAlias::X);
    }

    /// STY - Store Y register.
    /// Stores the contents of the Y register into memory.
    pub fn sty(&mut self, addr_mode: &AddressingMode) {
        self.store(addr_mode, &RegisterAlias::Y);
    }
}
