use super::{AddressingMode, Cpu, ShiftDirection, ShiftType};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// LSR - Logical Shift Right.
    /// Each of the bits in A or M is shift one place to the right. The bit that was in bit 0 is
    /// shifted into the carry flag. Bit 7 is set to zero.
    pub fn lsr(&mut self, addr_mode: &AddressingMode) {
        self.shift(addr_mode, &ShiftType::Shift, &ShiftDirection::Right);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        test_templates::{absolute, implied, zero},
        ShiftDirection, ShiftType,
    };

    const TYPE: ShiftType = ShiftType::Shift;
    const DIRECTION: ShiftDirection = ShiftDirection::Right;

    #[test]
    fn shift() {
        implied::shift(0x4A, &TYPE, &DIRECTION);
    }

    #[test]
    fn zero_shift() {
        zero::shift_mem(0x46, &TYPE, &DIRECTION);
    }

    #[test]
    fn zero_x_shift() {
        zero::x_shift_mem(0x56, &TYPE, &DIRECTION);
    }

    #[test]
    fn abs_shift() {
        absolute::shift_mem(0x4E, &TYPE, &DIRECTION);
    }

    #[test]
    fn abs_x_shift() {
        absolute::x_shift_mem(0x5E, &TYPE, &DIRECTION);
    }
}
