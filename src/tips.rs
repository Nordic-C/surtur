/* This is for providing the user with tips for specifc errors */

use colored::Colorize;

pub fn missing_proj_name() -> String {
    let blue_line = "|".bright_blue();
    format!(
        r#"
    You need to specify a name for your project before creating it:
    {}   {} new your_name
    {}              {}
    {}              Replace with your projects name
    "#,
        blue_line,
        "surtur".yellow(),
        blue_line,
        "^^^^^^^^^".yellow(),
        blue_line
    )
}

pub fn invalid_run_arg(arg: &str) -> String {
    let blue_line = "|".bright_blue();
    format!(
        r#"
    `{}` is not a valid run argument:
    {}   {} run -dbg
    {}              {}
    {}              Replace with a valid argument like -dbg or -d
    {}              or leave empty to use default config
    "#,
        arg,
        blue_line,
        "surtur".yellow(),
        blue_line,
        "^^^^^^^^^".yellow(),
        blue_line,
        blue_line,
    )
}

pub fn invalid_build_arg(arg: &str) -> String {
    let blue_line = "|".bright_blue();
    format!(
        r#"
    `{}` is not a valid build argument:
    {}   {} build -release
    {}                {}
    {}                Replace with a valid argument like -release or -asm
    {}                or leave empty to use default config
    "#,
        arg,
        blue_line,
        "surtur".yellow(),
        blue_line,
        "^^^^^^^^^".yellow(),
        blue_line,
        blue_line,
    )
}
