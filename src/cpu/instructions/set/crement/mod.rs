use super::{aliases::RegisterAlias, AddressingMode, Cpu};

#[cfg(test)]
use super::test_prep;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum CrementMode {
    Increment,
    Decrement,
}

impl Cpu {
    /// (IN/DE)_ - *crement a Register.
    /// Adds/subtracts one to some specified register setting the zero and negative flags as appropriate.
    fn crement_register(&mut self, target: &RegisterAlias, crement_mode: &CrementMode) {
        let result = match crement_mode {
            CrementMode::Increment => self.registers.get_register(target).wrapping_add(1),
            CrementMode::Decrement => self.registers.get_register(target).wrapping_sub(1),
        };

        self.registers.set_register(target, result);
        self.update_zero_and_negative_flags(result);
    }

    /// (IN/DE)C - *crement Memory.
    /// Adds/subtracts one to the value held at a specified memory location, setting the zero and
    /// negative flags as appropriate.
    fn crement_mem(&mut self, addr_mode: &AddressingMode, crement_mode: &CrementMode) {
        let addr = self.get_operand_address(addr_mode);

        let result = match crement_mode {
            CrementMode::Increment => self.memory.read(addr).wrapping_add(1),
            CrementMode::Decrement => self.memory.read(addr).wrapping_sub(1),
        };

        self.memory.write(addr, result);
        self.update_zero_and_negative_flags(result);
    }

    /// DEC - Decrement Memory.
    /// Subtracts one from the value held at a specified memory location, setting the zero and
    /// negative flags as appropriate.
    pub fn dec(&mut self, addr_mode: &AddressingMode) {
        self.crement_mem(addr_mode, &CrementMode::Decrement);
    }

    /// DEX - Decrement the X Register.
    /// Subtracts one from the X register setting the zero and negative flags as appropriate.
    pub fn dex(&mut self) {
        self.crement_register(&RegisterAlias::X, &CrementMode::Decrement);
    }

    /// DEY - Decrement the Y Register.
    /// Subtracts one from the Y register setting the zero and negative flags as appropriate.
    pub fn dey(&mut self) {
        self.crement_register(&RegisterAlias::Y, &CrementMode::Decrement);
    }

    /// INC - Increment Memory.
    /// Adds one to the value held at a specified memory location, setting the zero and negative
    /// flags as appropriate.
    pub fn inc(&mut self, addr_mode: &AddressingMode) {
        self.crement_mem(addr_mode, &CrementMode::Increment);
    }

    /// INX - Increment the X Register.
    /// Adds one to the X register setting the zero and negative flags as appropriate.
    pub fn inx(&mut self) {
        self.crement_register(&RegisterAlias::X, &CrementMode::Increment);
    }

    /// INY - Increment the Y Register.
    /// Adds one to the Y register setting the zero and negative flags as appropriate.
    pub fn iny(&mut self) {
        self.crement_register(&RegisterAlias::Y, &CrementMode::Increment);
    }
}
