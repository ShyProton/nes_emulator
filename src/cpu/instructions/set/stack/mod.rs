use super::Cpu;

#[cfg(test)]
use super::{
    aliases::{RegisterAlias, StatusFlagAlias},
    test_prep, AddressingMode,
};

#[cfg(test)]
mod tests;

impl Cpu {
    /// PHA - Push Accumulator.
    /// Pushes a copy of the accumulator on to the stack.
    pub fn pha(&mut self) {
        self.push_stack(self.registers.accumulator);
    }

    /// PHP - Push Processor Status.
    /// Pushes a copy of the status flags on to the stack.
    pub fn php(&mut self) {
        self.push_stack(self.registers.status.get_byte());
    }

    /// PLA - Pull Accumulator.
    /// Pulls an 8 bit value from the stack and into the accumulator. The zero and negative flags
    /// are set as appropriate.
    pub fn pla(&mut self) {
        self.registers.accumulator = self.pull_stack();
        self.update_zero_and_negative_flags(self.registers.accumulator);
    }

    /// PLP - Pull Processor Status.
    /// Pulls an 8 bit value from the stack and into the processor flags. The flags will take on
    /// new states as determined by the value pulled.
    pub fn plp(&mut self) {
        let pulled_status = self.pull_stack();
        self.registers.status.set_byte(pulled_status);
    }
}
