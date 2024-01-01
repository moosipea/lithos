use crate::ast::Tree;
use crate::lexer::Symbol;
use crate::Error;
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum Ast<'a> {
    NumberLiteral(i32),
    StringLiteral(String),
    Identifier(String),
    Call { name: &'a str, args: Vec<Ast<'a>> },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Signed32(i32),
    // String(String),
    // List(Vec<Value>),
    // Ident(String),
}

impl Value {
    fn signed32(&self) -> Option<i32> {
        match self {
            Value::Signed32(n) => Some(*n),
        }
    }
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

type Stack = Vec<Value>;

impl Op {
    pub fn eval(&self, stack: &mut Stack) -> Result<()> {
        let a = stack
            .pop()
            .ok_or(Error::UnexpectedArgN(2, 0))?
            .signed32()
            .ok_or(Error::Expected("number"))?;
        let b = stack
            .pop()
            .ok_or(Error::UnexpectedArgN(2, 1))?
            .signed32()
            .ok_or(Error::Expected("number"))?;

        let result = match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
        };

        println!("{result}");

        stack.push(Value::Signed32(result));

        Ok(())
    }
}

#[derive(Debug)]
pub enum Instruction {
    Load(Value),
    Operation(Op),
}

impl<'a> Ast<'a> {
    pub fn from_tree(tree: &'a Tree) -> Result<Self> {
        match tree {
            Tree::Branch(_) => ast_from_branch(tree),
            Tree::Leaf(_) => ast_from_leaf(tree),
        }
    }

    pub fn generate(&self) -> Result<Vec<Instruction>> {
        let mut instructions = Vec::new();
        match self {
            Ast::NumberLiteral(n) => instructions.push(Instruction::Load(Value::Signed32(*n))),
            Ast::Call { name, args } => {
                for arg in args.into_iter().rev() {
                    let bytecode = arg.generate()?;
                    instructions.extend(bytecode);
                }

                let op = match *name {
                    "+" => Op::Add,
                    "-" => Op::Sub,
                    "*" => Op::Mul,
                    "/" => Op::Div,
                    _ => return Err(Error::Unimplemented("operator").into()),
                };

                instructions.push(Instruction::Operation(op))
            }
            _ => return Err(Error::Unimplemented("ast variant").into()),
        }
        Ok(instructions)
    }
}

fn ast_from_leaf<'a>(tree: &'a Tree) -> Result<Ast<'a>> {
    match tree {
        Tree::Leaf(leaf) => match leaf {
            Symbol::Number(n) => Ok(Ast::NumberLiteral(*n)),
            Symbol::StringLiteral(s) => Ok(Ast::StringLiteral(s.to_string())),
            Symbol::Ident(ident) => Ok(Ast::Identifier(ident.to_string())),
        },
        _ => Err(Error::Expected("Tree::Leaf").into()),
    }
}

fn ast_from_branch<'a>(tree: &'a Tree) -> Result<Ast<'a>> {
    let branch = tree.branch().ok_or(Error::Expected("Tree::Branch"))?;

    let name = match branch
        .first()
        .ok_or(Error::Expected("nonempty branch"))?
        .leaf()
        .ok_or(Error::Expected("Tree::Leaf"))?
    {
        Symbol::Ident(s) => *s,
        _ => return Err(Error::Expected("Symbol::Ident").into()),
    };

    let args = match branch.get(1..) {
        Some(rst) => rst.into_iter().map(Ast::from_tree).collect(),
        None => Ok(Vec::new()),
    }?;

    Ok(Ast::Call { name, args })
}
