pub enum InputFrontend {
    NoArgs,
    TwoInts { int1: i32, int2: i32 },
    Array { array: Vec<i32> },
}

pub struct ParsedArgsFlags {
    pub debug: bool,
}

pub struct ParsedArgs {
    pub filename: String,
    pub frontend: InputFrontend,
    pub load_address: u32,
    pub flags: ParsedArgsFlags,
}

pub fn parse_args() -> Result<ParsedArgs, String> {
    let args: Vec<String> = std::env::args().collect();

    let mut arg = 1;

    // Consume flags
    let mut flags = ParsedArgsFlags { debug: false };
    loop {
        match args.get(arg) {
            Some(s) => match s.as_ref() {
                "--debug" => {
                    flags.debug = true;
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
