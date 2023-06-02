use super::{AddressingMode, Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates::{absolute, indirect, zero};

impl Cpu {
    /// STA - Store Accumulator.
    /// Stores the contents of the accumulator into memory.
    pub fn sta(&mut self, addr_mode: &AddressingMode) {
        self.store(addr_mode, &RegisterAlias::A);
    }
}

#[cfg(test)]
mod tests {
    use super::{absolute, indirect, zero, RegisterAlias};

    const REGISTER_ALIAS: RegisterAlias = RegisterAlias::A;

    #[test]
    fn zero_store_mem() {
        zero::store_mem(0x85, &REGISTER_ALIAS);
    }

    #[test]
    fn zero_x_store_mem() {
        zero::x_store_mem(0x95, &REGISTER_ALIAS);
    }

    #[test]
    fn abs_store_mem() {
        absolute::store_mem(0x8D, &REGISTER_ALIAS);
    }

    #[test]
    fn abs_x_store_mem() {
        absolute::x_store_mem(0x9D, &REGISTER_ALIAS);
    }

    #[test]
    fn abs_y_store_mem() {
        absolute::y_store_mem(0x99, &REGISTER_ALIAS);
    }

    #[test]
    fn ind_x_store_mem() {
        indirect::x_store_mem(0x81, &REGISTER_ALIAS);
    }

    #[test]
    fn ind_y_store_mem() {
        indirect::y_store_mem(0x91, &REGISTER_ALIAS);
    }
}
