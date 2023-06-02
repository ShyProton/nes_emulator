use super::{Cpu, CrementMode, RegisterAlias};

#[cfg(test)]
use super::test_templates::immediate;

impl Cpu {
    /// INX - Increment the X Register.
    /// Adds one to the X register setting the zero and negative flags as appropriate.
    pub fn inx(&mut self) {
        self.crement_register(&RegisterAlias::X, &CrementMode::Increment);
    }
}

#[cfg(test)]
mod tests {
    use super::{immediate, CrementMode, RegisterAlias};

    #[test]
    fn overflow() {
        immediate::wrapping(0xE8, &RegisterAlias::X, &CrementMode::Increment);
    }

    #[test]
    fn increment() {
        immediate::crement(0xE8, &RegisterAlias::X, &CrementMode::Increment);
    }
}
