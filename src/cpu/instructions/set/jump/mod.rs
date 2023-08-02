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
        let jump_location = self.get_operand_address(addr_mode);

        // Pushes PC - 1 onto stack if the instruction is JSR.
        if matches!(jump_type, JumpType::Subroutine) {
            self.push_stack_u16(self.registers.program_counter.wrapping_sub(1));
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
