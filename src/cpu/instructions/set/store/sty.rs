use super::{AddressingMode, Cpu, RegisterAlias};

impl Cpu {
    /// STY - Store Y register.
    /// Stores the contents of the Y register into memory.
    pub fn sty(&mut self, addr_mode: &AddressingMode) {
        self.store(addr_mode, &RegisterAlias::Y);
    }
}
