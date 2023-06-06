use super::{AddressingMode, Cpu, Operation};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    /// AND - Logical AND.
    /// A logical AND is performed, bit by bit, on the accumulator contents using the contents of
    /// a byte of memory.
    pub fn and(&mut self, addr_mode: &AddressingMode) {
        self.logical(addr_mode, &Operation::And);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        test_templates::{absolute, immediate, indirect, zero},
        Operation,
    };

    const OPERATION: Operation = Operation::And;

    #[test]
    fn imm_and() {
        immediate::logical(0x29, &OPERATION);
    }

    #[test]
    fn zero_and() {
        zero::logical(0x25, &OPERATION);
    }

    #[test]
    fn zero_x_and() {
        zero::x_logical(0x35, &OPERATION);
    }

    #[test]
    fn abs_and() {
        absolute::logical(0x2D, &OPERATION);
    }

    #[test]
    fn abs_x_and() {
        absolute::x_logical(0x3D, &OPERATION);
    }

    #[test]
    fn abs_y_and() {
        absolute::y_logical(0x39, &OPERATION);
    }

    #[test]
    fn ind_x_and() {
        indirect::x_logical(0x21, &OPERATION);
    }

    #[test]
    fn ind_y_and() {
        indirect::y_logical(0x31, &OPERATION);
    }
}
