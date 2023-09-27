/* This is for providing the user with tips for specifc errors */

use colored::Colorize;

pub enum Tip {
    MissingProjName,
    InvalidRunArg,
    InvalidBuildArg,
}

impl Tip {
    pub fn literal(&self) -> &str {
        match self {
            Self::InvalidRunArg => "Replace with a valid argument like -dbg or leave empty",
            Self::MissingProjName => "Put your project's name after `new`",
            Self::InvalidBuildArg => "Replace with a valid argument like -release or -asm",
        }
    }

    pub fn get_cmd(&self) -> &str {
        match self {
            Self::InvalidRunArg => "run -dbg",
            Self::MissingProjName => "new your_name",
            Self::InvalidBuildArg => "build -release",
        }
    }

    pub fn arrow_len(&self) -> i32 {
        let split: Vec<&str> = self.get_cmd().split(" ").collect();
        match split.last() {
            Some(word) => return word.len() as i32,
            None => return 0,
        }
    }
}

pub fn get_tip(tip: Tip) -> String {
    let mut arrow = String::new();
    for _ in 0..tip.arrow_len() {
        arrow.push('^');
    }
    let blue_line = "|".bright_blue();
    format!(
        r#"
    {}
    {}   {} {}
    {}              {}
    {}              {}
    "#,
        blue_line,
        blue_line,
        "surtur".yellow(),
        tip.get_cmd(),
        blue_line,
        arrow.yellow(),
        blue_line,
        tip.literal()
    )
}

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
