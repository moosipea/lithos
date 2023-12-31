use std::env;
use std::error::Error;
use std::fs::read_to_string;
use std::process::ExitCode;

use rust_lisp_parser::ast::Tree;
use rust_lisp_parser::codegen::Ast;
use rust_lisp_parser::codegen::Context;
use rust_lisp_parser::lexer::lex;
use rust_lisp_parser::lexer::Symbol;

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let mut args = env::args();
    let path = args.nth(1).ok_or("Not enough args")?;
    let content = read_to_string(path)?;

    let tokens = lex(&content)?;
    let tree = Tree::try_construct(&tokens)?;

    let mut ctx = Context::default();

    match tree {
        Tree::Branch(children) => {
            let values = children.iter().map(Ast::from_tree).map(|e| e.eval(&mut ctx));
            use rust_lisp_parser::codegen::Value;
            Ok(ExitCode::from(
                match values.last().ok_or("Evaluation failed")? {
                    Value::Signed32(n) => n as u8,
                    _ => 1u8 // TODO
                }
            ))
        }
        Tree::Leaf(leaf) => match leaf {
            Symbol::Number(n) => Ok(ExitCode::from(*n as u8)),
            _ => unimplemented!(),
        },
    }
}
