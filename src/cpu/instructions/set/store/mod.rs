use super::{aliases::RegisterAlias, AddressingMode, Cpu};

mod sta;
mod stx;
mod sty;

#[cfg(test)]
mod test_templates;

impl Cpu {
    /// ST_ - Store a Register.
    /// Stores the contents of a specified register into memory.
    fn store(&mut self, addr_mode: &AddressingMode, source: &RegisterAlias) {
        let addr = self.get_operand_address(addr_mode);
        self.memory.write(addr, *self.registers.by_alias(source));
    }
}
