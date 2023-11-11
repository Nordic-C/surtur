use cli::Cli;

mod compiler;
mod cli;
mod config;
mod creator;
mod initiator;
mod deps;
mod util;
mod tips;

fn main() {
    Cli::new().execute();
}
