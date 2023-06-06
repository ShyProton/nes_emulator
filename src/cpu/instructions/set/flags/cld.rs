use super::{Cpu, StatusFlagAlias};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// CLD - Clear Decimal Mode.
    /// Set the decimal mode flag to zero.
    pub fn cld(&mut self) {
        self.registers.status.set_flag(StatusFlagAlias::D, false);
    }
}

#[cfg(test)]
mod tests {
    use super::{test_templates::implied, StatusFlagAlias};

    #[test]
    fn check_flag() {
        implied::check_flag(0xD8, StatusFlagAlias::D, false);
    }
}
