use super::{Cpu, StatusFlagAlias};

#[cfg(test)]
use super::test_templates::implied;

impl Cpu {
    /// SED - Set Decimal Flag.
    /// Set the decimal mode flag to one.
    pub fn sed(&mut self) {
        self.registers.status.set_flag(StatusFlagAlias::D, true);
    }
}

#[cfg(test)]
mod tests {
    use super::{implied, StatusFlagAlias};

    #[test]
    fn check_flag() {
        implied::check_flag(0xF8, StatusFlagAlias::D, true);
    }
}
