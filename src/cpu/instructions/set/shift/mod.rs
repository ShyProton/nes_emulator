use super::{aliases::StatusFlagAlias, AddressingMode, Cpu};

mod asl;
mod lsr;
mod rol;
mod ror;

#[cfg(test)]
mod test_templates;

#[allow(clippy::module_name_repetitions)]
pub enum ShiftType {
    Shift,
    Rotate,
}

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
                result = byte >> 1 | carry_set << (8 - 1);
                old_bit = byte & 0b0000_0001;
            }
            ShiftDirection::Left => {
                result = byte << 1 | carry_set;
                old_bit = byte & 0b1000_0000;
            }
        }

        (result, old_bit)
    }

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
}
