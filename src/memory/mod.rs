// TODO: Generalize for CPU and PPU
pub struct Memory {
    memory: [u8; 0xFFFF],
}

impl Memory {
    pub const PRG_ROM_START: u16 = 0x8000;

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

    pub fn load(&mut self, program: &[u8]) {
        let start_idx = Self::PRG_ROM_START as usize;
        self.memory[start_idx..(start_idx + program.len())].copy_from_slice(program);
    }
}
