use super::{AddressingMode, ArithmeticMode, Cpu};

#[cfg(test)]
use super::test_templates;

impl Cpu {
    pub fn sbc(&mut self, subr_mode: &AddressingMode) {
        self.arithmetic(subr_mode, &ArithmeticMode::Subtraction);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        test_templates::{absolute, immediate, indirect, zero},
        ArithmeticMode,
    };

    const ARITHMETIC_MODE: ArithmeticMode = ArithmeticMode::Subtraction;

    #[test]
    fn imm_sub() {
        immediate::arithmetic(0xE9, &ARITHMETIC_MODE);
    }

    #[test]
    fn zero_sub() {
        zero::arithmetic(0xE5, &ARITHMETIC_MODE);
    }

    #[test]
    fn zero_x_sub() {
        zero::x_arithmetic(0xF5, &ARITHMETIC_MODE);
    }

    #[test]
    fn abs_sub() {
        absolute::arithmetic(0xED, &ARITHMETIC_MODE);
    }

    #[test]
    fn abs_x_sub() {
        absolute::x_arithmetic(0xFD, &ARITHMETIC_MODE);
    }

    #[test]
    fn abs_y_sub() {
        absolute::y_arithmetic(0xF9, &ARITHMETIC_MODE);
    }

    #[test]
    fn ind_x_sub() {
        indirect::x_arithmetic(0xE1, &ARITHMETIC_MODE);
    }

    #[test]
    fn ind_y_sub() {
        indirect::y_arithmetic(0xF1, &ARITHMETIC_MODE);
    }
}
