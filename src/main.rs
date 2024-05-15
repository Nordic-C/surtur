use cli::Cli;
pub mod cli;
pub mod util;

fn main() -> anyhow::Result<()> {
    Cli::default()?.exec()
}
