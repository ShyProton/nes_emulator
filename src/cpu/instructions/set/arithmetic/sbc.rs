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
        todo!();
    }

    #[test]
    fn zero_x_sub() {
        todo!();
    }

    #[test]
    fn abs_sub() {
        todo!();
    }

    #[test]
    fn abs_x_sub() {
        todo!();
    }

    #[test]
    fn abs_y_sub() {
        todo!();
    }

    #[test]
    fn ind_x_sub() {
        todo!();
    }

    #[test]
    fn ind_y_sub() {
        todo!();
    }
}
