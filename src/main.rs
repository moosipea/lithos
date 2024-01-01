use std::env;
use std::fs::read_to_string;
use std::process::ExitCode;

use rust_lisp_parser::ast::Tree;
use rust_lisp_parser::lexer::lex;
use rust_lisp_parser::lexer::Symbol;
use rust_lisp_parser::simulator::Ast;

use anyhow::Result;
use rust_lisp_parser::Error;

fn main() -> Result<ExitCode> {
    let mut args = env::args();
    let path = args.nth(1).ok_or(Error::UnexpectedArgN(2, 1))?;
    let content = read_to_string(path)?;

    let tokens = lex(&content)?;
    let tree = Tree::try_construct(&tokens)?;

    match tree {
        Tree::Branch(children) => {
            // Probably too much collecting righte?
            let bytecode = children
                .iter()
                .map(Ast::from_tree)
                .collect::<Result<Vec<_>>>()?
                .iter()
                .map(Ast::generate)
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            for instruction in &bytecode {
                println!("{instruction:?}");
            }

            rust_lisp_parser::run(bytecode)?;

            Ok(ExitCode::from(0)) // TODO
        }
        Tree::Leaf(leaf) => match leaf {
            Symbol::Number(n) => Ok(ExitCode::from(*n as u8)),
            _ => unimplemented!(),
        },
    }
}
