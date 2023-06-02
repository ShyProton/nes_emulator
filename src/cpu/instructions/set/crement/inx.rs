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

    const REGISTER_ALIAS: RegisterAlias = RegisterAlias::X;
    const CREMENT_MODE: CrementMode = CrementMode::Increment;

    #[test]
    fn overflow() {
        immediate::wrapping(0xE8, &REGISTER_ALIAS, &CREMENT_MODE);
    }

    #[test]
    fn increment() {
        immediate::crement(0xE8, &REGISTER_ALIAS, &CREMENT_MODE);
    }
}
