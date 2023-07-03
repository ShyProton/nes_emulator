use super::{AddressingMode, Cpu, ShiftDirection, ShiftType};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    pub fn ror(&mut self, addr_mode: &AddressingMode) {
        self.shift(addr_mode, &ShiftType::Rotate, &ShiftDirection::Right);
    }
}

#[cfg(test)]
mod tests {
    use super::test_templates::{absolute, implied, zero};

    #[test]
    fn rotate() {
        todo!();
    }

    #[test]
    fn zero_rotate() {
        todo!();
    }

    #[test]
    fn zero_x_rotate() {
        todo!();
    }

    #[test]
    fn abs_rotate() {
        todo!();
    }

    #[test]
    fn abs_x_rotate() {
        todo!();
    }
}
