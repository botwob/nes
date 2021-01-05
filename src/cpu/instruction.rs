// Opcode table: http://www.oxyron.de/html/opcodes02.html
const OPCODE_TABLE: [(Opcode, AddressingMode, CycleCount, CycleCount);256] =
    // (Operation, addressing mode, clock cycles, extra clock cycles if page boundary crossed)
    [   // 0x
        (BRK, imp, 7, 0), // x0
        (ORA, izx, 6, 0), // x1
        (KIL, imp, 0, 0), // x2
        (SLO, izx, 8, 0), // x3
        (NOP, zp,  3, 0), // x4
        (ORA, zp,  3, 0), // x5
        (ASL, zp,  5, 0), // x6
        (SLO, zp,  5, 0), // x7
        (PHP, imp, 3, 0), // x8
        (ORA, imm, 2, 0), // x9
        (ASL, acc, 2, 0), // xA
        (ANC, imm, 2, 0), // xB
        (NOP, abs, 4, 0), // xC
        (ORA, abs, 4, 0), // xD
        (ASL, abs, 6, 0), // xE
        (SLO, abs, 6, 0), // xF
        // 1x
        (BPL, rel, 2, 1), // x0
        (ORA, izy, 5, 1), // x1
        (KIL, imp, 0, 0), // x2
        (SLO, izy, 8, 0), // x3
        (NOP, zpx, 4, 0), // x4
        (ORA, zpx, 4, 0), // x5
        (ASL, zpx, 6, 0), // x6
        (SLO, zpx, 6, 0), // x7
        (CLC, imp, 2, 0), // x8
        (ORA, aby, 4, 1), // x9
        (NOP, imp, 2, 0), // xA
        (SLO, aby, 7, 0), // xB
        (NOP, abx, 4, 1), // xC
        (ORA, abx, 4, 1), // xD
        (ASL, abx, 7, 0), // xE
        (SLO, abx, 7, 0), // xF
        // 2x
        (JSR, abs, 6, 0), // x0
        (AND, izx, 6, 0), // x1
        (KIL, imp, 0, 0), // x2
        (RLA, izx, 8, 0), // x3
        (BIT, zp,  3, 0), // x4
        (AND, zp,  3, 0), // x5
        (ROL, zp,  5, 0), // x6
        (RLA, zp,  5, 0), // x7
        (PLP, imp, 4, 0), // x8
        (AND, imm, 2, 0), // x9
        (ROL, acc, 2, 0), // xA
        (ANC, imm, 2, 0), // xB
        (BIT, abs, 4, 0), // xC
        (AND, abs, 4, 0), // xD
        (ROL, abs, 6, 0), // xE
        (RLA, abs, 6, 0), // xF
        // 3x
        (BMI, rel, 2, 1), // x0
        (AND, izy, 5, 1), // x1
        (KIL, imp, 0, 0), // x2
        (RLA, izy, 8, 0), // x3
        (NOP, zpx, 4, 0), // x4
        (AND, zpx, 4, 0), // x5
        (ROL, zpx, 6, 0), // x6
        (RLA, zpx, 6, 0), // x7
        (SEC, imp, 2, 0), // x8
        (AND, aby, 4, 1), // x9
        (NOP, imp, 2, 0), // xA
        (RLA, aby, 7, 0), // xB
        (NOP, abx, 4, 1), // xC
        (AND, abx, 4, 1), // xD
        (ROL, abx, 7, 0), // xE
        (RLA, abx, 7, 0), // xF
        // 4x
        (RTI, imp, 6, 0), // x0
        (EOR, izx, 6, 0), // x1
        (KIL, imp, 0, 0), // x2
        (SRE, izx, 8, 0), // x3
        (NOP, zp,  3, 0), // x4
        (EOR, zp,  3, 0), // x5
        (LSR, zp,  5, 0), // x6
        (SRE, zp,  5, 0), // x7
        (PHA, imp, 3, 0), // x8
        (EOR, imm, 2, 0), // x9
        (LSR, imp, 2, 0), // xA
        (ALR, imm, 2, 0), // xB
        (JMP, abs, 3, 0), // xC
        (EOR, abs, 4, 0), // xD
        (LSR, abs, 6, 0), // xE
        (SRE, abs, 6, 0), // xF
        // 5x
        (BVC, rel, 2, 1), // x0
        (EOR, izy, 5, 1), // x1
        (KIL, imp, 0, 0), // x2
        (SRE, izy, 8, 0), // x3
        (NOP, zpx, 4, 0), // x4
        (EOR, zpx, 4, 0), // x5
        (LSR, zpx, 6, 0), // x6
        (SRE, zpx, 6, 0), // x7
        (CLI, imp, 2, 0), // x8
        (EOR, aby, 4, 1), // x9
        (NOP, imp, 2, 0), // xA
        (SRE, aby, 7, 0), // xB
        (NOP, abx, 4, 1), // xC
        (EOR, abx, 4, 1), // xD
        (LSR, abx, 7, 0), // xE
        (SRE, abx, 7, 0), // xF
        // 6x
        (RTS, imp, 6, 0), // x0
        (ADC, izx, 6, 0), // x1
        (KIL, imp, 0, 0), // x2
        (RRA, izx, 8, 0), // x3
        (NOP, zp,  3, 0), // x4
        (ADC, zp,  3, 0), // x5
        (ROR, zp,  5, 0), // x6
        (RRA, zp,  5, 0), // x7
        (PLA, imp, 4, 0), // x8
        (ADC, imm, 2, 0), // x9
        (ROR, imp, 2, 0), // xA
        (ARR, imm, 2, 0), // xB
        (JMP, ind, 5, 0), // xC
        (ADC, abs, 4, 0), // xD
        (ROR, abs, 6, 0), // xE
        (RRA, abs, 6, 0), // xF
        // 7x
        (BVS, rel, 2, 1), // x0
        (ADC, izy, 5, 1), // x1
        (KIL, imp, 0, 0), // x2
        (RRA, izy, 8, 0), // x3
        (NOP, zpx, 4, 0), // x4
        (ADC, zpx, 4, 0), // x5
        (ROR, zpx, 6, 0), // x6
        (RRA, zpx, 6, 0), // x7
        (SEI, imp, 2, 0), // x8
        (ADC, aby, 4, 1), // x9
        (NOP, imp, 2, 0), // xA
        (RRA, aby, 7, 0), // xB
        (NOP, abx, 4, 1), // xC
        (ADC, abx, 4, 1), // xD
        (ROR, abx, 7, 0), // xE
        (RRA, abx, 7, 0), // xF
        // 8x
        (NOP, imm, 2, 0), // x0
        (STA, izx, 6, 0), // x1
        (NOP, imm, 2, 0), // x2
        (SAX, izx, 6, 0), // x3
        (STY, zp,  3, 0), // x4
        (STA, zp,  3, 0), // x5
        (STX, zp,  3, 0), // x6
        (SAX, zp,  3, 0), // x7
        (DEY, imp, 2, 0), // x8
        (NOP, imm, 2, 0), // x9
        (TXA, imp, 2, 0), // xA
        (XAA, imm, 2, 1), // xB
        (STY, abs, 4, 0), // xC
        (STA, abs, 4, 0), // xD
        (STX, abs, 4, 0), // xE
        (SAX, abs, 4, 0), // xF
        // 9x
        (BCC, rel, 2, 1), // x0
        (STA, izy, 6, 0), // x1
        (KIL, imp, 0, 0), // x2
        (AHX, izy, 6, 0), // x3
        (STY, zpx, 4, 0), // x4
        (STA, zpx, 4, 0), // x5
        (STX, zpy, 4, 0), // x6
        (SAX, zpy, 4, 0), // x7
        (TYA, imp, 2, 0), // x8
        (STA, aby, 5, 0), // x9
        (TXS, imp, 2, 0), // xA
        (TAS, aby, 5, 0), // xB
        (SHY, abx, 5, 0), // xC
        (STA, abx, 5, 0), // xD
        (SHX, aby, 5, 0), // xE
        (AHX, aby, 5, 0), // xF
        // Ax
        (LDY, imm, 2, 0), // x0
        (LDA, izx, 6, 0), // x1
        (LDX, imm, 2, 0), // x2
        (LAX, izx, 6, 0), // x3
        (LDY, zp,  3, 0), // x4
        (LDA, zp,  3, 0), // x5
        (LDX, zp,  3, 0), // x6
        (LAX, zp,  3, 0), // x7
        (TAY, imp, 2, 0), // x8
        (LDA, imm, 2, 0), // x9
        (TAX, imp, 2, 0), // xA
        (LAX, imm, 2, 0), // xB
        (LDY, abs, 4, 0), // xC
        (LDA, abs, 4, 0), // xD
        (LDX, abs, 4, 0), // xE
        (LAX, abs, 4, 0), // xF
        // Bx
        (BCS, rel, 2, 1), // x0
        (LDA, izy, 5, 1), // x1
        (KIL, imp, 0, 0), // x2
        (LAX, izy, 5, 1), // x3
        (LDY, zpx, 4, 0), // x4
        (LDA, zpx, 4, 0), // x5
        (LDX, zpy, 4, 0), // x6
        (LAX, zpy, 4, 0), // x7
        (CLV, imp, 2, 0), // x8
        (LDA, aby, 4, 1), // x9
        (TSX, imp, 2, 0), // xA
        (LAS, aby, 4, 1), // xB
        (LDY, abx, 4, 1), // xC
        (LDA, abx, 4, 1), // xD
        (LDX, aby, 4, 1), // xE
        (LAX, aby, 4, 1), // xF
        // Cx
        (CPY, imm, 2, 0), // x0
        (CMP, izx, 6, 0), // x1
        (NOP, imm, 2, 0), // x2
        (DCP, izx, 8, 0), // x3
        (CPY, zp,  3, 0), // x4
        (CMP, zp,  3, 0), // x5
        (DEC, zp,  5, 0), // x6
        (DCP, zp,  5, 0), // x7
        (INY, imp, 2, 0), // x8
        (CMP, imm, 2, 0), // x9
        (DEX, imp, 2, 0), // xA
        (AXS, imm, 2, 0), // xB
        (CPY, abs, 4, 0), // xC
        (CMP, abs, 4, 0), // xD
        (DEC, abs, 6, 0), // xE
        (DCP, abs, 6, 0), // xF
        // Dx
        (BNE, rel, 2, 1), // x0
        (CMP, izy, 5, 1), // x1
        (KIL, imp, 0, 0), // x2
        (DCP, izy, 8, 0), // x3
        (NOP, zpx, 4, 0), // x4
        (CMP, zpx, 4, 0), // x5
        (DEC, zpx, 6, 0), // x6
        (DCP, zpx, 6, 0), // x7
        (CLD, imp, 2, 0), // x8
        (CMP, aby, 4, 1), // x9
        (NOP, imp, 2, 0), // xA
        (DCP, aby, 7, 0), // xB
        (NOP, abx, 4, 1), // xC
        (CMP, abx, 4, 1), // xD
        (DEC, abx, 7, 0), // xE
        (DCP, abx, 7, 0), // xF
        // Ex
        (CPX, imm, 2, 0), // x0
        (SBC, izx, 6, 0), // x1
        (NOP, imm, 2, 0), // x2
        (ISC, izx, 8, 0), // x3
        (CPX, zp,  3, 0), // x4
        (SBC, zp,  3, 0), // x5
        (INC, zp,  5, 0), // x6
        (ISC, zp,  5, 0), // x7
        (INX, imp, 2, 0), // x8
        (SBC, imm, 2, 0), // x9
        (NOP, imp, 2, 0), // xA
        (SBC, imm, 2, 0), // xB
        (CPX, abs, 4, 0), // xC
        (SBC, abs, 4, 0), // xD
        (INC, abs, 6, 0), // xE
        (ISC, abs, 6, 0), // xF
        // Fx
        (BEQ, rel, 2, 1), // x0
        (SBC, izy, 5, 1), // x1
        (KIL, imp, 0, 0), // x2
        (ISC, izy, 8, 0), // x3
        (NOP, zpx, 4, 0), // x4
        (SBC, zpx, 4, 0), // x5
        (INC, zpx, 6, 0), // x6
        (ISC, zpx, 6, 0), // x7
        (SED, imp, 2, 0), // x8
        (SBC, aby, 4, 1), // x9
        (NOP, imp, 2, 0), // xA
        (ISC, aby, 7, 0), // xB
        (NOP, abx, 4, 1), // xC
        (SBC, abx, 4, 1), // xD
        (INC, abx, 7, 0), // xE
        (ISC, abx, 7, 0), // xF
        ];

