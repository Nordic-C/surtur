use cli::Cli;

mod cli;
mod compiler;
mod config;
mod creator;
mod deps;
mod initiator;
mod macros;
mod tips;
mod util;

fn main() {
    Cli::new().execute();
}
