use std::{fs::File, io::Read};

use rslua::{
    ast::{Assignable, Block, Expr, Field, Stat},
    ast_walker::AstVisitor,
    lexer::Lexer,
    parser::Parser,
    tokens::Token,
};

pub struct LuaSource {
    path: String,
    ast: Block,
}

pub struct ProjectHandler {
    source: LuaSource,
    // TODO: Might want to change this to custom type for handling dependencies
    tool_version: f32,
    proj_name: String,
    proj_version: f32,
    deps: Vec<String>,
}

impl ProjectHandler {
    pub fn new(cfg_file_path: &str) -> Self {
        let source = LuaSource::new(cfg_file_path);
        ProjectHandler {
            source,
            tool_version: 0.0,
            proj_name: String::from("sus"),
            proj_version: 0.0,
            deps: Vec::new(),
        }
    }

    pub fn get_deps(&self) -> Vec<String> {
        let stats = &self.source.ast.stats;
        for stat in stats {
            match stat {
                Stat::IfStat(_) => todo!(),
                Stat::WhileStat(_) => todo!(),
                Stat::DoBlock(_) => todo!(),
                Stat::ForStat(_) => todo!(),
                Stat::RepeatStat(_) => todo!(),
                Stat::FuncStat(_) => todo!(),
                Stat::LocalStat(local) => match local.exprs.as_ref().unwrap().exprs.get(0).unwrap()
                {
                    Expr::Nil(_) => todo!(),
                    Expr::True(_) => todo!(),
                    Expr::False(_) => todo!(),
                    Expr::VarArg(_) => todo!(),
                    Expr::Float(_) => todo!(),
                    Expr::Int(_) => todo!(),
                    Expr::String(_) => todo!(),
                    Expr::Name(_) => todo!(),
                    Expr::ParenExpr(_) => todo!(),
                    Expr::FuncBody(_) => todo!(),
                    Expr::Table(table) => {
                        for field in &table.fields {
                            match field {
                                Field::RecField(_) => todo!(),
                                Field::ListField(list_field) => match list_field.value {
                                    Expr::Nil(_) => todo!(),
                                    Expr::True(_) => todo!(),
                                    Expr::False(_) => todo!(),
                                    Expr::VarArg(_) => todo!(),
                                    Expr::Float(_) => todo!(),
                                    Expr::Int(_) => todo!(),
                                    Expr::String(_) => todo!(),
                                    Expr::Name(_) => todo!(),
                                    Expr::ParenExpr(_) => todo!(),
                                    Expr::FuncBody(_) => todo!(),
                                    Expr::Table(_) => todo!(),
                                    Expr::BinExpr(_) => todo!(),
                                    Expr::UnExpr(_) => todo!(),
                                    Expr::SuffixedExpr(_) => todo!(),
                                },
                            }
                        }
                    }
                    Expr::BinExpr(_) => todo!(),
                    Expr::UnExpr(_) => todo!(),
                    Expr::SuffixedExpr(_) => todo!(),
                },
                Stat::LabelStat(_) => todo!(),
                Stat::RetStat(_) => todo!(),
                Stat::BreakStat(_) => todo!(),
                Stat::GotoStat(_) => todo!(),
                Stat::AssignStat(assingment) => match assingment.left.assignables.get(0).unwrap() {
                    Assignable::Name(name) => todo!("joe"),
                    Assignable::SuffixedExpr(_) => todo!(),
                },
                Stat::CallStat(_) => todo!(),
            }
        }
        todo!()
    }
}

impl LuaSource {
    fn new(cfg_file_path: &str) -> Self {
        // File
        let mut file = File::open(cfg_file_path).expect("Failed to open file");
        let mut source = String::new();
        file.read_to_string(&mut source)
            .expect("Failed to read file");

        // Code
        let mut lexer = Lexer::default();
        let tokens = lexer.run(&source).expect("Failed to tokenize code");

        let mut parser = Parser::default();
        let ast = parser.run(tokens).expect("Failed to construct ast");

        LuaSource {
            path: cfg_file_path.to_string(),
            ast,
        }
    }
}
