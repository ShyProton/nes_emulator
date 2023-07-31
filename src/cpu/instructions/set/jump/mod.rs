use super::{AddressingMode, Cpu};

#[cfg(test)]
use super::test_prep;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum JumpType {
    Jump,
    Subroutine,
}

impl Cpu {
    fn jump(&mut self, addr_mode: &AddressingMode, jump_type: &JumpType) {
        let addr = self.get_operand_address(&AddressingMode::Absolute);

        let jump_location = match addr_mode {
            AddressingMode::Absolute => addr,
            AddressingMode::Implied => self.memory.read_u16(addr),
            _ => panic!("Unsupported addressing mode for jumping instruction."),
        };

        if matches!(jump_type, JumpType::Subroutine) {
            // Pushes PC - 1 onto stack.
            self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(0x01);
            self.memory.write_u16(
                self.get_stack_addr(),
                self.registers.program_counter.wrapping_sub(1),
            );
            self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(0x01);
        }

        self.registers.program_counter = jump_location;
    }

    /// JMP - Jump
    /// Sets the program counter to the address specified by the operand.
    pub fn jmp(&mut self, addr_mode: &AddressingMode) {
        self.jump(addr_mode, &JumpType::Jump);
    }

    /// JSR - Jump to Subroutine
    /// The JSR instruction pushes the address (minus one) of the return point on to the stack
    /// and then sets the program counter to the target memory address.
    pub fn jsr(&mut self) {
        self.jump(&AddressingMode::Absolute, &JumpType::Subroutine);
    }
}
