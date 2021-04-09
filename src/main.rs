mod variable;
mod path;
mod action;
mod command;

use std::process::{self, ExitStatus};

use variable::Variable;
use path::Path;
// use action::Action;
use command::Command;

fn main() {
    let sea = Sea::new(Path::new());

    sea.run();
}

struct Sea {
    // vars: Variable,
    path: Path,
    // command: Command,
}

impl Sea {
    fn new(path: Path) -> Self {
        Self {
            path: path,
        }
    }

    fn run(self) {
        loop {
            let comd: Command;
            comd = Command::new(input!("> "));

            if let Err(e) = self.cmd(comd) {
            }
        }
    }

    fn cmd(&self, command: Command) -> Result<(), String> {
        let mut child = match process::Command::new(&command.executable)
            .args(&command.arguments)
            .spawn() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return Err("999".into())
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

        Ok(())
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
