use std::cmp::Ordering;

use crate::{debugger::Debugger, Error, codegen::Program};
use anyhow::Result;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub enum Value {
    U64(u64),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::U64(n) => write!(f, "{}", *n),
        }
    }
}

impl std::cmp::Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Value::U64(lhs) => match other {
                Value::U64(rhs) => lhs.cmp(rhs),
            },
        }
    }
}

macro_rules! impl_value_op {
    ($f:ident, $op:tt) => {
        fn $f(self, rhs: Self) -> Self {
            match self {
                Self::U64(lhs) => match rhs {
                    Self::U64(rhs) => Self::U64(lhs $op rhs)
                }
            }
        }
    };
}

impl Value {
    impl_value_op!(add, +);
    impl_value_op!(sub, -);
    impl_value_op!(mul, *);
    impl_value_op!(div, /);

    fn is_truthy(self) -> bool {
        match self {
            Value::U64(n) => n != 0,
        }
    }

    fn is_falsey(self) -> bool {
        !self.is_truthy()
    }

    fn as_u64(self) -> Result<u64> {
        match self {
            Value::U64(n) => Ok(n),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn compute(&self, args: Box<[Value]>) -> Result<Value> {
        let f = match self {
            Op::Add => Value::add,
            Op::Sub => Value::sub,
            Op::Mul => Value::mul,
            Op::Div => Value::div,
        };
        args.into_iter()
            .cloned()
            .rev() // Keep?
            .reduce(|a, b| f(a, b))
            .ok_or(Error::Expected("reduction not to fail").into())
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Push(Value),
    Operator(Op, usize),
    Call,
    Ret,
    Jump,
    JumpTruthy,
    JumpFalsy,
    Dump,
    Dup,
    Halt,
}

type Stack = Vec<Value>;
#[derive(Debug)]
pub struct Interperter<'a> {
    code: &'a [Instruction],
    addr: usize,
    stack: Stack,
    return_reg: Option<Value>,
}

impl<'a> Interperter<'a> {
    fn new(code: &'a [Instruction], addr: usize) -> Self {
        Self {
            code,
            addr,
            stack: Stack::new(),
            return_reg: None,
        }
    }

    // TODO: allow field access?
    pub fn code(&self) -> &[Instruction] {
        self.code
    }

    // TODO: allow field access?
    pub fn addr(&self) -> usize {
        self.addr
    }

    // TODO: allow field access?
    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    fn get(&mut self) -> Option<Instruction> {
        let ret = self.code.get(self.addr).cloned();
        self.addr += 1;
        ret
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn jump(&mut self) -> Result<()> {
        let addr = self.pop()?.as_u64()? as usize;

        if addr >= self.code.len() {
            return Err(Error::OutOfBoundsJump.into());
        }

        self.addr = addr;

        Ok(())
    }

    fn popn(&mut self, n: usize) -> Result<Box<[Value]>> {
        let mut accum = Vec::with_capacity(n);

        for (i, e) in (0..n).map(|_| self.stack.pop()).enumerate() {
            match e {
                Some(value) => accum.push(value),
                None => return Err(Error::UnexpectedArgN(n, i).into()),
            }
        }

        Ok(accum.into_boxed_slice())
    }

    fn pop(&mut self) -> Result<Value> {
        self.stack.pop().ok_or(Error::UnexpectedArgN(1, 0).into())
    }

    fn dup(&mut self) -> Result<()> {
        let value = self.stack.last().ok_or(Error::EmptyStack)?;
        self.push(value.clone());
        Ok(())
    }
}

pub fn run(program: &Program, debug: bool) -> Result<()> {
    let mut ctx = Interperter::new(&program.code, program.entrypoint);
    let mut debugger = Debugger::new(1000)?;

    if debug {
        debugger.start()?;
    }

    while let Some(instruction) = ctx.get() {
        if debug {
            debugger.show(&ctx)?;
        }
        match instruction {
            Instruction::Push(value) => ctx.push(value.clone()),
            Instruction::Operator(operator, argc) => {
                let value = operator.compute(ctx.popn(argc)?)?;
                ctx.push(value);
            }
            Instruction::Call => {
                ctx.return_reg = Some(Value::U64(ctx.addr as u64));
                ctx.jump()?;
            }
            Instruction::Ret => {
                let addr = ctx.return_reg.ok_or(Error::Expected("return address"))?;
                ctx.push(addr);
                ctx.jump()?;
            }
            Instruction::Jump => ctx.jump()?,
            Instruction::JumpTruthy => {
                if ctx.pop()?.is_truthy() {
                    ctx.jump()?
                }
            }
            Instruction::JumpFalsy => {
                if ctx.pop()?.is_falsey() {
                    ctx.jump()?
                }
            }
            Instruction::Dump => println!("{}", ctx.pop()?),
            Instruction::Dup => ctx.dup()?,
            Instruction::Halt => break,
        }
        if debug {
            debugger.timeout();
        }
    }
    Ok(())
}
