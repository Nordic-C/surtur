pub mod cli;
pub mod global;
pub mod tool;
pub mod util;

use cli::Project;

fn main() -> anyhow::Result<()> {
    global::init_dir()?;
    Project::try_open()?.exec()
}
