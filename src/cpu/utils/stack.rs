use super::Cpu;

impl Cpu {
    fn get_stack_addr(&self) -> u16 {
        0x0100 + u16::from(self.registers.stack_pointer)
    }

    pub fn pull_stack(&mut self) -> u8 {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(0x01);
        self.memory.read(self.get_stack_addr())
    }

    pub fn push_stack(&mut self, data: u8) {
        self.memory.write(self.get_stack_addr(), data);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(0x01);
    }

    pub fn pull_stack_u16(&mut self) -> u16 {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(0x01);
        let pulled_value = self.memory.read_u16(self.get_stack_addr());
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(0x01);

        pulled_value
    }

    pub fn push_stack_u16(&mut self, data: u16) {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(0x01);
        self.memory.write_u16(self.get_stack_addr(), data);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(0x01);
    }
}
