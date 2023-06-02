use super::{aliases::RegisterAlias, AddressingMode, Cpu};

mod dec;
mod dex;
mod dey;
mod inc;
mod inx;
mod iny;

#[cfg(test)]
mod test_templates;

pub enum CrementMode {
    Increment,
    Decrement,
}

impl Cpu {
    /// (IN/DE)_ - *crement a Register.
    /// Adds/subtracts one to some specified register setting the zero and negative flags as appropriate.
    pub fn crement_register(&mut self, target: &RegisterAlias, crement_mode: &CrementMode) {
        let result = match crement_mode {
            CrementMode::Increment => self.registers.by_alias(target).wrapping_add(1),
            CrementMode::Decrement => self.registers.by_alias(target).wrapping_sub(1),
        };

        self.update_zero_and_negative_flags(result);

        *self.registers.by_alias(target) = result;
    }

    /// (IN/DE)C - *crement Memory.
    /// Adds/subtracts one to the value held at a specified memory location, setting the zero and
    /// negative flags as appropriate.
    pub fn crement_mem(&mut self, addr_mode: &AddressingMode, crement_mode: &CrementMode) {
        let addr = self.get_operand_address(addr_mode);

        let result = match crement_mode {
            CrementMode::Increment => self.memory.read(addr).wrapping_add(1),
            CrementMode::Decrement => self.memory.read(addr).wrapping_sub(1),
        };

        self.update_zero_and_negative_flags(result);
        self.memory.write(addr, result);
    }
}
