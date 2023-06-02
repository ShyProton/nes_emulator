use super::Registers;

pub enum RegisterAlias {
    A, // Accumulator
    X, // X Register
    Y, // Y Register
    S, // Stack Pointer
}

impl Registers {
    pub fn set_register(&mut self, alias: &RegisterAlias, value: u8) {
        match alias {
            RegisterAlias::A => self.accumulator = value,
            RegisterAlias::X => self.index_x = value,
            RegisterAlias::Y => self.index_y = value,
            RegisterAlias::S => self.stack_pointer = value,
        }
    }

    pub fn get_register(&mut self, alias: &RegisterAlias) -> u8 {
        match alias {
            RegisterAlias::A => self.accumulator,
            RegisterAlias::X => self.index_x,
            RegisterAlias::Y => self.index_y,
            RegisterAlias::S => self.stack_pointer,
        }
    }

    // TODO: Use the above getters and setters instead of this by_alias method.
    pub fn by_alias(&mut self, alias: &RegisterAlias) -> &mut u8 {
        match alias {
            RegisterAlias::S => &mut self.stack_pointer,
            RegisterAlias::A => &mut self.accumulator,
            RegisterAlias::X => &mut self.index_x,
            RegisterAlias::Y => &mut self.index_y,
        }
    }
}

#[derive(Copy, Clone)]
pub enum StatusFlagAlias {
    C = 0, // Carry Flag
    Z = 1, // Zero Flag
    I = 2, // Interrupt Disable
    D = 3, // Decimal Mode Flag
    B = 4, // Break Command
    V = 6, // Overflow Flag
    N = 7, // Negative Flag
}

impl StatusFlagAlias {
    pub const fn index(self) -> usize {
        self as usize
    }
}
