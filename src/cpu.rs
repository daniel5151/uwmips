use crate::bus::Bus;
use crate::instr::Instr;

#[derive(Clone)]
pub struct CPU {
    /// CPU address space
    mem: Bus,
    /// Program Counter
    pc: u32,
    /// General Purpose Regsters (reg[0] is _always_ 0)
    reg: [u32; 32],
    /// hi multiplication / division register
    hi: u32,
    /// lo multiplication / division register
    lo: u32,
}

#[derive(Debug)]
pub enum Error {
    InvalidReg,
    BadInstr,
}

/// CPU Register. Used for traces / debugging.
pub enum Reg {
    /// Program Counter
    PC,
    /// High multiplication / division register
    Hi,
    /// Low multiplication / division register
    Lo,
    /// General Purpose Register
    Reg(usize),
}

impl CPU {
    /// Create a new CPU.
    pub fn new(mem: Bus, load_addr: u32) -> CPU {
        let mut cpu = CPU {
            mem,
            pc: load_addr,
            reg: [0; 32],
            hi: 0,
            lo: 0,
        };
        cpu.reg[29] = load_addr;
        cpu.reg[30] = 0x01000000 + load_addr;
        cpu.reg[31] = 0x8123456c;
        cpu
    }

    /// Peek a location in the CPU memory space.
    pub fn peek(&self, addr: u32) -> u32 {
        self.mem.peek(addr)
    }

    /// Perform a load in the CPU memory space.
    pub fn load(&mut self, addr: u32) -> u32 {
        self.mem.load(addr)
    }

    /// Perform a store in the CPU memory space.
    pub fn store(&mut self, addr: u32, val: u32) {
        self.mem.store(addr, val)
    }

    /// Set a register's value.
    /// Returns a Error::InvalidReg if the register index is out of bounds.
    pub fn set_reg(&mut self, reg: Reg, val: u32) -> Result<(), Error> {
        match reg {
            Reg::PC => self.pc = val,
            Reg::Hi => self.hi = val,
            Reg::Lo => self.lo = val,
            Reg::Reg(r) => {
                if r >= 32 {
                    return Err(Error::InvalidReg);
                }
                self.reg[r] = val;
            }
        }

        Ok(())
    }

    /// Get a register's value.
    /// Returns a Error::InvalidReg if the register index is out of bounds.
    pub fn get_reg(&self, reg: Reg) -> Result<u32, Error> {
        let val = match reg {
            Reg::PC => self.pc,
            Reg::Hi => self.hi,
            Reg::Lo => self.lo,
            Reg::Reg(r) => {
                if r >= 32 {
                    return Err(Error::InvalidReg);
                }
                self.reg[r]
            }
        };

        Ok(val)
    }

    /// Tick the CPU forward a single iteration
    /// Returns a bool indicating if the CPU is still running, or an [Error] if
    /// something went wrong.
    pub fn step(&mut self) -> Result<bool, Error> {
        // Check for jump back to "OS"
        if self.pc == 0x8123456c {
            return Ok(false);
        }

        let instr = Instr::from_u32(self.mem.load(self.pc));
        self.pc += 4;

        // println!("0x{:08x}: {}", self.pc - 4, instr);

        use crate::instr::{I::*, J, R::*};
        match instr {
            Instr::Inval(_) => return Err(Error::BadInstr),
            Instr::J { op, i } => match op {
                J::J => self.pc = i << 2,
                J::JAL => {
                    self.reg[31] = self.pc;
                    self.pc = i << 2;
                }
            },
            Instr::I { op, s, t, i } => match op {
                BEQ => {
                    if self.reg[s] == self.reg[t] {
                        self.pc = self.pc.wrapping_add(i.wrapping_mul(4))
                    }
                }
                BNE => {
                    if self.reg[s] != self.reg[t] {
                        self.pc = self.pc.wrapping_add(i.wrapping_mul(4))
                    }
                }
                ADDI => self.reg[t] = self.reg[s].wrapping_add(i),
                LW => self.reg[t] = self.mem.load(self.reg[s].wrapping_add(i)),
                SW => self.mem.store(self.reg[s].wrapping_add(i), self.reg[t]),
            },
            Instr::R { op, s, t, d } => match op {
                MFHI => self.reg[d] = self.hi,
                MFLO => self.reg[d] = self.lo,
                LIS => {
                    self.reg[d] = self.mem.load(self.pc);
                    self.pc += 4;
                }
                JR => self.pc = self.reg[s],
                JALR => {
                    let tmp = self.reg[s];
                    self.reg[31] = self.pc;
                    self.pc = tmp;
                }
                MULT => {
                    let res = (self.reg[s] as i64).wrapping_mul(self.reg[t] as i64);
                    self.hi = (res >> 32) as u32;
                    self.lo = (res >> 00) as u32;
                }
                MULTU => {
                    let res = (self.reg[s] as u64).wrapping_mul(self.reg[t] as u64);
                    self.hi = (res >> 32) as u32;
                    self.lo = (res >> 00) as u32;
                }
                DIV => {
                    self.hi = ((self.reg[s] as i32) % (self.reg[t] as i32)) as u32;
                    self.lo = (self.reg[s] as i32).wrapping_div(self.reg[t] as i32) as u32;
                }
                DIVU => {
                    self.hi = self.reg[s] % self.reg[t];
                    self.lo = self.reg[s].wrapping_div(self.reg[t]);
                }
                ADD => self.reg[d] = self.reg[s].wrapping_add(self.reg[t]),
                SUB => self.reg[d] = self.reg[s].wrapping_sub(self.reg[t]),
                SLT => self.reg[d] = ((self.reg[s] as i32) < (self.reg[t] as i32)) as u32,
                SLTU => self.reg[d] = (self.reg[s] < self.reg[t]) as u32,
            },
        }

        // Enforce that reg[0] is always 0
        self.reg[0] = 0;

        Ok(true)
    }
}

use std::fmt;
impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = (1..=31)
            .map(|i| {
                format!(
                    "${:02} = 0x{:08x} {:13}",
                    i,
                    self.reg[i],
                    format!("({})", self.reg[i] as i32)
                )
            })
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|chunk| chunk.join(" "))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{} $pc = 0x{:08x}", res, self.pc)
    }
}
