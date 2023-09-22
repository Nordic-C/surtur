use std::{fs::File, io::Read};

use rlua::{Lua, Result, Table};

fn main() -> Result<()> {
    let lua = Lua::new();

    let mut file = File::open("example/project.lua").expect("Failed to open file");

    let mut source = String::new();

    file.read_to_string(&mut source).expect("Failed to read file");

    lua.context(|ctx| {
        ctx.load(&source).exec()?;

        let my_table: Table = ctx.globals().get("deps")?;

        for key in my_table.pairs::<String, String>() {
            let (k, v) = key?;
            println!("Key: {}, Value: {}", k, v);
        }

        Ok(())
    })
}
