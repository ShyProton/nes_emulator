use super::{Cpu, CrementMode, RegisterAlias};

#[cfg(test)]
use super::test_templates::immediate;

impl Cpu {
    /// DEY - Decrement the Y Register.
    /// Subtracts one from the Y register setting the zero and negative flags as appropriate.
    pub fn dey(&mut self) {
        self.crement_register(&RegisterAlias::Y, &CrementMode::Decrement);
    }
}

#[cfg(test)]
mod tests {
    use super::{immediate, CrementMode, RegisterAlias};

    const REGISTER_ALIAS: RegisterAlias = RegisterAlias::Y;
    const CREMENT_MODE: CrementMode = CrementMode::Decrement;

    #[test]
    fn underflow() {
        immediate::wrapping(0x88, &REGISTER_ALIAS, &CREMENT_MODE);
    }

    #[test]
    fn decrement() {
        immediate::crement(0x88, &REGISTER_ALIAS, &CREMENT_MODE);
    }
}
