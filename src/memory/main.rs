// TODO: Generalize for CPU and PPU
// TODO: Make tests for memory.
pub struct Memory {
    memory: [u8; 0xFFFF],
}

impl Memory {
    pub const PROGRAM_COUNTER_ADDRESS: u16 = 0xFFFC;
    const PRG_ROM_START: u16 = 0x8000;

    pub const fn new() -> Self {
        Self {
            memory: [0x00; 0xFFFF],
        }
    }

    pub const fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    /// Reads a two-byte sequence of data from a memory address.
    pub fn read_u16(&self, address: u16) -> u16 {
        // Reading as little-endian.
        let lo = u16::from(self.read(address));
        let hi = u16::from(self.read(address + 1));
        (hi << 8) | lo
    }

    /// Writes a two-byte sequence of data to a memory address.
    pub fn write_u16(&mut self, address: u16, data: u16) {
        // Writing as little-endian.
        let hi = (data >> 8) as u8;
        let lo = (data & 0xFF) as u8;

        self.write(address, lo);
        self.write(address + 1, hi);
    }

    pub fn load(&mut self, program: &[u8]) {
        let mem_idx = Self::PRG_ROM_START as usize;

        self.memory = [0; 0xFFFF];
        self.memory[mem_idx..(mem_idx + program.len())].copy_from_slice(program);

        self.write_u16(Self::PROGRAM_COUNTER_ADDRESS, Self::PRG_ROM_START);
    }
}
