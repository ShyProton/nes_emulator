use super::{Cpu, CrementMode, RegisterAlias};

#[cfg(test)]
use super::test_templates::immediate;

impl Cpu {
    /// INY - Increment the Y Register.
    /// Adds one to the Y register setting the zero and negative flags as appropriate.
    pub fn iny(&mut self) {
        self.crement_register(&RegisterAlias::Y, &CrementMode::Increment);
    }
}

#[cfg(test)]
mod tests {
    use super::{immediate, CrementMode, RegisterAlias};

    #[test]
    fn overflow() {
        immediate::wrapping(0xC8, &RegisterAlias::Y, &CrementMode::Increment);
    }

    #[test]
    fn increment() {
        immediate::crement(0xC8, &RegisterAlias::Y, &CrementMode::Increment);
    }
}
