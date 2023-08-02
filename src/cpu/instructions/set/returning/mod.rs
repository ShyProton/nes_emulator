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
        let addition = match return_mode {
            ReturnMode::Interrupt => {
                self.plp();
                0x0000
            }
            ReturnMode::Subroutine => 0x0001,
        };

        self.registers.program_counter = self.pull_stack_u16().wrapping_add(addition);
    }

    /// RTI - Return from Interrupt.
    /// The RTI instruction is used at the end of an interrupt processing routine. It pulls the
    /// processor flags from the stack followed by the program counter.
    pub fn rti(&mut self) {
        self.return_from(&ReturnMode::Interrupt);
    }

    /// RTS - Return from Subroutine.
    /// The RTS instruction is used at the end of a subroutine to return to the calling routine.
    /// It pulls the program counter (minus one) from the stack.
    pub fn rts(&mut self) {
        self.return_from(&ReturnMode::Subroutine);
    }
}
