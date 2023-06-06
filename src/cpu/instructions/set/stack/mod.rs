use super::{aliases::RegisterAlias, Cpu};

#[cfg(test)]
use super::aliases::StatusFlagAlias;

mod pha;
mod php;
mod pla;
mod plp;

#[cfg(test)]
mod test_templates;

impl Cpu {
    fn get_stack_addr(&self) -> u16 {
        0x0100 + u16::from(self.registers.stack_pointer)
    }

    fn push_stack(&mut self, target: &RegisterAlias) {
        let register_val = self.registers.get_register(target);

        self.memory.write(self.get_stack_addr(), register_val);

        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(0x01);
    }

    fn pull_stack(&mut self, target: &RegisterAlias) {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(0x01);

        let stack_val = self.memory.read(self.get_stack_addr());

        self.registers.set_register(target, stack_val);
    }
}
