use super::{Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// PLP - Pul Processor Status.
    /// Pulls an 8 bit value from the stack and into the processor flags. The flags will take on
    /// new states as determined by the value pulled.
    pub fn plp(&mut self) {
        self.pull_stack(&RegisterAlias::P);
    }
}

#[cfg(test)]
mod tests {
    use super::{test_templates::implied, RegisterAlias};

    #[test]
    fn pull_stack() {
        implied::pull_stack(0x28, &RegisterAlias::P);
    }
}
