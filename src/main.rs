use cli::Cli;
pub mod cli;
pub mod util;

fn main() {
    Cli::default().execute()
}
