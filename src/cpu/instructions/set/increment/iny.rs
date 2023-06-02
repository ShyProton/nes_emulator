use super::{Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates::immediate;

impl Cpu {
    /// INY - Increment the Y Register.
    /// Adds one to the Y register setting the zero and negative flags as appropriate.
    pub fn iny(&mut self) {
        self.increment(&RegisterAlias::Y);
    }
}

#[cfg(test)]
mod tests {
    use super::{immediate, RegisterAlias};

    #[test]
    fn increment() {
        immediate::increment(0xC8, &RegisterAlias::Y);
    }

    #[test]
    fn overflow_check() {
        immediate::overflow(0xC8, &RegisterAlias::Y);
    }
}
