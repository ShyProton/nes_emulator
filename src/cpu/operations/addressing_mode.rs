use super::Cpu;

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
            AddressingMode::Immediate => self.registers.program_counter,

            AddressingMode::ZeroPage => self.zero_page(0),
            AddressingMode::ZeroPageX => self.zero_page(self.registers.index_x),
            AddressingMode::ZeroPageY => self.zero_page(self.registers.index_y),

            AddressingMode::Absolute => self.absolute(0),
            AddressingMode::AbsoluteX => self.absolute(self.registers.index_x),
            AddressingMode::AbsoluteY => self.absolute(self.registers.index_y),

            // TODO: Extract Indirect implementations to a single method.
            // TODO: Test Indirect implementations with the JMP command.
            AddressingMode::IndirectX => {
                let base = self.memory.read(self.registers.program_counter);

                let ptr = base.wrapping_add(self.registers.index_x);
                let lo = self.memory.read(ptr.into());
                let hi = self.memory.read(ptr.wrapping_add(1).into());

                u16::from(hi) << 8 | u16::from(lo)
            }

            AddressingMode::IndirectY => {
                let base = self.memory.read(self.registers.program_counter);

                let lo = self.memory.read(base.into());
                let hi = self.memory.read(u16::from(base).wrapping_add(1));

                let deref_base = u16::from(hi) << 8 | u16::from(lo);
                deref_base.wrapping_add(self.registers.index_y.into())
            }

            AddressingMode::Implied => {
                panic!("mode is implied, address does not need to be looked up");
            }
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
}
