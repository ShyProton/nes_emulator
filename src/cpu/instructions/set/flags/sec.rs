use super::{Cpu, StatusFlagAlias};

#[cfg(test)]
use super::test_templates::implied;

impl Cpu {
    /// SEC - Set Carry Flag.
    /// Set the carry flag to one.
    pub fn sec(&mut self) {
        self.registers.status.set_flag(StatusFlagAlias::C, true);
    }
}

#[cfg(test)]
mod tests {
    use super::{implied, StatusFlagAlias};

    #[test]
    fn check_flag() {
        implied::check_flag(0x38, StatusFlagAlias::C, true);
    }
}
