use super::{aliases::StatusFlagAlias, AddressingMode, Cpu};

#[cfg(test)]
use super::test_prep;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum ShiftType {
    Shift,
    Rotate,
}

#[derive(PartialEq, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum ShiftDirection {
    Left,
    Right,
}

impl Cpu {
    fn find_result_and_old_bit(
        &self,
        byte: u8,
        shift_type: &ShiftType,
        shift_direction: &ShiftDirection,
    ) -> (u8, u8) {
        let (result, old_bit);

        let carry_set = match shift_type {
            ShiftType::Shift => 0,
            ShiftType::Rotate => u8::from(self.registers.status.get_flag(StatusFlagAlias::C)),
        };

        match shift_direction {
            ShiftDirection::Right => {
                result = byte >> 1 | carry_set << 7;
                old_bit = byte & 0b0000_0001;
            }
            ShiftDirection::Left => {
                result = byte << 1 | carry_set;
                old_bit = byte & 0b0000_0001 << 7;
            }
        }

        (result, old_bit)
    }

    /// Base for shifting/rotating.
    fn shift(
        &mut self,
        addr_mode: &AddressingMode,
        shift_type: &ShiftType,
        shift_direction: &ShiftDirection,
    ) {
        let (result, old_bit);

        if matches!(addr_mode, AddressingMode::Implied) {
            (result, old_bit) = self.find_result_and_old_bit(
                self.registers.accumulator,
                shift_type,
                shift_direction,
            );

            self.registers.accumulator = result;
        } else {
            let addr = self.get_operand_address(addr_mode);
            let argument = self.memory.read(addr);

            (result, old_bit) = self.find_result_and_old_bit(argument, shift_type, shift_direction);

            self.memory.write(addr, result);
        };

        self.registers
            .status
            .set_flag(StatusFlagAlias::C, old_bit != 0);

        self.update_zero_and_negative_flags(result);
    }

    /// ASL - Arithmetic Shift Left.
    /// This operation shifts all the bits of the accumulator or memory contents one bit left. Bit 0
    /// is set to 0 and bit 7 is placed in the carry flag. The effect of this operation is to
    /// multiply the memory contents by 2 (ignoring 2's complement considerations), setting the
    /// carry if the result will not fit in 8 bits.
    pub fn asl(&mut self, addr_mode: &AddressingMode) {
        self.shift(addr_mode, &ShiftType::Shift, &ShiftDirection::Left);
    }

    /// LSR - Logical Shift Right.
    /// Each of the bits in A or M is shift one place to the right. The bit that was in bit 0 is
    /// shifted into the carry flag. Bit 7 is set to zero.
    pub fn lsr(&mut self, addr_mode: &AddressingMode) {
        self.shift(addr_mode, &ShiftType::Shift, &ShiftDirection::Right);
    }

    /// ROL - Rotate Left.
    /// Move each of the bits in either A or M one place to the left. Bit 0 is filled with the
    /// current value of the carry flag whilst the old bit 7 becomes the new carry flag value.
    pub fn rol(&mut self, addr_mode: &AddressingMode) {
        self.shift(addr_mode, &ShiftType::Rotate, &ShiftDirection::Left);
    }

    /// ROR - Rotate Right.
    /// Move each of the bits in either A or M one place to the right. Bit 7 is filled with the
    /// current value of the carry flag whilst the old bit 0 becomes the new carry flag value.
    pub fn ror(&mut self, addr_mode: &AddressingMode) {
        self.shift(addr_mode, &ShiftType::Rotate, &ShiftDirection::Right);
    }
}
