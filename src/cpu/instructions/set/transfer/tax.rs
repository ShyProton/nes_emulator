use super::{Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// TAX - Transfer Accumulator to X.
    /// Copies the current contents of the accumulator into the X register, setting the zero and
    /// negative flags as appropriate.
    pub fn tax(&mut self) {
        self.transfer(&RegisterAlias::A, &RegisterAlias::X);
    }
}

#[cfg(test)]
mod tests {
    use super::{test_templates::implied, RegisterAlias};

    #[test]
    fn transfer() {
        implied::transfer(0xAA, &RegisterAlias::A, &RegisterAlias::X);
    }

    #[test]
    fn flag_check() {
        implied::flag_check(0xAA, &RegisterAlias::A);
    }
}