struct Instruction {
    op: Operation,
    mode: AddressingMode,
    mode_arguments: u16,
    write_target: Option<u16>,
    num_clocks: u8,
    oops_cycle: bool,
}

impl Instruction {
    fn advance_pc(&self) -> bool {
        match self.op {
            JMP => false,
            JSR => false,
            RTS => false,
            _ => true,
        }
    }
}
#[derive(Copy, Clone, Debug)]
enum Opcode {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    KIL,
    ISC,
    DCP,
    AXS,
    LAS,
    LAX,
    AHX,
    SAX,
    XAA,
    SHX,
    RRA,
    TAS,
    SHY,
    ARR,
    SRE,
    ALR,
    RLA,
    ANC,
    SLO,
}

#[derive(Debug, Clone, Copy)]
enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    Accumulator,
    Implicit,
}

use AdressingMode::*;
use Opcode::*;

const abs: AddressingMode = Absolute;
const acc: AddressingMode = Accumulator;
const imm: AddressingMode = Immediate;
const imp: AddressingMode = Implicit;
const izx: AddressingMode = IndirectX;
const izy: AddressingMode = IndirectY;
const zp: AddressingMode = ZeroPage;
const zpx: AddressingMode = ZeroPageX;
const zpy: AddressingMode = ZeroPageY;
const rel: AddressingMode = Relative;
const abx: AddressingMode = AbsoluteX;
const aby: AddressingMode = AbsoluteY;
const ind: AddressingMode = Indirect;
