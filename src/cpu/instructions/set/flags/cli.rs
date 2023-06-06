use super::{Cpu, StatusFlagAlias};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// CLI - Clear Interrupt Disable.
    /// Clears the interrupt disable flag allowing normal interrupt requests to be serviced.
    pub fn cli(&mut self) {
        self.registers.status.set_flag(StatusFlagAlias::I, false);
    }
}

#[cfg(test)]
mod tests {
    use super::{test_templates::implied, StatusFlagAlias};

    #[test]
    fn check_flag() {
        implied::check_flag(0x58, StatusFlagAlias::I, false);
    }
}
