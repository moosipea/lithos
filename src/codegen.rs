use crate::{ast::Ast, simulator::Instruction};
use anyhow::Result;
use std::collections::HashMap;

enum MaybeSolved<'a> {
    Solved(Instruction),
    UnresolvedCall(&'a str),
}

pub struct Program {
    pub code: Box<[Instruction]>,
    pub entrypoint: usize,
}

struct Compiler {}

impl Compiler {
    fn new() -> Self {
        Self {}
    }

    fn generate(&mut self, ast: &Ast) -> Result<Vec<Instruction>> {
        todo!()
    }
}

// See: https://en.wikipedia.org/wiki/Stack-oriented_programming#Stack_effect_diagrams
// See: https://en.wikipedia.org/wiki/Stack-oriented_programming#PostScript_stacks

pub fn compile(asts: &[Ast]) -> Result<Program> {
    let mut compiler = Compiler::new();

    // Todo: declarative
    let mut code = Vec::new();
    for chunk in asts.into_iter().map(|ast| compiler.generate(ast)) {
        code.extend(chunk?);
    }

    todo!()
}
