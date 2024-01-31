#[macro_export]
macro_rules! subcommand {
    ($name:expr, $about:expr, $arg:expr) => {{
        use clap::{value_parser, ArgAction, Command};
        use std::path::PathBuf;

        Command::new($name)
            .arg(
                $arg.value_parser(value_parser!(PathBuf))
                    .action(ArgAction::Append),
            )
            .about($about)
    }};
}
