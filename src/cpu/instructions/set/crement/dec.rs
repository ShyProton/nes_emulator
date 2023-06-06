use super::{AddressingMode, Cpu, CrementMode};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// DEC - Decrement Memory.
    /// Subtracts one from the value held at a specified memory location, setting the zero and
    /// negative flags as appropriate.
    pub fn dec(&mut self, addr_mode: &AddressingMode) {
        self.crement_mem(addr_mode, &CrementMode::Decrement);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        test_templates::{absolute, zero},
        CrementMode,
    };

    const CREMENT_MODE: CrementMode = CrementMode::Decrement;

    #[test]
    fn zero_underflow() {
        zero::wrapping(0xC6, &CREMENT_MODE);
    }

    #[test]
    fn zero_decrement_mem() {
        zero::crement_mem(0xC6, &CREMENT_MODE);
    }

    #[test]
    fn zero_x_decrement_mem() {
        zero::x_crement_mem(0xD6, &CREMENT_MODE);
    }

    #[test]
    fn abs_decrement_mem() {
        absolute::crement_mem(0xCE, &CREMENT_MODE);
    }

    #[test]
    fn abs_x_decrement_mem() {
        absolute::x_crement_mem(0xDE, &CREMENT_MODE);
    }
}
