use super::{aliases::RegisterAlias, AddressingMode, Cpu};

#[cfg(test)]
use super::aliases::StatusFlagAlias;

mod lda;
mod ldx;
mod ldy;

#[cfg(test)]
mod test_templates;

impl Cpu {
    /// LD_ - Load a Register.
    /// Loads a byte of memory into some specified register, setting the zero and negative flags as
    /// appropriate.
    fn load(&mut self, addr_mode: &AddressingMode, target: &RegisterAlias) {
        let addr = self.get_operand_address(addr_mode);
        let result = self.memory.read(addr);

        self.registers.set_register(target, result);
        self.update_zero_and_negative_flags(result);
    }
}
