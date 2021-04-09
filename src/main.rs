mod action;
mod command;
mod path;
mod seamds;
mod variable;

use std::{env, process}; // NOTE(Able): This style to make code folding easy and also less repitition

use lliw::Fg;
//use lliw::{Fg,Style}; // NOTE(Able): Commented out to reduce warnings

use action::Action;
use command::Command;
use path::Path;
use variable::Variable;

fn main() {
    let variables = Variable::load(); // NOTE(Able): Split out because functions inside functions weird me out
    let path = Path::new(); // NOTE(Able): Plus it also cuts down on repitition if you ever need to reuse those in main
    let sea = Sea::new(variables, path);

    sea.run();
}

struct Sea {
    home: String,
    vars: Variable,
    path: Path,
}

impl Sea {
    fn new(vars: Variable, path: Path) -> Self {
        Self {
            home: env::var("LOGNAME").unwrap(),
            vars,
            path,
        }
    }

    fn run(self) {
        let mut errstuffs: String;
        loop {
            let comd: Command;
            let cdir = env::current_dir().unwrap();
            let dir = cdir.to_str().unwrap();

            comd = Command::new(input!("{}{}@{}{} > ", Fg::Cyan, self.home, dir, Fg::Reset));

            let acton: Action;
            match self.cmd(comd) {
                Ok(a) => {
                    acton = a;
                    errstuffs = "".into()
                }
                Err(e) => {
                    acton = Action::Next();
                    errstuffs = e
                }
            }

            match acton {
                Action::Next() => print!("{}{}{} | ", Fg::Red, errstuffs, Fg::Reset),
                Action::Execute => {}
                Action::Exit => return,
            }
        }
    }

    fn cmd(&self, command: Command) -> Result<Action, String> {
        match command.executable.as_str() {
            "cd" => return cd!("{}", command.arguments[0]),
            "exit" => return Ok(Action::Exit),
            /*
            // NOTE(Able): First attempt at adding a builtin command
                        "path" => return self.path
            */
            _ => (),
        }

        let mut child = match process::Command::new(&command.executable)
            .args(&command.arguments)
            .spawn()
        {
            Ok(c) => c,
            Err(_e) => {
                // TODO(Able): Handle this error in case its not an executable not found error
                eprintln!("Error: {} not found in path", command.executable);
                return Err("seash code: 999".into());
            }
        };

        let e = match child.wait() {
            Ok(c) => c,
            Err(e) => {
                return Err(format!("{}", e));
            }
        };

        if !e.success() {
            return Err(e.to_string());
        }

        Ok(Action::Execute)
    }
}

#[macro_export]
macro_rules! input {
    ($($arg:tt)*) => ($crate::_input(format_args!($($arg)*)))
}

pub fn _input(text: std::fmt::Arguments) -> String {
    print!("{}", text);
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("failed to read stdin");
    buf.trim_end().to_string()
}
