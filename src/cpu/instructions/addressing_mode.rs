use super::Cpu;

use self::AddressingMode::{
    Absolute, AbsoluteX, AbsoluteY, Immediate, Implied, IndirectX, IndirectY, ZeroPage, ZeroPageX,
    ZeroPageY,
};

#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    Implied,
}

impl Cpu {
    pub fn get_operand_address(&mut self, addr_mode: &AddressingMode) -> u16 {
        let addr = match addr_mode {
            Immediate => self.registers.program_counter,

            ZeroPage => self.zero_page(0),
            ZeroPageX => self.zero_page(self.registers.index_x),
            ZeroPageY => self.zero_page(self.registers.index_y),

            Absolute => self.absolute(0),
            AbsoluteX => self.absolute(self.registers.index_x),
            AbsoluteY => self.absolute(self.registers.index_y),

            IndirectX | IndirectY => self.indirect(addr_mode),

            Implied => panic!("mode is implied, address does not need to be looked up"),
        };

        // Always reads at least one byte.
        self.registers.program_counter += 1;

        addr
    }

    fn zero_page(&self, register: u8) -> u16 {
        let pos = self.memory.read(self.registers.program_counter);
        u16::from(pos.wrapping_add(register))
    }

    fn absolute(&mut self, register: u8) -> u16 {
        let base = self.memory.read_u16(self.registers.program_counter);
        self.registers.program_counter += 1; // Reads an extra byte.

        base.wrapping_add(u16::from(register))
    }

    fn indirect(&mut self, addr_mode: &AddressingMode) -> u16 {
        let mut base = self.memory.read(self.registers.program_counter);

        if matches!(addr_mode, AddressingMode::IndirectX) {
            base = base.wrapping_add(self.registers.index_x);
        }

        let lo = self.memory.read(u16::from(base));
        let hi = self.memory.read(u16::from(base).wrapping_add(1));

        let mut addr = u16::from(hi) << 8 | u16::from(lo);

        if matches!(addr_mode, AddressingMode::IndirectY) {
            addr = addr.wrapping_add(self.registers.index_y.into());
        }

        addr
    }
}
