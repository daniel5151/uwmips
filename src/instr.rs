/// Register mode Opcodes
/// Bits 0..6 on a R mode instruction
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Copy, Clone)]
pub enum R {
    MFHI  = 0b_0001_0000,
    MFLO  = 0b_0001_0010,
    LIS   = 0b_0001_0100,
    JR    = 0b_0000_1000,
    JALR  = 0b_0000_1001,
    MULT  = 0b_0001_1000,
    MULTU = 0b_0001_1001,
    DIV   = 0b_0001_1010,
    DIVU  = 0b_0001_1011,
    ADD   = 0b_0010_0000,
    SUB   = 0b_0010_0010,
    SLT   = 0b_0010_1010,
    SLTU  = 0b_0010_1011,
}

/// Immediate mode Opcodes
/// Bits 27..31 on a I mode instruction
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Copy, Clone)]
pub enum I {
    BEQ  = 0b_0001_00,
    BNE  = 0b_0001_01,
    ADDI = 0b_0010_00,
    LW   = 0b_1000_11,
    SW   = 0b_1010_11,
}

/// Jump mode Opcodes
/// Bits 27..31 on a J mode instruction
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Copy, Clone)]
pub enum J {
    J   = 0b_0000_10,
    JAL = 0b_0000_11,
}

/// A MIPS instruction.
/// Consists of an opcode, and some associated operands.
#[derive(Copy, Clone)]
pub enum Instr {
    J { op: J, i: u32 },
    I { op: I, s: usize, t: usize, i: u32 },
    R { op: R, s: usize, t: usize, d: usize },
    Inval(u32),
}

/// Convert raw opcode bits into the associated enum
trait FromRawOp: Sized {
    fn from_raw_op(n: u8) -> Option<Self>;
}

impl FromRawOp for R {
    fn from_raw_op(n: u8) -> Option<R> {
        use R::*;
        let r = match n {
            0b_0001_0000 => MFHI,
            0b_0001_0010 => MFLO,
            0b_0001_0100 => LIS,
            0b_0000_1000 => JR,
            0b_0000_1001 => JALR,
            0b_0001_1000 => MULT,
            0b_0001_1001 => MULTU,
            0b_0001_1010 => DIV,
            0b_0001_1011 => DIVU,
            0b_0010_0000 => ADD,
            0b_0010_0010 => SUB,
            0b_0010_1010 => SLT,
            0b_0010_1011 => SLTU,
            _ => return None,
        };
        Some(r)
    }
}

impl FromRawOp for I {
    fn from_raw_op(n: u8) -> Option<I> {
        use I::*;
        let i = match n {
            0b_0001_00 => BEQ,
            0b_0001_01 => BNE,
            0b_0010_00 => ADDI,
            0b_1000_11 => LW,
            0b_1010_11 => SW,
            _ => return None,
        };
        Some(i)
    }
}

impl FromRawOp for J {
    fn from_raw_op(n: u8) -> Option<J> {
        let j = match n {
            0b_0000_10 => J::J,
            0b_0000_11 => J::JAL,
            _ => return None,
        };
        Some(j)
    }
}

impl Instr {
    /// Try to convert a raw 32 bit word into a MIPS instruction.
    /// Instead of returning an Optional, [Instr] includes a `Invalid` variant,
    /// which represents a u32 which doesn't match any known MIPS instruction.
    pub fn from_u32(raw: u32) -> Instr {
        match raw >> (24 + 2) {
            // R all start with 0000 00
            0 => {
                let s = (raw >> (20 + 1)) & 0b11111;
                let t = (raw >> (16 + 0)) & 0b11111;
                let d = (raw >> (8 + 3)) & 0b11111;
                let op = match R::from_raw_op((raw & 0b_1111_1111) as u8) {
                    Some(op) => op,
                    None => return Instr::Inval(raw),
                };

                Instr::R {
                    op: op,
                    s: s as usize,
                    t: t as usize,
                    d: d as usize,
                }
            }
            // J start with 0000 XX
            op if op >> 2 == 0 => {
                let i = raw & 0b_11_1111_1111_1111_1111_1111_1111;
                let op = match J::from_raw_op(op as u8) {
                    Some(op) => op,
                    None => return Instr::Inval(raw),
                };

                Instr::J {
                    op: op,
                    i: i as u32,
                }
            }
            // I start with non-0000
            op => {
                let s = (raw >> (20 + 1)) & 0b11111;
                let t = (raw >> (16 + 0)) & 0b11111;
                // casts required to properly sign extend
                let i = (raw as i16) as i32;
                let op = match I::from_raw_op(op as u8) {
                    Some(op) => op,
                    None => return Instr::Inval(raw),
                };

                Instr::I {
                    op: op,
                    s: s as usize,
                    t: t as usize,
                    i: i as u32,
                }
            }
        }
    }
}

// ------------------------- Display Implementations ------------------------ //

use std::fmt;

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use I::*;
        use R::*;
        match *self {
            Instr::J { op, i } => write!(f, "{:<5} 0x{:08x}", op, i),
            Instr::I { op, s, t, i } => {
                let i = i as i16;
                #[cfg_attr(rustfmt, rustfmt_skip)]
                match op {
                    BEQ | BNE  => write!(f, "{:<5} ${}, ${}, {}", op, s, t, i),
                    ADDI       => write!(f, "{:<5} ${}, ${}, {}", op, t, s, i),
                    LW  | SW   => write!(f, "{:<5} ${}, {}(${})", op, t, i, s),
                }
            }
            Instr::R { op, s, t, d } =>
            {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                match op {
                    MFHI | MFLO  | LIS        => write!(f, "{:<5} ${}", op, d),
                    JR   | JALR               => write!(f, "{:<5} ${}", op, s),
                    MULT | MULTU | DIV | DIVU => write!(f, "{:<5} ${}, ${}", op, s, t),
                    ADD  | SUB   | SLT | SLTU => write!(f, "{:<5} ${}, ${}, ${}", op, d, s, t),
                }
            }
            Instr::Inval(raw) => write!(f, ".word 0x{:08x} ({})", raw, raw as i32),
        }
    }
}

impl fmt::Display for R {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use R::*;
        #[cfg_attr(rustfmt, rustfmt_skip)]
        match *self {
            MFHI  => write!(f, "mfhi"),
            MFLO  => write!(f, "mflo"),
            LIS   => write!(f, "lis"),
            JR    => write!(f, "jr"),
            JALR  => write!(f, "jalr"),
            MULT  => write!(f, "mult"),
            MULTU => write!(f, "multu"),
            DIV   => write!(f, "div"),
            DIVU  => write!(f, "divu"),
            ADD   => write!(f, "add"),
            SUB   => write!(f, "sub"),
            SLT   => write!(f, "slt"),
            SLTU  => write!(f, "sltu"),
        }
    }
}

impl fmt::Display for I {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use I::*;
        #[cfg_attr(rustfmt, rustfmt_skip)]
        match *self {
            BEQ  => write!(f, "beq"),
            BNE  => write!(f, "bne"),
            ADDI => write!(f, "addi"),
            LW   => write!(f, "lw"),
            SW   => write!(f, "sw"),
        }
    }
}

impl fmt::Display for J {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        match *self {
            J::J   => write!(f, "j"),
            J::JAL => write!(f, "jal"),
        }
    }
}
