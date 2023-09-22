mod proj_cfg;

use std::{fs::File, io::Read};
use proj_cfg::ProjectHandler;
use rslua::{parser::Parser, lexer::Lexer};

fn main() {
    let proj = ProjectHandler::new("example/project.lua");

    let deps = proj.get_deps();
}

