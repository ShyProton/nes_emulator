use super::{Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates::implied;

impl Cpu {
    /// TAY - Transfer Accumulator to Y.
    /// Copies the current contents of the accumulator into the Y register, setting the zero and
    /// negative flags as appropriate.
    pub fn tay(&mut self) {
        self.transfer(&RegisterAlias::A, &RegisterAlias::Y);
    }
}

#[cfg(test)]
mod tests {
    use super::{implied, RegisterAlias};

    #[test]
    fn transfer() {
        implied::transfer(0xA8, &RegisterAlias::A, &RegisterAlias::Y);
    }

    #[test]
    fn flag_check() {
        implied::flag_check(0xA8, &RegisterAlias::A);
    }
}
