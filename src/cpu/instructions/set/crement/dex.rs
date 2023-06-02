use super::{Cpu, CrementMode, RegisterAlias};

#[cfg(test)]
use super::test_templates::immediate;

impl Cpu {
    /// DEX - Decrement the X Register.
    /// Subtracts one from the X register setting the zero and negative flags as appropriate.
    pub fn dex(&mut self) {
        self.crement_register(&RegisterAlias::X, &CrementMode::Decrement);
    }
}

#[cfg(test)]
mod tests {
    use super::{immediate, CrementMode, RegisterAlias};

    const REGISTER_ALIAS: RegisterAlias = RegisterAlias::X;
    const CREMENT_MODE: CrementMode = CrementMode::Decrement;

    #[test]
    fn underflow() {
        immediate::wrapping(0xCA, &REGISTER_ALIAS, &CREMENT_MODE);
    }

    #[test]
    fn decrement() {
        immediate::crement(0xCA, &REGISTER_ALIAS, &CREMENT_MODE);
    }
}
