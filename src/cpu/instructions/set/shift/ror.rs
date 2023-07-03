use super::{AddressingMode, Cpu, ShiftDirection, ShiftType};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// ROR - Rotate Right.
    /// Move each of the bits in either A or M one place to the right. Bit 7 is filled with the
    /// current value of the carry flag whilst the old bit 0 becomes the new carry flag value.
    pub fn ror(&mut self, addr_mode: &AddressingMode) {
        self.shift(addr_mode, &ShiftType::Rotate, &ShiftDirection::Right);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        test_templates::{absolute, implied, zero},
        ShiftDirection, ShiftType,
    };

    const TYPE: ShiftType = ShiftType::Rotate;
    const DIRECTION: ShiftDirection = ShiftDirection::Right;

    #[test]
    fn rotate() {
        implied::shift(0x6A, &TYPE, &DIRECTION);
    }

    #[test]
    fn zero_rotate() {
        zero::shift_mem(0x66, &TYPE, &DIRECTION);
    }

    #[test]
    fn zero_x_rotate() {
        zero::x_shift_mem(0x76, &TYPE, &DIRECTION);
    }

    #[test]
    fn abs_rotate() {
        absolute::shift_mem(0x6E, &TYPE, &DIRECTION);
    }

    #[test]
    fn abs_x_rotate() {
        absolute::x_shift_mem(0x7E, &TYPE, &DIRECTION);
    }
}
