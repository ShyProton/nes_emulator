use super::{aliases::RegisterAlias, Cpu};

#[cfg(test)]
use super::{aliases::StatusFlagAlias, test_prep, AddressingMode};

#[cfg(test)]
mod tests;

impl Cpu {
    pub fn get_stack_addr(&self) -> u16 {
        0x0100 + u16::from(self.registers.stack_pointer)
    }

    /// PH_ - Push Register.
    /// Pushes a copy of a register on to the stack.
    fn push_stack(&mut self, target: &RegisterAlias) {
        let register_val = self.registers.get_register(target);

        self.memory.write(self.get_stack_addr(), register_val);

        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(0x01);
    }

    /// PL_ - Pull Register.
    /// Pulls an 8 bit value from the stack into a register.
    fn pull_stack(&mut self, target: &RegisterAlias) {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(0x01);

        let stack_val = self.memory.read(self.get_stack_addr());

        self.registers.set_register(target, stack_val);
    }

    /// PHA - Push Accumulator.
    /// Pushes a copy of the accumulator on to the stack.
    pub fn pha(&mut self) {
        self.push_stack(&RegisterAlias::A);
    }

    /// PHP - Push Processor Status.
    /// Pushes a copy of the status flags on to the stack.
    pub fn php(&mut self) {
        self.push_stack(&RegisterAlias::P);
    }

    /// PLA - Pull Accumulator.
    /// Pulls an 8 bit value from the stack and into the accumulator. The zero and negative flags
    /// are set as appropriate.
    pub fn pla(&mut self) {
        self.pull_stack(&RegisterAlias::A);
        self.update_zero_and_negative_flags(self.registers.accumulator);
    }

    /// PLP - Pull Processor Status.
    /// Pulls an 8 bit value from the stack and into the processor flags. The flags will take on
    /// new states as determined by the value pulled.
    pub fn plp(&mut self) {
        self.pull_stack(&RegisterAlias::P);
    }
}
