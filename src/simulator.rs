use std::cmp::Ordering;

use crate::Error;
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
    Pop,
    Operator(Op, usize),
    Comp,
    Jump(usize),
    JumpCond(usize, bool),
    Dump,
    Dup,
    Halt,
}

type Stack = Vec<Value>;
struct Interperter<'a> {
    code: &'a [Instruction],
    addr: usize,
    stack: Stack,
}

impl<'a> Interperter<'a> {
    fn new(code: &'a [Instruction], addr: usize) -> Self {
        Self {
            code,
            addr,
            stack: Stack::new(),
        }
    }

    fn get(&mut self) -> Option<Instruction> {
        let ret = self.code.get(self.addr).cloned();
        self.addr += 1;
        ret
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
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
}

pub fn run(bytecode: &[Instruction], entry: usize) -> Result<()> {
    let mut ctx = Interperter::new(bytecode, entry);

    while let Some(instruction) = ctx.get() {
        match instruction {
            Instruction::Push(value) => ctx.push(value.clone()),
            Instruction::Pop => {
                let _ = ctx.pop()?; // FIXME
            },
            Instruction::Operator(operator, argc) => {
                let value = operator.compute(ctx.popn(argc)?)?;
                ctx.push(value);
            }
            Instruction::Comp => {
                let values = ctx.popn(2)?;
                ctx.push(Value::U64(match values[1].cmp(&values[0]) {
                    Ordering::Less => 1, // FIXME
                    Ordering::Equal => 0,
                    Ordering::Greater => 0, // FIXME
                }));
            }
            Instruction::Jump(addr) => ctx.addr = addr,
            Instruction::JumpCond(addr, cond) => {
                if ctx.pop()?.is_truthy() == cond {
                    ctx.addr = addr
                }
            }
            Instruction::Dump => println!("{}", ctx.pop()?),
            Instruction::Dup => {
                let value = ctx.pop()?;
                ctx.push(value.clone());
                ctx.push(value);
            }
            Instruction::Halt => break,
        }
    }

    Ok(())
}
