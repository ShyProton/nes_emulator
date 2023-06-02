use super::{
    aliases::{RegisterAlias, StatusFlagAlias},
    AddressingMode, Cpu,
};

mod cmp;
mod cpx;
mod cpy;

impl Cpu {
    /// CP_ - Compare a register.
    /// This instruction compares the contents of a given register with another memory held value
    /// and sets the zero and carry flags as appropriate.
    pub fn compare(&mut self, target: &RegisterAlias, addr_mode: &AddressingMode) {
        let addr = self.get_operand_address(addr_mode);
        let register_value = self.registers.get_register(target);

        let comparison = self.memory.read(addr).cmp(&register_value);

        // TODO: Finish after refactoring register getting/setting.
        // self.registers.status.set_flag(StatusFlagAlias::C, true);
    }
}
