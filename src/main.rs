pub mod cli;
pub mod global;
pub mod tool;
pub mod util;

use cli::Cli;

fn main() -> anyhow::Result<()> {
    global::init_dir()?;
    Cli::new()?.exec()
}
