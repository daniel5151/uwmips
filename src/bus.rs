use std::io::Read;

use crate::mem::MEM;

#[derive(Debug)]
pub struct Bus {
    mem: MEM,
}

pub enum Effect {}

impl Bus {
    pub fn new(mem: MEM) -> Bus {
        Bus { mem }
    }

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

    pub fn store(&mut self, addr: u32, val: u32) {
        match addr {
            0xffff000c => print!("{}", (addr as u8) as char),
            addr => self.mem.store(addr, val),
        }
    }
}
