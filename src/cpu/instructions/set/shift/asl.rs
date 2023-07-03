use super::{AddressingMode, Cpu, ShiftDirection, ShiftType};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// ASL - Arithmetic Shift Left.
    /// This operation shifts all the bits of the accumulator or memory contents one bit left. Bit 0
    /// is set to 0 and bit 7 is placed in the carry flag. The effect of this operation is to
    /// multiply the memory contents by 2 (ignoring 2's complement considerations), setting the
    /// carry if the result will not fit in 8 bits.
    pub fn asl(&mut self, addr_mode: &AddressingMode) {
        self.shift(addr_mode, &ShiftType::Shift, &ShiftDirection::Left);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        test_templates::{absolute, implied, zero},
        ShiftDirection, ShiftType,
    };

    const TYPE: ShiftType = ShiftType::Shift;
    const DIRECTION: ShiftDirection = ShiftDirection::Left;

    #[test]
    fn shift() {
        implied::shift(0x0A, &TYPE, &DIRECTION);
    }

    #[test]
    fn zero_shift() {
        zero::shift_mem(0x06, &TYPE, &DIRECTION);
    }

    #[test]
    fn zero_x_shift() {
        zero::x_shift_mem(0x16, &TYPE, &DIRECTION);
    }

    #[test]
    fn abs_shift() {
        absolute::shift_mem(0x0E, &TYPE, &DIRECTION);
    }

    #[test]
    fn abs_x_shift() {
        absolute::x_shift_mem(0x1E, &TYPE, &DIRECTION);
    }
}
