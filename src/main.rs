use std::process::ExitCode;
use anyhow::Result;
use rust_lisp_parser::simulator::{Instruction, run, Value, Op};

fn main() -> Result<ExitCode> {
    use Instruction::*;
    use Value::*;
    let code = &[
        Push(U64(34)),
        Push(U64(35)),
        Operator(Op::Add, 2),
        Push(U64(420)),
        Operator(Op::Mul, 2)
    ];
    run(code, 0)?;
    Ok(ExitCode::SUCCESS)
}
