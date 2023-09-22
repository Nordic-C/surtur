extern crate rlua;

use std::{fs::File, io::Read};

use rlua::{Lua, Result, Table};

fn main() -> Result<()> {
    // Create a Lua runtime environment
    let lua = Lua::new();

    let mut file = File::open("example/project.lua").expect("Failed to open file");

    // Lua code to evaluate
    let mut lua_code = String::new();

    file.read_to_string(&mut lua_code).expect("Failed to read file");

    // Evaluate the Lua code
    lua.context(|ctx| {
        ctx.load(&lua_code).exec()?;

        // Access the Lua table "my_table"
        let my_table: Table = ctx.globals().get("deps")?;

        // Now you have a Table containing the Lua table
        // You can work with it as needed
        for key in my_table.pairs::<String, String>() {
            let (k, v) = key?;
            println!("Key: {}, Value: {}", k, v);
        }

        Ok(())
    })
}
