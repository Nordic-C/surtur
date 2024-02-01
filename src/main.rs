use colored::Colorize;
use error::throw;

pub mod cli;
pub mod error;
pub mod util;

fn main() {
    throw(
        "Command is invalid. Cannot find a command with the name `testing`".into(),
        "Cannot find a command by the name `testing`, did you mean `test`?".into(),
        format!("surtur {}", "testing".bright_red()),
        7,
        7,
    );
}
