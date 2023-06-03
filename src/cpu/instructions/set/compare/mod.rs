use std::cmp::Ordering;

use super::{
    aliases::{RegisterAlias, StatusFlagAlias},
    AddressingMode, Cpu,
};

mod cmp;
mod cpx;
mod cpy;

#[cfg(test)]
mod test_templates;

impl Cpu {
    /// CP_ - Compare a register.
    /// This instruction compares the contents of a given register with another memory held value
    /// and sets the zero and carry flags as appropriate.
    pub fn compare(&mut self, target: &RegisterAlias, addr_mode: &AddressingMode) {
        let addr = self.get_operand_address(addr_mode);
        let register_value = self.registers.get_register(target);

        let comparison = register_value.cmp(&self.memory.read(addr));

        // println!("|| ORDERING: {comparison:?}, REGISTER: {target:?} ||");
        // println!(
        //     "|| REGISTER: {register_value:#04x}, MEMORY: {:#04x} ||",
        //     self.memory.read(addr)
        // );
        // println!("|| READ FROM: {addr:#06x} ||");

        self.update_zero_and_negative_flags(register_value);

        self.registers
            .status
            .set_flag(StatusFlagAlias::C, comparison != Ordering::Less)
            .set_flag(StatusFlagAlias::Z, comparison == Ordering::Equal);
    }
}
