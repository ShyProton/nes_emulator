use std::ops::Index;

pub struct Registers {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub status: u8,
}

impl Registers {
    pub const fn new() -> Self {
        Self {
            program_counter: 0x00,
            stack_pointer: 0x00,
            accumulator: 0x00,
            index_x: 0x00,
            index_y: 0x00,
            status: 0x00,
        }
    }

    pub fn reset_registers(&mut self) {
        self.program_counter = 0x00;
        self.stack_pointer = 0x00;
        self.accumulator = 0x00;
        self.index_x = 0x00;
        self.index_y = 0x00;
        self.status = 0x00;
    }
}

const FLAGS: [char; 8] = ['N', 'V', ' ', 'B', 'D', 'I', 'Z', 'C'];

struct Status {
    status: u8,
}

impl Status {
    pub const fn new() -> Self {
        Self { status: 0x00 }
    }

    // TODO: Implement easy setting/getting of the flags.
    pub fn set(flag: char, value: bool) {
        let mask = 0b0000_0001;
        let shift = FLAGS.iter().position(|&f| f == flag).unwrap();
    }

    pub fn get(flag: char) -> bool {
        todo!()
    }
}
