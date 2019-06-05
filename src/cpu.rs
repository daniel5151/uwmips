use std::fmt;

use crate::bus::Bus;
use crate::instr::Instr;

pub struct CPU {
    mem: Bus,
    pc: u32,
    reg: [u32; 32],
    hi: u32,
    lo: u32,
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = (1..=31)
            .map(|i| format!("${:02} = 0x{:08x}", i, self.reg[i]))
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|chunk| chunk.join("   "))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}   $pc = 0x{:08x}", res, self.pc)
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidReg,
    BadInstr,
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

    pub fn load(&mut self, addr: u32) -> u32 {
        self.mem.load(addr)
    }

    pub fn store(&mut self, addr: u32, val: u32) {
        self.mem.store(addr, val)
    }

    pub fn set_reg(&mut self, reg: usize, val: u32) -> Result<(), Error> {
        if reg >= 32 {
            return Err(Error::InvalidReg);
        }

        self.reg[reg] = val;

        Ok(())
    }

    pub fn step(&mut self) -> Result<bool, Error> {
        use crate::instr::{I::*, J, R::*};

        if self.pc == 0x8123456c {
            return Ok(false);
        }

        let instr = Instr::from_u32(self.mem.load(self.pc));
        self.pc += 4;

        match instr {
            Instr::Inval(_) => return Err(Error::BadInstr),
            Instr::J { op, i } => match op {
                J::J => self.pc = i,
                J::JAL => {
                    self.reg[31] = self.pc;
                    self.pc = i
                }
            },
            Instr::I { op, s, t, i } => match op {
                BEQ => {
                    if self.reg[s] == self.reg[t] {
                        self.pc = self.pc.wrapping_add(i * 4)
                    }
                }
                BNE => {
                    if self.reg[s] != self.reg[t] {
                        self.pc = self.pc.wrapping_add(i * 4)
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

        Ok(true)
    }
}
