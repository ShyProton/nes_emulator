use super::{Cpu, RegisterAlias};

#[cfg(test)]
use super::test_modes;

impl Cpu {
    /// TYA - Transfer Y to Accumulator.
    /// Copies the current contents of the Y register into the accumulator, setting the zero and
    /// negative flags as appropriate.
    pub fn tya(&mut self) {
        self.transfer(&RegisterAlias::Y, &RegisterAlias::A);
    }
}

#[cfg(test)]
mod tests {
    use super::{test_modes::implied, RegisterAlias};

    #[test]
    fn transfer() {
        implied::transfer(0x98, &RegisterAlias::Y, &RegisterAlias::A);
    }

    #[test]
    fn flag_check() {
        implied::flag_check(0x98, &RegisterAlias::Y);
    }
}
