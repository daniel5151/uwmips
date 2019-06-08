use std::collections::HashMap;

/// A generic word addressable memory structure.
// TODO: enforce word addressing
// TODO: switch to something more efficient than a HashMap lol
#[derive(Clone, Debug)]
pub struct MEM {
    mem: HashMap<u32, u32>,
}

impl MEM {
    /// Create a new MEM instance
    pub fn new() -> MEM {
        MEM {
            mem: HashMap::new(),
        }
    }

    pub fn peek(&self, addr: u32) -> u32 {
        *self.mem.get(&addr).unwrap_or(&0)
    }

    /// Read a value from a specified `addr`
    pub fn load(&mut self, addr: u32) -> u32 {
        self.peek(addr)
    }

    /// Write a value `val` into a specified `addr`
    pub fn store(&mut self, addr: u32, val: u32) {
        self.mem.insert(addr, val);
    }
}
