use std::process::ExitCode;
use anyhow::Result;
use rust_lisp_parser::simulator::{Instruction, run, Value, Op};

fn main() -> Result<ExitCode> {
    use Instruction::*;
    use Value::*;
    let code = &[
        Push(U64(0)),
        Push(U64(1)),
        Operator(Op::Add, 2),
        Dup,
        Dump,
        Dup,
        Push(U64(100)),
        Comp,
        Dup,
        JumpCond(2, true),
        Halt,
    ];
    run(code, 0)?;
    Ok(ExitCode::SUCCESS)
}
