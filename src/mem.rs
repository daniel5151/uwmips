use std::collections::HashMap;

#[derive(Debug)]
pub struct MEM {
    mem: HashMap<u32, u32>,
}

impl MEM {
    pub fn new() -> MEM {
        MEM {
            mem: HashMap::new(),
        }
    }

    pub fn load(&mut self, addr: u32) -> u32 {
        *self.mem.get(&addr).unwrap_or(&0)
    }

    pub fn store(&mut self, addr: u32, val: u32) {
        self.mem.insert(addr, val);
    }
}
