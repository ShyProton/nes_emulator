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
            ADC => self.adc(&instruction.addr_mode),
            SBC => self.sbc(&instruction.addr_mode),

            // Logical.
            AND => self.and(&instruction.addr_mode),
            EOR => self.eor(&instruction.addr_mode),
            ORA => self.ora(&instruction.addr_mode),

            // Bitshifting.
            ASL => self.asl(&instruction.addr_mode),
            LSR => self.lsr(&instruction.addr_mode),

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
            BIT => self.bit(&instruction.addr_mode),

            // Force break.
            BRK => self.brk(),

            // Flag clearing.
            CLC => self.clc(),
            CLD => self.cld(),
            CLI => self.cli(),
            CLV => self.clv(),

            // Comparisons.
            CMP => self.cmp(&instruction.addr_mode),
            CPX => self.cpx(&instruction.addr_mode),
            CPY => self.cpy(&instruction.addr_mode),

            // Decrementing.
            DEC => self.dec(&instruction.addr_mode),
            DEX => self.dex(),
            DEY => self.dey(),

            // Incrementing.
            INC => self.inc(&instruction.addr_mode),
            INX => self.inx(),
            INY => self.iny(),

            // Jumping.
            JMP => todo!(),
            JSR => todo!(),

            // Loading.
            LDA => self.lda(&instruction.addr_mode),
            LDX => self.ldx(&instruction.addr_mode),
            LDY => self.ldy(&instruction.addr_mode),

            // No operation.
            NOP => {}

            // Push stack.
            PHA => self.pha(),
            PHP => self.php(),

            // Pull stack.
            PLA => self.pla(),
            PLP => self.plp(),

            // Rotating.
            ROL => self.rol(&instruction.addr_mode),
            ROR => self.ror(&instruction.addr_mode),

            // Returning.
            RTI => todo!(),
            RTS => todo!(),

            // Flag setting.
            SEC => self.sec(),
            SED => self.sed(),
            SEI => self.sei(),

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
