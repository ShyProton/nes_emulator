use super::{AddressingMode, Cpu};

#[cfg(test)]
use super::aliases::RegisterAlias;

mod and;
mod eor;
mod ora;

#[cfg(test)]
mod test_templates;

pub enum Operation {
    And,
    Eor,
    Ora,
}

fn perform_logical_operation(byte_1: &mut u8, byte_2: u8, operation: &Operation) {
    match operation {
        Operation::And => *byte_1 &= byte_2,
        Operation::Eor => *byte_1 ^= byte_2,
        Operation::Ora => *byte_1 |= byte_2,
    }
}

impl Cpu {
    fn logical(&mut self, addr_mode: &AddressingMode, logical_op: &Operation) {
        let addr = self.get_operand_address(addr_mode);
        let argument = self.memory.read(addr);

        perform_logical_operation(&mut self.registers.accumulator, argument, logical_op);
        self.update_zero_and_negative_flags(self.registers.accumulator);
    }
}
