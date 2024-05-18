pub mod cli;
pub mod global;
pub mod util;

use cli::Cli;

fn main() -> anyhow::Result<()> {
    global::init_dir()?;
    Cli::default()?.exec()
}
