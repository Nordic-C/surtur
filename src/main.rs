use std::{
    env,
    fs::{self, File},
    io::Read,
    process::Command,
};

use rlua::{Lua, Result, Table};

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let result = eval_lua();

    // TODO: implement error handling
    if args.get(1).unwrap() == "run" {
        run_c();
    }
}

fn run_c() {
    build_c();
    let command = "./example/build/main.exe";

    let mut child = Command::new(command);

    let result = child.output();

    match result {
        Ok(output) => {
            if output.status.success() {
                println!(
                    "Command output:\n{}",
                    String::from_utf8_lossy(&output.stdout)
                );
            } else {
                eprintln!(
                    "Command failed with error: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}

fn build_c() {
    let cmd = "gcc";

    let build_path = "C:/Users/Admin/programming/rust/surtur/example/build";

    let mut binding = Command::new(cmd);
    let output = binding
        .arg("C:/Users/Admin/programming/rust/surtur/example/src/main.c")
        .arg("-o")
        .arg(format!("{}/main.exe", build_path));

    match output.status() {
        Ok(_) => println!("sucess"),
        Err(_) => println!("error"),
    }

    println!("{:?}", output)
}

fn eval_lua() -> Result<()> {
    let lua = Lua::new();

    let mut file = File::open("example/project.lua").expect("Failed to open file");

    let mut source = String::new();

    file.read_to_string(&mut source)
        .expect("Failed to read file");

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
