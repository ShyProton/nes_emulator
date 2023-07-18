use super::{Cpu, RegisterAlias};

#[cfg(test)]
use super::test_modes;

impl Cpu {
    /// TXS - Transfer X to Stack Pointer.
    /// Copies the current contents of the X register into the stack register, setting the zero and
    /// negative flags as appropriate.
    pub fn txs(&mut self) {
        self.transfer(&RegisterAlias::X, &RegisterAlias::S);
    }
}

#[cfg(test)]
mod tests {
    use super::{test_modes::implied, RegisterAlias};

    #[test]
    fn transfer() {
        implied::transfer(0x9A, &RegisterAlias::X, &RegisterAlias::S);
    }

    #[test]
    fn flag_check() {
        implied::flag_check(0x9A, &RegisterAlias::X);
    }
}
