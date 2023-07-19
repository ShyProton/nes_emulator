use super::{aliases::RegisterAlias, AddressingMode, Cpu};

#[cfg(test)]
use super::{aliases::StatusFlagAlias, test_prep};

#[cfg(test)]
mod tests;

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

    /// LDA - Load Accumulator.
    /// Loads a byte of memory into the accumulator setting the zero and negative flags as
    /// appropriate.
    pub fn lda(&mut self, addr_mode: &AddressingMode) {
        self.load(addr_mode, &RegisterAlias::A);
    }

    /// LDX - Load X Register.
    /// Loads a byte of memory into the X register setting the zero and negative flags as
    /// appropriate.
    pub fn ldx(&mut self, addr_mode: &AddressingMode) {
        self.load(addr_mode, &RegisterAlias::X);
    }

    /// LDY - Load Y Register.
    /// Loads a byte of memory into the Y register setting the zero and negative flags as
    /// appropriate.
    pub fn ldy(&mut self, addr_mode: &AddressingMode) {
        self.load(addr_mode, &RegisterAlias::Y);
    }
}
