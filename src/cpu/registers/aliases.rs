use super::Registers;

pub enum RegisterAlias {
    A, // Accumulator
    X, // X Register
    Y, // Y Register
    S, // Stack Pointer
}

impl Registers {
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
