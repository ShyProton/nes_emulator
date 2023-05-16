use super::{AddressingMode, Cpu, InstructionAlias, INSTRUCTION_LOOKUP};

use InstructionAlias::{
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS, CLC, CLD, CLI, CLV, CMP, CPX,
    CPY, DEC, DEX, DEY, EOR, INC, INX, INY, JMP, JSR, LDA, LDX, LDY, LSR, NOP, ORA, PHA, PHP, PLA,
    PLP, ROL, ROR, RTI, RTS, SBC, SEC, SED, SEI, STA, STX, STY, TAX, TAY, TSX, TXA, TXS, TYA,
};

pub struct Instruction {
    alias: InstructionAlias,
    bytes: u8,
    cycles: u8,
    addr_mode: AddressingMode,
}

impl Instruction {
    pub const fn new(
        alias: InstructionAlias,
        bytes: u8,
        cycles: u8,
        addr_mode: AddressingMode,
    ) -> Self {
        Self {
            alias,
            bytes,
            cycles,
            addr_mode,
        }
    }
}

impl Cpu {
    /// Executes the next CPU instruction as defined in the memory.
    /// Only returns false if the instruction is to halt the program.
    pub fn execute_instruction(&mut self) -> bool {
        let opcode = self.memory.read(self.registers.program_counter);

        let instruction = INSTRUCTION_LOOKUP.get(&[opcode]).map_or_else(
            || panic!("invalid opcode lookup for instruction"),
            |instruction| instruction,
        );

        self.registers.program_counter += 1;

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
            BRK => return false,

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
            TSX => todo!(),
            TXA => todo!(),
            TXS => todo!(),
            TYA => todo!(),
        }

        true
    }
}
