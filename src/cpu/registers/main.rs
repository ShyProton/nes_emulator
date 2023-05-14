use super::{Memory, Status};

pub trait Register {
    fn get_nth_bit(&self, position: usize) -> bool;
}

impl Register for u8 {
    fn get_nth_bit(&self, position: usize) -> bool {
        assert!(position <= 7, "bit position is out of bounds");
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

    pub fn reset(&mut self, memory: &Memory) {
        self.stack_pointer = 0x00;
        self.accumulator = 0x00;
        self.index_x = 0x00;
        self.index_y = 0x00;

        self.program_counter = memory.read_u16(Memory::PROGRAM_COUNTER_ADDRESS);
        self.status.reset_flags();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_nth_bit() {
        let mut registers = Registers::new();

        for pos in 0..7 {
            // Tests to ensure the bit is set.
            registers.accumulator |= 0b0000_0001 << pos;
            assert!(registers.accumulator.get_nth_bit(pos));

            // Tests to ensure the bit is unset.
            registers.accumulator &= !(0b0000_0001 << pos);
            assert!(!registers.accumulator.get_nth_bit(pos));
        }
    }

    #[test]
    #[should_panic(expected = "bit position is out of bounds")]
    fn get_nth_bit_oob() {
        let registers = Registers::new();
        registers.accumulator.get_nth_bit(8);
    }
}
