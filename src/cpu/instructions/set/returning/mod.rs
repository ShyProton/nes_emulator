use super::Cpu;

#[cfg(test)]
use super::{test_prep, AddressingMode};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Eq, Hash, Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum ReturnMode {
    Interrupt,
    Subroutine,
}

impl Cpu {
    /// Base for returning instructions.
    fn return_from(&mut self, return_mode: &ReturnMode) {
        let subtraction = match return_mode {
            ReturnMode::Interrupt => {
                self.plp();
                0x0000
            }
            ReturnMode::Subroutine => 0x0001,
        };

        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(0x01);

        self.registers.program_counter = self
            .memory
            .read_u16(self.get_stack_addr())
            .wrapping_sub(subtraction);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(0x01);
    }

    /// RTI - Return from Interrupt.
    /// The RTI instruction is used at the end of an interrupt processing routine. It pulls the
    /// processor flags from the stack followed by the program counter.
    pub fn rti(&mut self) {
        self.return_from(&ReturnMode::Interrupt);
    }

    /// RTS - Reutnr from Subroutine.
    /// The RTS instruction is used at the end of a subroutine to return to the calling routine.
    /// It pulls the program counter (minus one) from the stack.
    pub fn rts(&mut self) {
        self.return_from(&ReturnMode::Subroutine);
    }
}
