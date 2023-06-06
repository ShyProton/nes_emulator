use super::{Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// PHA - Push Accumulator.
    /// Pushes a copy of the accumulator on to the stack.
    pub fn pha(&mut self) {
        self.push_stack(&RegisterAlias::A);
    }
}

#[cfg(test)]
mod tests {
    use super::{test_templates::implied, RegisterAlias};

    #[test]
    fn push_stack() {
        implied::push_stack(0x48, &RegisterAlias::A);
    }
}
