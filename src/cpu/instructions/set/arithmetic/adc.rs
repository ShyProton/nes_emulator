use super::{AddressingMode, ArithmeticMode, Cpu};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    pub fn adc(&mut self, addr_mode: &AddressingMode) {
        self.arithmetic(addr_mode, &ArithmeticMode::Addition);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        test_templates::{absolute, immediate, indirect, zero},
        ArithmeticMode,
    };

    const ARITHMETIC_MODE: ArithmeticMode = ArithmeticMode::Addition;

    #[test]
    fn imm_add() {
        immediate::arithmetic(0x69, &ARITHMETIC_MODE);
    }

    #[test]
    fn zero_add() {
        todo!();
    }

    #[test]
    fn zero_x_add() {
        todo!();
    }

    #[test]
    fn abs_add() {
        todo!();
    }

    #[test]
    fn abs_x_add() {
        todo!();
    }

    #[test]
    fn abs_y_add() {
        todo!();
    }

    #[test]
    fn ind_x_add() {
        todo!();
    }

    #[test]
    fn ind_y_add() {
        todo!();
    }
}
