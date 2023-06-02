use super::{AddressingMode, Cpu, CrementMode};

#[cfg(test)]
use super::test_templates::{absolute, zero};

impl Cpu {
    /// INC - Increment Memory.
    /// Adds one to the value held at a specified memory location, setting the zero and negative
    /// flags as appropriate.
    pub fn inc(&mut self, addr_mode: &AddressingMode) {
        self.crement_mem(addr_mode, &CrementMode::Increment);
    }
}

#[cfg(test)]
mod tests {
    use super::{absolute, zero, CrementMode};

    const CREMENT_MODE: CrementMode = CrementMode::Increment;

    #[test]
    fn zero_overflow() {
        zero::wrapping(0xE6, &CREMENT_MODE);
    }

    #[test]
    fn zero_increment_mem() {
        zero::crement_mem(0xE6, &CREMENT_MODE);
    }

    #[test]
    fn zero_x_increment_mem() {
        zero::x_crement_mem(0xF6, &CREMENT_MODE);
    }

    #[test]
    fn abs_increment_mem() {
        absolute::crement_mem(0xEE, &CREMENT_MODE);
    }

    #[test]
    fn abs_x_increment_mem() {
        absolute::x_crement_mem(0xFE, &CREMENT_MODE);
    }
}
