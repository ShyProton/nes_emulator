use super::{AddressingMode, Cpu, RegisterAlias};

impl Cpu {
    /// STX - Store X register.
    /// Stores the contents of the X register into memory.
    pub fn stx(&mut self, addr_mode: &AddressingMode) {
        self.store(addr_mode, &RegisterAlias::X);
    }
}
