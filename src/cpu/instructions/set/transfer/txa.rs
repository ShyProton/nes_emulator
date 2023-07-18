use super::{Cpu, RegisterAlias};

#[cfg(test)]
use super::test_modes;

impl Cpu {
    /// TXA - Transfer X to Accumulator.
    /// Copies the current contents of the X register into the accumulator, setting the zero and
    /// negative flags as appropriate.
    pub fn txa(&mut self) {
        self.transfer(&RegisterAlias::X, &RegisterAlias::A);
    }
}

#[cfg(test)]
mod tests {
    use super::{test_modes::implied, RegisterAlias};

    #[test]
    fn transfer() {
        implied::transfer(0x8A, &RegisterAlias::X, &RegisterAlias::A);
    }

    #[test]
    fn flag_check() {
        implied::flag_check(0x8A, &RegisterAlias::X);
    }
}
