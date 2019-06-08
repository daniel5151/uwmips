use std::str::FromStr;

use crate::cpu;
use crate::instr;

/// A (time-travelling!) debugger for MIPS CPUs.
pub struct Debugger {
    cpu: cpu::CPU,
    state: State,
    prev_command: Cmd,
    past_states: Vec<cpu::CPU>,
}

/// Debugger commands
#[derive(Debug, Copy, Clone)]
enum Cmd {
    Step,
    StepBackwards,
    Exit,
    Run,
    Help,
}

impl FromStr for Cmd {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Cmd, &'static str> {
        let cmd = match s {
            "run" => Cmd::Run,
            "step" | "s" | "sf" => Cmd::Step,
            "step-backwards" | "sb" => Cmd::StepBackwards,
            "exit" | "quit" | "q" => Cmd::Exit,
            "help" => Cmd::Help,
            _ => return Err("Invalid Command"),
        };
        Ok(cmd)
    }
}

impl Cmd {
    fn help() {
        eprintln!("  run ------------ run program");
        eprintln!("  step ----------- step forward a single instruction");
        eprintln!("  | s");
        eprintln!("  | sf");
        eprintln!("  step-backwards - step backwards a single instruction");
        eprintln!("  | sb");
        eprintln!("  exit ----------- quit the debugger");
        eprintln!("  | quit");
        eprintln!("  | q");
        eprintln!("  help ----------- open help");
    }
}

enum State {
    Running,
    AcceptCmd,
    Done,
}

impl Debugger {
    /// Create a new debugger instance.
    pub fn new(cpu: cpu::CPU) -> Debugger {
        Debugger {
            cpu,
            state: State::AcceptCmd,
            prev_command: Cmd::Step,
            past_states: Vec::new(),
        }
    }

    /// Dump machine state in a pretty format.
    fn dump_cpu_state(&mut self) {
        // Print Stack RAM
        let range = -6i32..=6;

        eprintln!("  -------------==== Stack ====-------------");
        eprintln!("       ADDR    |     HEX     |     VAL     ");
        eprintln!("  -------------|-------------|-------------");

        let stack_addr = self.cpu.get_reg(cpu::Reg::Reg(30)).unwrap();
        let range = range.map(|offset| stack_addr.wrapping_add((4 * offset) as u32));

        for addr in range {
            let indicator = if addr == stack_addr { '>' } else { ' ' };
            let val = self.cpu.peek(addr);
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

        let pc = self.cpu.get_reg(cpu::Reg::PC).unwrap();
        let range = range.map(|offset| pc.wrapping_add((4 * offset) as u32));

        for addr in range {
            let indicator = if addr == pc { '>' } else { ' ' };
            let val = self.cpu.peek(addr);
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
        eprintln!("{}", self.cpu);
    }

    fn step_cpu(&mut self) -> Result<(), String> {
        let prev_cpu = self.cpu.clone();
        self.past_states.push(prev_cpu);

        let keep_running = self.cpu.step().map_err(|e| format!("CPU Error: {:?}", e))?;
        if !keep_running {
            self.state = State::Done;
        }

        Ok(())
    }

    fn exec_command(&mut self, cmd: Cmd) -> Result<(), String> {
        match cmd {
            Cmd::Run => self.state = State::Running,
            Cmd::Step => {
                self.step_cpu()?;
                self.dump_cpu_state();
            }
            Cmd::StepBackwards => {
                // Retrieve previous CPU state
                if let Some(prev_cpu) = self.past_states.pop() {
                    self.cpu = prev_cpu;
                }
                self.dump_cpu_state();
            }
            Cmd::Exit => std::process::exit(1),
            Cmd::Help => {
                Cmd::help();
                return Ok(());
            }
        };

        self.prev_command = cmd;
        Ok(())
    }

    pub fn debug(&mut self) -> Result<(), String> {
        self.dump_cpu_state();

        loop {
            match self.state {
                State::Running => self.step_cpu()?,
                State::Done => {
                    eprintln!("Execution completed successfully!");
                    eprintln!("{}", self.cpu);
                    break Ok(());
                }
                State::AcceptCmd => {
                    eprint!("{:?}> ", self.prev_command);

                    let mut cmd = String::new();
                    std::io::stdin()
                        .read_line(&mut cmd)
                        .map_err(|_| "Failed to read next command")?;

                    let cmd = match cmd[..cmd.len() - 1].parse::<Cmd>() {
                        Ok(cmd) => cmd,
                        Err(_) => {
                            if cmd == "\n" {
                                self.prev_command.clone()
                            } else {
                                eprintln!("Invalid commmand.");
                                continue;
                            }
                        }
                    };

                    self.exec_command(cmd)?;
                }
            };
        }
    }
}
