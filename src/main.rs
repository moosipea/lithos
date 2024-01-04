use std::process::ExitCode;
use anyhow::Result;
use rust_lisp_parser::simulator::{Instruction, run, Value, Op};

fn main() -> Result<ExitCode> {
    use Instruction::*;
    use Value::*;

    // Goal: function that squares a number
    let code = &[
        Push(U64(5)), // argument
        Push(U64(5)), // function address
        Call,
        Dump,
        Halt,

        // begin function
        Dup,
        Operator(Op::Mul, 2),
        Ret,
    ];
    run(code, 0, false)?;
    Ok(ExitCode::SUCCESS)
}
