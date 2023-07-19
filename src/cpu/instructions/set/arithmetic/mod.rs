use super::{aliases::StatusFlagAlias, AddressingMode, Cpu};

#[cfg(test)]
use super::test_prep;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum ArithmeticMode {
    Addition,
    Subtraction,
}

const fn overflow_occurred(operand_1: u8, operand_2: u8, result: u8) -> bool {
    let sign_operand_1 = operand_1 & 0b1000_0000 != 0;
    let sign_opreand_2 = operand_2 & 0b1000_0000 != 0;
    let sign_result = result & 0b1000_0000 != 0;

    // Overflow only occurs if both operand's signs are the same, AND the result's sign differs
    // from the operand's
    sign_operand_1 == sign_opreand_2 && sign_operand_1 != sign_result
}

fn carry_occurred(operand_1: u8, operand_2: u8, carry_bit: u8, result: u8) -> bool {
    // Carry only occurs if the result is less than any of the operands.
    [operand_1, operand_2, carry_bit]
        .iter()
        .any(|&n| result < n)
}

impl Cpu {
    /// Base for adding/subtracting.
    fn arithmetic(&mut self, addr_mode: &AddressingMode, arithmetic_mode: &ArithmeticMode) {
        let addr = self.get_operand_address(addr_mode);

        let operand = match arithmetic_mode {
            ArithmeticMode::Addition => self.memory.read(addr),
            ArithmeticMode::Subtraction => !self.memory.read(addr), // SBC #num = ADC #~num
        };

        let carry_bit = u8::from(self.registers.status.get_flag(StatusFlagAlias::C));

        let result = self
            .registers
            .accumulator
            .wrapping_add(operand)
            .wrapping_add(carry_bit);

        let overflow_occurred = overflow_occurred(self.registers.accumulator, operand, result);
        let carry_occurred = carry_occurred(self.registers.accumulator, operand, carry_bit, result);

        self.registers.accumulator = result;

        self.update_zero_and_negative_flags(self.registers.accumulator);
        self.registers
            .status
            .set_flag(StatusFlagAlias::C, carry_occurred)
            .set_flag(StatusFlagAlias::V, overflow_occurred);
    }

    /// ADC - Add with Carry.
    /// This instruction adds the contents of a memory location to the
    /// accumulator together with the carry bit. If verflow occurs the carry bit
    /// is set, this enables multiple byte addition to be performed.
    pub fn adc(&mut self, addr_mode: &AddressingMode) {
        self.arithmetic(addr_mode, &ArithmeticMode::Addition);
    }

    /// SBC - Subtract with Carry.
    /// This instructions subtracts the contents of a memory location to the
    /// accumulator together with the not of the carry bit. If overflow occurs
    /// the carry bit is clear, this enabled multiple byte subtraction to be
    /// performed.
    pub fn sbc(&mut self, subr_mode: &AddressingMode) {
        self.arithmetic(subr_mode, &ArithmeticMode::Subtraction);
    }
}
