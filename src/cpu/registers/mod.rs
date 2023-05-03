use status::Status;

mod status;

trait Register {
    fn bit_is_set(&self, position: usize) -> bool;
}

impl Register for u8 {
    fn bit_is_set(&self, position: usize) -> bool {
        assert!(position <= 7, "Bit position is out of bounds.");
        self & (0b0000_0001 << position) != 0
    }
}

pub struct Registers {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub status: Status,
}

impl Registers {
    pub const fn new() -> Self {
        Self {
            program_counter: 0x0000,
            stack_pointer: 0x00,
            accumulator: 0x00,
            index_x: 0x00,
            index_y: 0x00,
            status: Status::new(),
        }
    }

    pub fn reset_registers(&mut self) {
        self.program_counter = 0x0000;
        self.stack_pointer = 0x00;
        self.accumulator = 0x00;
        self.index_x = 0x00;
        self.index_y = 0x00;
        self.status.reset_flags();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_is_set() {
        let mut registers = Registers::new();

        for pos in 0..7 {
            // Tests to ensure the bit is set.
            registers.accumulator |= 0b0000_0001 << pos;
            assert!(registers.accumulator.bit_is_set(pos));

            // Tests to ensure the bit is unset.
            registers.accumulator &= !(0b0000_0001 << pos);
            assert!(!registers.accumulator.bit_is_set(pos));
        }
    }

    #[test]
    #[should_panic]
    fn bit_is_set_oob() {
        let registers = Registers::new();
        registers.accumulator.bit_is_set(8);
    }
}
