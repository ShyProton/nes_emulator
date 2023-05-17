use super::{Cpu, Instruction, InstructionAlias};

use InstructionAlias::{
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS, CLC, CLD, CLI, CLV, CMP, CPX,
    CPY, DEC, DEX, DEY, EOR, INC, INX, INY, JMP, JSR, LDA, LDX, LDY, LSR, NOP, ORA, PHA, PHP, PLA,
    PLP, ROL, ROR, RTI, RTS, SBC, SEC, SED, SEI, STA, STX, STY, TAX, TAY, TSX, TXA, TXS, TYA,
};

impl Cpu {
    pub fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction.alias {
            // Arithmetic.
            ADC => todo!(),
            SBC => todo!(),

            // Logical.
            AND => todo!(),
            EOR => todo!(),
            ORA => todo!(),

            // Bitshifting.
            ASL => todo!(),
            LSR => todo!(),

            // Branching.
            BCC => todo!(),
            BCS => todo!(),
            BEQ => todo!(),
            BMI => todo!(),
            BNE => todo!(),
            BPL => todo!(),
            BVC => todo!(),
            BVS => todo!(),

            // Bit test.
            BIT => todo!(),

            // Force break.
            // TODO: Set B flag to 1.
            BRK => self.brk(),

            // Flag clearing.
            CLC => todo!(),
            CLD => todo!(),
            CLI => todo!(),
            CLV => todo!(),

            // Comparisons.
            CMP => todo!(),
            CPX => todo!(),
            CPY => todo!(),

            // Decrementing.
            DEC => todo!(),
            DEX => todo!(),
            DEY => todo!(),

            // Incrementing.
            INC => todo!(),
            INX => self.inx(),
            INY => todo!(),

            // Jumping.
            JMP => todo!(),
            JSR => todo!(),

            // Loading.
            LDA => self.lda(&instruction.addr_mode),
            LDX => self.ldx(&instruction.addr_mode),
            LDY => self.ldy(&instruction.addr_mode),

            // No operation.
            NOP => todo!(),

            // Push stack.
            PHA => todo!(),
            PHP => todo!(),

            // Pull stack.
            PLA => todo!(),
            PLP => todo!(),

            // Rotating.
            ROL => todo!(),
            ROR => todo!(),

            // Returning.
            RTI => todo!(),
            RTS => todo!(),

            // Flag setting.
            SEC => todo!(),
            SED => todo!(),
            SEI => todo!(),

            // Storing.
            STA => self.sta(&instruction.addr_mode),
            STX => self.stx(&instruction.addr_mode),
            STY => self.sty(&instruction.addr_mode),

            // Transferring.
            TAX => self.tax(),
            TAY => self.tay(),
            TSX => self.tsx(),
            TXA => self.txa(),
            TXS => self.txs(),
            TYA => self.tya(),
        }
    }
}
