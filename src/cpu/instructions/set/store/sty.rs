use super::{AddressingMode, Cpu};

impl Cpu {
    /// STY - Store Y Register.
    /// Stores the contents of the Y register into memory.
    pub fn sty(&mut self, addr_mode: &AddressingMode) {
        let addr = self.get_operand_address(addr_mode);
        self.memory.write(addr, self.registers.index_y);
    }
}

#[cfg(test)]
mod tests {}
