use std::{env, fmt::Arguments};

use crate::action::Action;

#[macro_export]
macro_rules! cd {
    ($($arg:tt)*) => ($crate::seamds::_cd(format_args!($($arg)*)))
}

pub fn _cd(dir: Arguments) -> Result<Action, String> {
    if let Err(e) = env::set_current_dir(dir.to_string()) {
        return Err(e.to_string());
    }
    Ok(Action::Execute)
}
