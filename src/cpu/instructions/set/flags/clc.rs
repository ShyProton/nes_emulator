use super::{Cpu, StatusFlagAlias};

#[cfg(test)]
use super::test_templates::implied;

impl Cpu {
    /// CLC - Clear Carry Flag.
    /// Set the carry flag to zero.
    pub fn clc(&mut self) {
        self.registers.status.set_flag(StatusFlagAlias::C, false);
    }
}

#[cfg(test)]
mod tests {
    use super::{implied, StatusFlagAlias};

    #[test]
    fn check_flag() {
        implied::check_flag(0x18, StatusFlagAlias::C, false);
    }
}
