use super::{AddressingMode, Cpu, ShiftDirection, ShiftType};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// ROL - Rotate Left.
    /// Move each of the bits in either A or M one place to the left. Bit 0 is filled with the
    /// current value of the carry flag whilst the old bit 7 becomes the new carry flag value.
    pub fn rol(&mut self, addr_mode: &AddressingMode) {
        self.shift(addr_mode, &ShiftType::Rotate, &ShiftDirection::Left);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        test_templates::{absolute, implied, zero},
        ShiftDirection, ShiftType,
    };

    const TYPE: ShiftType = ShiftType::Rotate;
    const DIRECTION: ShiftDirection = ShiftDirection::Left;

    #[test]
    fn rotate() {
        implied::shift(0x2A, &TYPE, &DIRECTION);
    }

    #[test]
    fn zero_rotate() {
        zero::shift_mem(0x26, &TYPE, &DIRECTION);
    }

    #[test]
    fn zero_x_rotate() {
        zero::x_shift_mem(0x36, &TYPE, &DIRECTION);
    }

    #[test]
    fn abs_rotate() {
        absolute::shift_mem(0x2E, &TYPE, &DIRECTION);
    }

    #[test]
    fn abs_x_rotate() {
        absolute::x_shift_mem(0x3E, &TYPE, &DIRECTION);
    }
}
