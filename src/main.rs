use anyhow::Result;
use rust_lisp_parser::{
    ast::{Tree, make_tree},
    codegen::compile,
    lexer::lex, simulator::run,
};
use std::process::ExitCode;

fn main() -> Result<ExitCode> {
    let src = "(add 34 35)";
    let tokens = lex(src)?;
    let tree = Tree::try_construct(&tokens)?;
    let ast = make_tree(&tree)?;
    let program = compile(&ast)?;
    run(&program, false)?;

    Ok(ExitCode::SUCCESS)
}
