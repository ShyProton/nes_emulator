use super::{Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// PLA - Pull Accumulator.
    /// Pulls an 8 bit value from the stack and into the accumulator. The zero and negative flags
    /// are set as appropriate.
    pub fn pla(&mut self) {
        self.pull_stack(&RegisterAlias::A);
        self.update_zero_and_negative_flags(self.registers.accumulator);
    }
}

#[cfg(test)]
mod tests {
    use super::{test_templates::implied, RegisterAlias};

    #[test]
    fn pull_stack() {
        implied::pull_stack(0x68, &RegisterAlias::A);
    }
}
