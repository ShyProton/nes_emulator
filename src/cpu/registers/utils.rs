pub trait RegisterByte {
    fn get_nth_bit(&self, position: usize) -> bool;
}

impl RegisterByte for u8 {
    fn get_nth_bit(&self, position: usize) -> bool {
        assert!(position <= 7, "bit position is out of bounds");
        self & (0b0000_0001 << position) != 0
    }
}

pub enum RegisterAlias {
    A, // Accumulator
    X, // X Register
    Y, // Y Register
    S, // Stack Pointer
}

pub enum StatusAlias {
    C, // Carry Flag
    Z, // Zero Flag
    I, // Interrupt Disable
    D, // Decimal Mode Flag
    B, // Break Command
    V, // Overflow Flag
    N, // Negative Flag
}
