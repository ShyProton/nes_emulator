use super::{Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// PHP - Push Processor Status.
    /// Pushes a copy of the status flags on to the stack.
    pub fn php(&mut self) {
        self.push_stack(&RegisterAlias::P);
    }
}

#[cfg(test)]
mod tests {
    use super::{test_templates::implied, RegisterAlias};

    #[test]
    fn push_stack() {
        implied::push_stack(0x08, &RegisterAlias::P);
    }
}
