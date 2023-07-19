use super::{AddressingMode, Cpu};

#[cfg(test)]
use super::test_prep;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum LogicalOperation {
    And,
    Eor,
    Ora,
}

fn perform_logical_operation(byte_1: &mut u8, byte_2: u8, operation: &LogicalOperation) {
    match operation {
        LogicalOperation::And => *byte_1 &= byte_2,
        LogicalOperation::Eor => *byte_1 ^= byte_2,
        LogicalOperation::Ora => *byte_1 |= byte_2,
    }
}

impl Cpu {
    fn logical(&mut self, addr_mode: &AddressingMode, logical_op: &LogicalOperation) {
        let addr = self.get_operand_address(addr_mode);
        let argument = self.memory.read(addr);

        perform_logical_operation(&mut self.registers.accumulator, argument, logical_op);
        self.update_zero_and_negative_flags(self.registers.accumulator);
    }

    /// AND - Logical AND.
    /// A logical AND is performed, bit by bit, on the accumulator contents using the contents of
    /// a byte of memory.
    pub fn and(&mut self, addr_mode: &AddressingMode) {
        self.logical(addr_mode, &LogicalOperation::And);
    }

    /// EOR - Exclusive OR.
    /// An exclusive OR is performed, bit by bit, on the accumulator contents using the contents of
    /// a byte of memory.
    pub fn eor(&mut self, addr_mode: &AddressingMode) {
        self.logical(addr_mode, &LogicalOperation::Eor);
    }

    /// ORA - Logical Inclusive OR.
    /// An inclusive OR is performed, bit by bit, on the accumulator contents using the contents of
    /// a byte of memory.
    pub fn ora(&mut self, addr_mode: &AddressingMode) {
        self.logical(addr_mode, &LogicalOperation::Ora);
    }
}
