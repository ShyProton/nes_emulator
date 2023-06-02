use super::{Cpu, StatusFlagAlias};

#[cfg(test)]
use super::test_templates::implied;

impl Cpu {
    /// SEI - Set Interrupt Disable.
    /// Set the interrupt disable flag to one.
    pub fn sei(&mut self) {
        self.registers.status.set_flag(StatusFlagAlias::I, true);
    }
}

#[cfg(test)]
mod tests {
    use super::{implied, StatusFlagAlias};

    #[test]
    fn check_flag() {
        implied::check_flag(0x78, StatusFlagAlias::I, true);
    }
}
