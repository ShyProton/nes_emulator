use super::{AddressingMode, Cpu, RegisterAlias};

impl Cpu {
    /// STA - Store Accumulator.
    /// Stores the contents of the accumulator into memory.
    pub fn sta(&mut self, addr_mode: &AddressingMode) {
        self.store(addr_mode, &RegisterAlias::A);
    }
}
