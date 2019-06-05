//! `uwmips` - A simulator for the MIPS instruction set used in CS241 and CS230
//! at the University of Waterloo.

use std::fs::File;
use std::io::Read;

pub mod bus;
pub mod cpu;
pub mod instr;
pub mod mem;

fn print_usage() -> ! {
    let exec_name = std::env::args().next().unwrap();
    eprintln!();
    eprintln!(
        "Usage: {} [OPTIONS] [frontend] <filename> [...args] [load_address]",
        exec_name
    );
    eprintln!("   OPTIONS: --step      Dump state after every CPU instruction");
    eprintln!("                          and wait for 'enter' key to continue");
    eprintln!();
    eprintln!("  frontend: twoints     - <no args>");
    eprintln!("            twointsargs - <int1> <int2>");
    eprintln!("            array       - <no args>");
    eprintln!("            noargs      - <no args>");
    std::process::exit(1);
}

enum InputFrontend {
    NoArgs,
    TwoInts { int1: i32, int2: i32 },
    Array { array: Vec<i32> },
}

struct ParsedArgsFlags {
    step: bool,
}

struct ParsedArgs {
    filename: String,
    frontend: InputFrontend,
    load_address: u32,
    flags: ParsedArgsFlags,
}

fn parse_args() -> Result<ParsedArgs, String> {
    let args: Vec<String> = std::env::args().collect();

    let mut arg = 1;

    // Consume flags
    let mut flags = ParsedArgsFlags { step: false };
    loop {
        match args.get(arg) {
            Some(s) => match s.as_ref() {
                "--step" => {
                    flags.step = true;
                    arg += 1;
                }
                _ => break,
            },
            None => return Err("Not enough arguments".to_string()),
        }
    }

    // Consume frontend
    if args.get(arg).is_none() {
        return Err("No frontend specified".to_string());
    }
    let frontend = &args[arg];
    arg += 1;

    // Consume Filename
    if args.get(arg).is_none() {
        return Err("No filename specified".to_string());
    }
    let filename = args[arg].clone();
    arg += 1;

    // Per-frontend matching
    let frontend = match frontend.as_ref() {
        "noargs" => InputFrontend::NoArgs,
        "twoints" => {
            let mut ints: [i32; 2] = [0; 2];

            for i in 0..2 {
                eprint!("Enter value for register {}: ", i + 1);
                let mut buf = String::new();
                std::io::stdin()
                    .read_line(&mut buf)
                    .map_err(|_| format!("Failed to read register {} value", i + 1))?;

                ints[i] = buf[..buf.len() - 1]
                    .parse()
                    .map_err(|_| format!("Failed to parse register {} value", i + 1))?;
            }

            InputFrontend::TwoInts {
                int1: ints[0],
                int2: ints[1],
            }
        }
        "twointsargs" => {
            if args.get(arg).is_none() {
                return Err("int1 not specified".to_string());
            }
            if args.get(arg + 1).is_none() {
                return Err("int2 not specified".to_string());
            }

            arg += 2;

            InputFrontend::TwoInts {
                int1: args[arg - 2].parse().map_err(|_| "Failed to parse int1")?,
                int2: args[arg - 1].parse().map_err(|_| "Failed to parse int2")?,
            }
        }
        "array" => {
            let mut array = Vec::new();

            eprint!("Enter length of array: ");
            let mut buf = String::new();
            std::io::stdin()
                .read_line(&mut buf)
                .map_err(|_| "Failed to read length of array")?;
            let array_len: usize = buf[..buf.len() - 1]
                .parse()
                .map_err(|_| "Failed to parse length of array")?;

            for i in 0..array_len {
                eprint!("Enter array element {}: ", i);
                let mut buf = String::new();
                std::io::stdin()
                    .read_line(&mut buf)
                    .map_err(|_| "Failed to read array element")?;

                array.push(
                    buf[..buf.len() - 1]
                        .parse()
                        .map_err(|_| "Failed to parse array element")?,
                );
            }

            InputFrontend::Array { array }
        }
        _ => return Err("Invalid mode".to_string()),
    };

    // check for load_address
    let load_address = match args.get(arg) {
        Some(addr) => addr
            .parse()
            .map_err(|_| "Failed to parse load load_address")?,
        None => 0,
    };

    if load_address % 4 != 0 {
        return Err("load_address must be word aligned".to_string());
    }

    Ok(ParsedArgs {
        filename,
        frontend,
        load_address,
        flags,
    })
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
    loop {
        match cpu.step() {
            Ok(true) => {
                /* keep on running */
                if flags.step {
                    // Print Stack RAM
                    let range = -6i32..=6;

                    eprintln!("  ----==== Stack ====-----");
                    eprintln!("     ADDR    |   HEXVAL   ");
                    eprintln!("  -----------|------------");

                    let stack_addr = cpu.get_reg(cpu::Reg::Reg(30)).unwrap();
                    let range = range.map(|offset| stack_addr.wrapping_add((4 * offset) as u32));

                    for addr in range {
                        let indicator = if addr == stack_addr { '>' } else { ' ' };
                        let val = cpu.load(addr);
                        eprintln!("{} 0x{:08x} | 0x{:08x}", indicator, addr, val,);
                    }

                    eprintln!();

                    // Print Program RAM
                    let range = -6i32..=6;

                    eprintln!("  ---------====== Program RAM ======--------");
                    eprintln!("     ADDR    |   HEXVAL   :     MIPS ASM    ");
                    eprintln!("  -----------|------------------------------");

                    let pc = cpu.get_reg(cpu::Reg::PC).unwrap();
                    let range = range.map(|offset| pc.wrapping_add((4 * offset) as u32));

                    for addr in range {
                        let indicator = if addr == pc { '>' } else { ' ' };
                        let val = cpu.load(addr);
                        eprintln!(
                            "{} 0x{:08x} | 0x{:08x} : {}",
                            indicator,
                            addr,
                            val,
                            instr::Instr::from_u32(val)
                        );
                    }

                    eprintln!();

                    // Print CPU State
                    eprintln!(
                        "-------------------------====== CPU State ======-------------------------"
                    );
                    eprintln!("{}", cpu);

                    // Wait for input to continue
                    let mut buf = String::new();
                    let _ = std::io::stdin().read_line(&mut buf);
                }
            }
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
