use std::process::ExitCode;
use anyhow::Result;
use rust_lisp_parser::simulator::{Instruction, run, Value, Op};

fn main() -> Result<ExitCode> {
    use Instruction::*;
    use Value::*;
    // let code = &[
    //     Push(U64(0)),
    //     Push(U64(1)),
    //     Operator(Op::Add, 2),
    //     Dup,
    //     Dump,
    //     Dup,
    //     Push(U64(100)),
    //     Comp,
    //     Dup,
    //     JumpCond(2, true),
    //     Halt,
    // ];

    // Goal: factorial function
    let code = &[
        // square(a) decl
        Push(U64(1)),
        Dup, // stack ends with [...; 5; 5]

        Operator(Op::Mul, 2),

        // Retrieve return address and jump back
        Push(U64(2)),
        Dup,
        Jump,
        // End of function

        // Entrypoint
        Push(U64(10)), // return address (Dump instruction)
        Push(U64(5)), // input
        Push(U64(0)), // function address
        Jump,

        // Dump and exit
        Dump,
        Halt,
    ];
    run(code, 6)?;
    Ok(ExitCode::SUCCESS)
}
