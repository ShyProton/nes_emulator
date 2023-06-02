use super::{AddressingMode, Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates::{absolute, zero};

impl Cpu {
    /// STY - Store Y register.
    /// Stores the contents of the Y register into memory.
    pub fn sty(&mut self, addr_mode: &AddressingMode) {
        self.store(addr_mode, &RegisterAlias::Y);
    }
}

#[cfg(test)]
mod tests {
    use super::{absolute, zero, RegisterAlias};

    const REGISTER_ALIAS: RegisterAlias = RegisterAlias::Y;

    #[test]
    fn zero_store_mem() {
        zero::store_mem(0x84, &REGISTER_ALIAS);
    }

    #[test]
    fn zero_x_store_mem() {
        zero::x_store_mem(0x94, &REGISTER_ALIAS);
    }

    #[test]
    fn abs_store_mem() {
        absolute::store_mem(0x8C, &REGISTER_ALIAS);
    }
}
