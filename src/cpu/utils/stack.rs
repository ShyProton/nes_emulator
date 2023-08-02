use super::Cpu;

impl Cpu {
    pub fn get_stack_addr(&self) -> u16 {
        0x0100 + u16::from(self.registers.stack_pointer)
    }
}
