use super::Registers;

#[derive(Debug)]
pub enum RegisterAlias {
    A, // Accumulator
    X, // X Register
    Y, // Y Register
    S, // Stack Pointer
    P, // Processor status
}

impl Registers {
    pub fn set_register(&mut self, alias: &RegisterAlias, value: u8) -> &mut Self {
        match alias {
            RegisterAlias::A => self.accumulator = value,
            RegisterAlias::X => self.index_x = value,
            RegisterAlias::Y => self.index_y = value,
            RegisterAlias::S => self.stack_pointer = value,
            RegisterAlias::P => self.status.set_byte(value),
        }

        self
    }

    pub fn get_register(&mut self, alias: &RegisterAlias) -> u8 {
        match alias {
            RegisterAlias::A => self.accumulator,
            RegisterAlias::X => self.index_x,
            RegisterAlias::Y => self.index_y,
            RegisterAlias::S => self.stack_pointer,
            RegisterAlias::P => self.status.get_byte(),
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
