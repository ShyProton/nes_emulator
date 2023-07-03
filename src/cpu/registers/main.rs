use super::{Memory, Status};

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
        self.stack_pointer = 0xFF;
        self.accumulator = 0x00;
        self.index_x = 0x00;
        self.index_y = 0x00;

        self.program_counter = memory.read_u16(Memory::PROGRAM_COUNTER_ADDRESS);
        self.status.reset_flags();
    }
}
