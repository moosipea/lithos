use anyhow::Result;
use rust_lisp_parser::{
    ast::{make_tree, Tree},
    lexer::lex,
};
use std::process::ExitCode;

fn main() -> Result<ExitCode> {
    let src = "(add 34 35)";
    let tokens = lex(src)?;
    let tree = Tree::try_construct(&tokens)?;
    let ast = make_tree(&tree)?;

    Ok(ExitCode::SUCCESS)
}
