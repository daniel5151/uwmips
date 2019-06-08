use std::io::Read;

use crate::mem::MEM;

/// Mediates CPU memory accesses.
#[derive(Clone, Debug)]
pub struct Bus {
    mem: MEM,
}

impl Bus {
    /// Create a new Bus instance
    pub fn new(mem: MEM) -> Bus {
        Bus { mem }
    }

    /// Reads a value from a specified `addr` without side-effects.
    pub fn peek(&self, addr: u32) -> u32 {
        match addr {
            0xffff0004 => 0x10101010,
            addr => self.mem.peek(addr),
        }
    }

    /// Read a value from a specified `addr`
    /// Reads from `0xffff0004` get a char from stdin.
    pub fn load(&mut self, addr: u32) -> u32 {
        match addr {
            0xffff0004 => std::io::stdin()
                .bytes()
                .next()
                .expect("unexpectedly ran out of stdin")
                .ok()
                .unwrap() as u32,
            addr => self.mem.load(addr),
        }
    }

    /// Write a value `val` into a specified `addr`
    /// Writes to `0xffff0004` put a char onto stdout.
    pub fn store(&mut self, addr: u32, val: u32) {
        match addr {
            0xffff000c => print!("{}", (val as u8) as char),
            addr => self.mem.store(addr, val),
        }
    }
}
