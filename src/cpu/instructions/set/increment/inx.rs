use super::{Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates::immediate;

impl Cpu {
    /// INX - Increment the X Register.
    /// Adds one to the X register setting the zero and negative flags as appropriate.
    pub fn inx(&mut self) {
        self.increment(&RegisterAlias::X);
    }
}

#[cfg(test)]
mod tests {
    use super::{immediate, RegisterAlias};

    #[test]
    fn increment() {
        immediate::increment(0xE8, &RegisterAlias::X);
    }

    #[test]
    fn overflow_check() {
        immediate::overflow(0xE8, &RegisterAlias::X);
    }
}
