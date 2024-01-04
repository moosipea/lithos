use crate::{
    ast::Ast,
    simulator::{Instruction, Op, Value},
    Error,
};
use anyhow::Result;
use std::collections::HashMap;

enum MaybeSolved<'a> {
    Solved(Instruction),
    UnresolvedCall(&'a str),
}

struct Function {
    // maybe store some metadata
    instructions: Box<[Instruction]>,
}

pub struct Program {
    pub code: Box<[Instruction]>,
    pub entrypoint: usize,
}

struct Compiler {
    macros: HashMap<String, ()>, // TODO
    functions: HashMap<String, Function>,
}

impl Compiler {
    fn new() -> Self {
        Self {
            macros: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn def_func(&mut self, name: &str, instructions: Box<[Instruction]>) {
        self.functions
            .insert(name.to_string(), Function { instructions });
    }

    fn resolve_functions(&self) -> (HashMap<String, usize>, Vec<Instruction>) {
        let mut addresses = HashMap::new();
        let mut code = Vec::new();

        for (name, instructions) in &self.functions {
            addresses.insert(name.to_string(), code.len());
            code.extend(instructions.instructions.clone().into_vec());
        }

        (addresses, code)
    }

    fn finish(self, code: Vec<MaybeSolved>) -> Result<Program> {
        let mut final_code = Vec::new();

        let (func_addresses, funcs) = self.resolve_functions();
        final_code.extend(funcs);

        for instruction in code {
            match instruction {
                MaybeSolved::Solved(instruction) => final_code.push(instruction),
                MaybeSolved::UnresolvedCall(name) => {
                    let addr = self.resolve_call(name, &func_addresses)?;
                    final_code.push(Instruction::Push(Value::U64(addr as u64)));
                    final_code.push(Instruction::Call);
                }
            }
        }

        // FIXME
        let entrypoint = self.resolve_call("main", &func_addresses)?;

        Ok(Program {
            code: final_code.into_boxed_slice(),
            entrypoint,
        })
    }

    fn resolve_call(&self, name: &str, addresses: &HashMap<String, usize>) -> Result<usize> {
        addresses
            .get(name)
            .cloned()
            .ok_or(Error::UnknownFunction(name.to_string()).into())
    }

    fn is_macro(&self, name: &str) -> bool {
        self.macros.contains_key(name)
    }

    fn is_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }
}

fn generate<'a>(ast: &'a Ast, compiler: &Compiler) -> Result<Vec<MaybeSolved<'a>>> {
    use MaybeSolved::*;
    let mut bytecode = Vec::new();
    match ast {
        Ast::NumberLiteral(n) => bytecode.push(Solved(Instruction::Push(Value::U64(*n)))),
        Ast::Call { name, args } => {
            if compiler.is_macro(name) {
                todo!()
            } else if compiler.is_function(name) {
                // todo: reverse order?
                for arg in args.into_iter().rev() {
                    bytecode.extend(generate(arg, compiler)?);
                }
                bytecode.push(UnresolvedCall(name));
            } else {
                return Err(Error::Unimplemented("this").into());
            }
        }
        _ => panic!("not yet implemented: {ast:?}"),
    }
    Ok(bytecode)
}

pub fn compile(asts: &[Ast]) -> Result<Program> {
    let mut compiler = Compiler::new();

    compiler.def_func(
        "add",
        vec![Instruction::Operator(Op::Add, 2), Instruction::Ret].into_boxed_slice(),
    );

    // Todo: declarative
    let mut code = Vec::new();
    for chunk in asts.into_iter().map(|ast| generate(ast, &compiler)) {
        code.extend(chunk?);
    }

    compiler.finish(code)
}
