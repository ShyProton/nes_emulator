use super::{AddressingMode, Cpu};

mod asl;
mod lsr;
mod rol;
mod ror;

#[cfg(test)]
mod test_templates;

#[allow(clippy::module_name_repetitions)]
pub enum ShiftType {
    Shift,
    Rotate,
}

impl Cpu {
    fn shift(&mut self, addr_mode: &AddressingMode, shift_type: &ShiftType) {
        todo!();
    }
}
