use super::{Cpu, RegisterAlias};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// TSX - Transfer Stack Pointer to X.
    /// Copies the current contents of the stack register into the X register, setting the zero and
    /// negative flags as appropriate.
    pub fn tsx(&mut self) {
        self.transfer(&RegisterAlias::S, &RegisterAlias::X);
    }
}

#[cfg(test)]
mod tests {
    use super::{test_templates::implied, RegisterAlias};

    #[test]
    fn transfer() {
        implied::transfer(0xBA, &RegisterAlias::S, &RegisterAlias::X);
    }

    #[test]
    fn flag_check() {
        implied::flag_check(0xBA, &RegisterAlias::S);
    }
}
