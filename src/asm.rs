use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern "C" {
    fn assemble(lines_raw: *const c_char) -> *const c_char;
    fn get_word() -> u64;
}

pub struct Asm;
impl Asm {
    pub fn assemble(lines: String) -> Result<Vec<u32>, String> {
        let lines_raw = CString::new(lines).unwrap();
        let err_str = unsafe { CStr::from_ptr(assemble(lines_raw.as_ptr())) }
            .to_str()
            .unwrap();
        match err_str {
            "" => {
                let mut out = Vec::new();
                loop {
                    let val = unsafe { get_word() };
                    if val != (-1i64 as u64) {
                        out.push(val as u32);
                    } else {
                        break Ok(out);
                    }
                }
            }
            err => Err(err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert!(vec![0x3e00008] == Asm::assemble("jr $31".to_string()).unwrap());
    }
}
