use super::{AddressingMode, Cpu, RegisterAlias};

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
        let value = self.get_value(addr_mode);
        self.update_zero_and_negative_flags(value);

        *self.registers.by_alias(target) = value;
    }
}
