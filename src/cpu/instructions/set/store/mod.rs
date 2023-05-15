use super::{AddressingMode, Cpu, RegisterAlias};

mod sta;
mod stx;
mod sty;

#[cfg(test)]
mod test_templates;

impl Cpu {
    /// ST_ - Store a Register.
    /// Stores the contents of a register into memory.
    fn store(&mut self, addr_mode: &AddressingMode, register_alias: &RegisterAlias) {
        let addr = self.get_operand_address(addr_mode);
        self.memory
            .write(addr, *self.registers.by_alias(register_alias));
    }
}
