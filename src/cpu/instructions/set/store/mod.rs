use super::{aliases::RegisterAlias, AddressingMode, Cpu};

#[cfg(test)]
use super::test_prep;

mod sta;
mod stx;
mod sty;

#[cfg(test)]
mod tests;

impl Cpu {
    /// ST_ - Store a Register.
    /// Stores the contents of a specified register into memory.
    fn store(&mut self, addr_mode: &AddressingMode, source: &RegisterAlias) {
        let addr = self.get_operand_address(addr_mode);
        self.memory.write(addr, self.registers.get_register(source));
    }
}
