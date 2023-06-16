use super::{AddressingMode, Cpu, ShiftType};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    pub fn asl(&mut self, addr_mode: &AddressingMode) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::test_templates::{absolute, implied, zero};

    #[test]
    fn shift() {
        todo!();
    }

    #[test]
    fn zero_shift() {
        todo!();
    }

    #[test]
    fn zero_x_shift() {
        todo!();
    }

    #[test]
    fn abs_shift() {
        todo!();
    }

    #[test]
    fn abs_x_shift() {
        todo!();
    }
}
