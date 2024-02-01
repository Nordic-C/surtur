use std::process;

use colored::Colorize;

pub fn throw(msg: String, desc: String, command: String, err_pos: usize, err_len: usize) -> ! {
    let mut msg = format!(concat!(
        "{} {}\n\n{}\n"
    ), "Error:".red(), msg, command);
    for _ in 0..err_pos {
        msg.push(' ');
    }
    for _ in 0..err_len {
        msg.push('^');
    }
    msg.push_str("\n\n");
    for _ in 0..err_pos {
        msg.push(' ');
    }
    msg.push_str(&desc);
    println!("{}", msg);
    process::exit(1)
}
