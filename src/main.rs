mod variable;
mod path;
mod action;
mod command;
mod seamds;

use std::env::{self, Vars};
use std::process;

use lliw::{Style, Fg};

use variable::Variable;
use path::Path;
use action::Action;
use command::Command;

fn main() {
    let sea = Sea::new(Variable::load(), Path::new());

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
            path
        }
    }

    fn run(self) {
        let mut errstuffs: String = "0".into();
        loop {
            let comd: Command;
            let cdir = env::current_dir().unwrap();
            let dir = cdir.to_str().unwrap();

            comd = Command::new(input!("{}@{}> ", self.home, dir));

            let acton: Action;
            match self.cmd(comd) {
                Ok(a) => {acton = a; errstuffs = "".into()}
                Err(e) => {acton = Action::Next(); errstuffs = e}
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
            _ => (),
        }

        let mut child = match process::Command::new(&command.executable)
            .args(&command.arguments)
            .spawn() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return Err("seash code: 999".into())
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
    std::io::stdin().read_line(&mut buf).expect("failed to read stdin");
    buf.trim_end().to_string()
}
