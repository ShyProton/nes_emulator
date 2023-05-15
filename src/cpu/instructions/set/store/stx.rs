use super::{AddressingMode, Cpu};

impl Cpu {
    /// STX - Store X Register.
    /// Stores the contents of the X register into memory.
    pub fn stx(&mut self, addr_mode: &AddressingMode) {
        let addr = self.get_operand_address(addr_mode);
        self.memory.write(addr, self.registers.index_x);
    }
}

#[cfg(test)]
mod tests {}
