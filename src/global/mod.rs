//! Module responsible for global file system operations
//! like installing dependencies and saving configurations

use std::fs;

use anyhow::Context;
use dirs::home_dir;

/// This creates the .surtur directory if it does not exist yet
pub(super) fn init_dir() -> anyhow::Result<()> {
    let home = home_dir().context("Failed")?;
    let surtur_path = &home.join(".surtur");
    if !surtur_path.exists() {
        fs::create_dir(surtur_path)?;
    }
    Ok(())
}
