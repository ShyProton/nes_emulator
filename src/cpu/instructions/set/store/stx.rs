use super::{AddressingMode, Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates::{absolute, zero};

impl Cpu {
    /// STX - Store X register.
    /// Stores the contents of the X register into memory.
    pub fn stx(&mut self, addr_mode: &AddressingMode) {
        self.store(addr_mode, &RegisterAlias::X);
    }
}

#[cfg(test)]
mod tests {
    use super::{absolute, zero, RegisterAlias};

    #[test]
    fn zero_store_mem() {
        zero::store_mem(0x86, &RegisterAlias::X);
    }

    #[test]
    fn zero_y_store_mem() {
        zero::y_store_mem(0x96, &RegisterAlias::X);
    }

    #[test]
    fn abs_store_mem() {
        absolute::store_mem(0x8E, &RegisterAlias::X);
    }
}
