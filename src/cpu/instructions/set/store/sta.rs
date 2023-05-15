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

    #[test]
    fn zero_store_mem() {
        zero::store_mem(0x85, &RegisterAlias::A);
    }

    #[test]
    fn zerox_store_mem() {
        zero::x_store_mem(0x95, &RegisterAlias::A);
    }

    #[test]
    fn abs_store_mem() {
        absolute::store_mem(0x8D, &RegisterAlias::A);
    }

    #[test]
    fn absx_store_mem() {
        absolute::x_store_mem(0x9D, &RegisterAlias::A);
    }

    #[test]
    fn absy_store_mem() {
        absolute::y_store_mem(0x99, &RegisterAlias::A);
    }

    #[test]
    fn indx_store_mem() {
        indirect::x_store_mem(0x81, &RegisterAlias::A);
    }

    #[test]
    fn indy_store_mem() {
        indirect::y_store_mem(0x91, &RegisterAlias::A);
    }
}
