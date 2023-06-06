use super::{AddressingMode, Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// STX - Store X register.
    /// Stores the contents of the X register into memory.
    pub fn stx(&mut self, addr_mode: &AddressingMode) {
        self.store(addr_mode, &RegisterAlias::X);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        test_templates::{absolute, zero},
        RegisterAlias,
    };

    const REGISTER_ALIAS: RegisterAlias = RegisterAlias::X;

    #[test]
    fn zero_store_mem() {
        zero::store_mem(0x86, &REGISTER_ALIAS);
    }

    #[test]
    fn zero_y_store_mem() {
        zero::y_store_mem(0x96, &REGISTER_ALIAS);
    }

    #[test]
    fn abs_store_mem() {
        absolute::store_mem(0x8E, &REGISTER_ALIAS);
    }
}
