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
        "Usage: {} [frontend] <filename> [...args] [load_address]",
        exec_name
    );
    eprintln!("  frontend: twoints     - <no args>");
    eprintln!("            twointsargs - <int1> <int2>");
    eprintln!("            array       - <no args>");
    std::process::exit(1);
}

enum InputFrontend {
    NoArgs,
    TwoInts { int1: i32, int2: i32 },
    Array { array: Vec<i32> },
}

struct ParsedArgs {
    filename: String,
    frontend: InputFrontend,
    load_address: u32,
}

fn parse_args() -> Result<ParsedArgs, String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        return Err("No frontend specified".to_string());
    }

    if args.len() == 2 {
        return Err("No filename specified".to_string());
    }

    let mut load_address_str = "0";

    let frontend = match args[1].as_ref() {
        "noargs" => {
            if args.len() == 4 {
                load_address_str = &args[3]
            }
            InputFrontend::NoArgs
        }
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

            if args.len() == 4 {
                load_address_str = &args[3]
            }

            InputFrontend::TwoInts {
                int1: ints[0],
                int2: ints[1],
            }
        }
        "twointsargs" => {
            if args.len() == 3 {
                return Err("int1 not specified".to_string());
            }
            if args.len() == 4 {
                return Err("int2 not specified".to_string());
            }

            if args.len() == 6 {
                load_address_str = &args[5]
            }

            InputFrontend::TwoInts {
                int1: args[3].parse().map_err(|_| "Failed to parse int1")?,
                int2: args[4].parse().map_err(|_| "Failed to parse int2")?,
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

            if args.len() == 4 {
                load_address_str = &args[3]
            }

            InputFrontend::Array { array }
        }
        _ => return Err("Invalid mode".to_string()),
    };

    let load_address = load_address_str
        .parse()
        .map_err(|_| "Failed to parse load load_address")?;

    if load_address % 4 != 0 {
        return Err("load_address must be word aligned".to_string());
    }

    Ok(ParsedArgs {
        filename: args[2].clone(),
        frontend,
        load_address,
    })
}

fn main() {
    let ParsedArgs {
        filename,
        frontend,
        load_address,
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
            let _ = cpu.set_reg(1, int1 as u32);
            let _ = cpu.set_reg(2, int2 as u32);
        }
        InputFrontend::Array { array } => {
            let base = 0x20 + load_address;
            for (i, n) in array.iter().enumerate() {
                cpu.store(base + (i as u32) * 4, *n as u32);
            }
            let _ = cpu.set_reg(1, base);
            let _ = cpu.set_reg(2, array.len() as u32);
        }
    }

    // Step 3: Run the VM
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
