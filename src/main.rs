//! `uwmips` - A simulator for the MIPS instruction set used in CS241 and CS230
//! at the University of Waterloo.

use std::fs::File;
use std::io::Read;

use crate::args::*;
use crate::debug::Debugger;

mod args;
pub mod bus;
pub mod cpu;
mod debug;
pub mod instr;
pub mod mem;

fn print_usage() -> ! {
    let exec_name = std::env::args().next().unwrap();
    eprintln!();
    eprintln!(
        "Usage: {} [OPTIONS] [frontend] <filename> [...args] [load_address]",
        exec_name
    );
    eprintln!("   OPTIONS: --debug     Launch an interactive debugger");
    eprintln!();
    eprintln!("  frontend: twoints     - <no args>");
    eprintln!("            twointsargs - <int1> <int2>");
    eprintln!("            array       - <no args>");
    eprintln!("            noargs      - <no args>");
    std::process::exit(1);
}

fn main() {
    let ParsedArgs {
        filename,
        frontend,
        load_address,
        flags,
    } = match parse_args() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Error! {}", err);
            print_usage();
        }
    };

    // Construct the VM
    let mem = mem::MEM::new();
    let bus = bus::Bus::new(mem);
    let mut cpu = cpu::CPU::new(bus, load_address);

    // Step 1: Load program into memory
    let f = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    };

    f.bytes()
        .map(|b| b.map(|b| b as u32))
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .chunks(4)
        .map(|c| {
            if c.len() != 4 {
                eprintln!("File is not word aligned");
                std::process::exit(1)
            }
            c[0] << 24 | c[1] << 16 | c[2] << 8 | c[3]
        })
        .enumerate()
        .for_each(|tup| {
            let (i, word) = tup;
            cpu.store(load_address + (i as u32) * 4, word);
        });

    // Step 2: Load args into memory
    match frontend {
        InputFrontend::NoArgs => {}
        InputFrontend::TwoInts { int1, int2 } => {
            let _ = cpu.set_reg(cpu::Reg::Reg(1), int1 as u32);
            let _ = cpu.set_reg(cpu::Reg::Reg(2), int2 as u32);
        }
        InputFrontend::Array { array } => {
            let base = 0x20 + load_address;
            for (i, n) in array.iter().enumerate() {
                cpu.store(base + (i as u32) * 4, *n as u32);
            }
            let _ = cpu.set_reg(cpu::Reg::Reg(1), base);
            let _ = cpu.set_reg(cpu::Reg::Reg(2), array.len() as u32);
        }
    }

    // Step 3: Run the VM
    if flags.debug {
        let mut debugger = Debugger::new();
        if let Err(msg) = debugger.debug(&mut &mut cpu) {
            eprintln!("Error! {}", msg);
            std::process::exit(1);
        }
    } else {
        loop {
            match cpu.step() {
                Ok(true) => { /* keep on running */ }
                Ok(false) => {
                    eprintln!("Execution completed successfully!");
                    break;
                }
                Err(err) => {
                    eprintln!("Error! {:?}", err);
                    break;
                }
            }
        }
        // Dump final CPU state
        eprintln!("{}", cpu);
    }
}
