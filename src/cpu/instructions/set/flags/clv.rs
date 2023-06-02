use super::{Cpu, StatusFlagAlias};

#[cfg(test)]
use super::test_templates::implied;

impl Cpu {
    /// CLV - Clear Overflow Flag.
    /// Clears the overflow flag.
    pub fn clv(&mut self) {
        self.registers.status.set_flag(StatusFlagAlias::V, false);
    }
}

#[cfg(test)]
mod tests {
    use super::{implied, StatusFlagAlias};

    #[test]
    fn check_flag() {
        implied::check_flag(0xB8, StatusFlagAlias::V, false);
    }
}
