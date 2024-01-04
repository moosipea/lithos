use std::cmp::Ordering;

use crate::{codegen::Program, Error};
use anyhow::Result;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub enum Value {
    U64(u64),
    Function(Vec<Instruction>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::U64(n) => write!(f, "{}", *n),
            Value::Function(_) => write!(f, "(func)"),
        }
    }
}

impl std::cmp::Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Value::U64(lhs) => match other {
                Value::U64(rhs) => lhs.cmp(rhs),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}

macro_rules! impl_value_op {
    ($f:ident, $op:tt) => {
        fn $f(self, rhs: Self) -> Self {
            match self {
                Self::U64(lhs) => match rhs {
                    Self::U64(rhs) => Self::U64(lhs $op rhs),
                    _ => unimplemented!()
                },
                _ => unimplemented!()
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
            _ => true,
        }
    }

    fn is_falsey(self) -> bool {
        !self.is_truthy()
    }

    fn as_u64(self) -> Result<u64> {
        match self {
            Value::U64(n) => Ok(n),
            _ => Err(Error::UnexpectedType("U64", "FunctionA").into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Instruction {
    Push(Value),
    Operator(Op, usize),
    Dup,
    Drop,
    Swap,
    Over,
    Rot,
}

type Stack<T> = Vec<T>;
#[derive(Debug)]
pub struct Interperter<'a> {
    code: &'a [Instruction],
    addr: usize,
    stack: Stack<Value>,
    variable_stack: Stack<(String, Value)>,
}

impl<'a> Interperter<'a> {
    fn new(code: &'a [Instruction], addr: usize) -> Self {
        Self {
            code,
            addr,
            stack: Stack::new(),
            variable_stack: Stack::new(),
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

    fn instruction(&self) -> Option<&Instruction> {
        self.code.get(self.addr)
    }

    fn popn(&mut self, n: usize) -> Result<Vec<Value>> {
        let mut accum = Vec::with_capacity(n);

        for (i, e) in (0..n).map(|_| self.stack.pop()).enumerate() {
            match e {
                Some(value) => accum.push(value),
                None => return Err(Error::UnexpectedArgN(n, i).into()),
            }
        }

        Ok(accum)
    }

    fn dup(&mut self) -> Result<()> {
        let a = self.stack.last().ok_or(Error::UnexpectedArgN(1, 0))?;
        self.stack.push(a.clone());
        Ok(())
    }

    fn drop(&mut self) -> Result<()> {
        match self.stack.pop().ok_or(Error::UnexpectedArgN(1, 0)) {
            Err(err) => Err(err.into()),
            _ => Ok(()),
        }
    }

    fn swap(&mut self) -> Result<()> {
        let xs = self.popn(2)?;
        self.stack.push(xs[1].clone());
        self.stack.push(xs[0].clone());
        Ok(())
    }

    fn over(&mut self) -> Result<()> {
        // FIXME
        let a = self
            .stack
            .get(self.stack.len() - 2)
            .ok_or(Error::UnexpectedArgN(2, 1))?; // penultimate value
        self.stack.push(a.clone());
        Ok(())
    }

    fn rot(&mut self) -> Result<()> {
        let xs = self.popn(3)?;
        self.stack.push(xs[2].clone());
        self.stack.push(xs[0].clone());
        self.stack.push(xs[2].clone());
        Ok(())
    }
}

// See: https://en.wikipedia.org/wiki/Stack-oriented_programming#Stack_effect_diagrams
// See: https://en.wikipedia.org/wiki/Stack-oriented_programming#PostScript_stacks

pub fn run(program: &Program) -> Result<()> {
    use Instruction::*;
    let mut ctx = Interperter::new(&program.code, program.entrypoint);

    while let Some(instruction) = ctx.instruction() {
        match instruction {
            Dup => ctx.dup()?,
            Drop => ctx.drop()?,
            Swap => ctx.swap()?,
            Over => ctx.over()?,
            Rot => ctx.rot()?,
            _ => todo!(),
        }
    }
    Ok(())
}
