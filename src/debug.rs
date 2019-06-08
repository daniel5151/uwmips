use crate::cpu;
use crate::instr;

/// A debugger for MIPS CPUs.
pub struct Debugger {
    state: State,
    prev_command: Cmd,
    // past_states: Vec<cpu::CPU>,
}

#[derive(Debug, Copy, Clone)]
enum Cmd {
    Step,
    Exit,
    Run,
}

enum State {
    Running,
    AcceptCmd,
    Done,
}

impl Debugger {
    /// Create a new debugger instance.
    pub fn new() -> Debugger {
        Debugger {
            state: State::AcceptCmd,
            prev_command: Cmd::Step,
            // past_states: Vec::new(),
        }
    }

    /// Dump machine state in a pretty format.
    fn dump_cpu_state(&mut self, cpu: &cpu::CPU) {
        // Print Stack RAM
        let range = -6i32..=6;

        eprintln!("  -------------==== Stack ====-------------");
        eprintln!("       ADDR    |     HEX     |     VAL     ");
        eprintln!("  -------------|-------------|-------------");

        let stack_addr = cpu.get_reg(cpu::Reg::Reg(30)).unwrap();
        let range = range.map(|offset| stack_addr.wrapping_add((4 * offset) as u32));

        for addr in range {
            let indicator = if addr == stack_addr { '>' } else { ' ' };
            let val = cpu.peek(addr);
            eprintln!(
                "{}  0x{:08x}  | 0x{:08x}  | {}",
                indicator, addr, val, val as i32
            );
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
            let val = cpu.peek(addr);
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
                        "------------------------------------------------====== CPU State ======------------------------------------------------"
                    );
        eprintln!("{}", cpu);
    }

    fn exec_command(&mut self, cmd: Cmd, cpu: &mut &mut cpu::CPU) -> Result<(), String> {
        match cmd {
            Cmd::Run => self.state = State::Running,
            Cmd::Step => {
                let keep_running = cpu.step().map_err(|e| format!("CPU Error: {:?}", e))?;
                if !keep_running {
                    self.state = State::Done;
                }

                self.dump_cpu_state(cpu);
            }
            Cmd::Exit => std::process::exit(1),
        };

        self.prev_command = cmd;
        Ok(())
    }

    pub fn debug(&mut self, cpu: &mut &mut cpu::CPU) -> Result<(), String> {
        self.dump_cpu_state(cpu);
        loop {
            match self.state {
                State::Running => {
                    let keep_running = cpu.step().map_err(|e| format!("CPU Error: {:?}", e))?;
                    if !keep_running {
                        self.state = State::Done;
                    }
                }
                State::Done => {
                    eprintln!("Execution completed successfully!");
                    eprintln!("{}", cpu);
                    break Ok(());
                }
                State::AcceptCmd => {
                    eprint!("{:?}> ", self.prev_command);

                    let mut cmd = String::new();
                    std::io::stdin()
                        .read_line(&mut cmd)
                        .map_err(|_| "Failed to read next command")?;

                    let cmd = match &cmd[..cmd.len() - 1] {
                        "run" => Cmd::Run,
                        "step" | "s" => Cmd::Step,
                        "exit" | "quit" => Cmd::Exit,
                        "" => self.prev_command.clone(),
                        _ => {
                            eprintln!("Invalid commmand.");
                            continue;
                        }
                    };

                    self.exec_command(cmd, cpu)?;
                }
            };
        }
    }
}
