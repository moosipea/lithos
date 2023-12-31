use crate::ast::Tree;
use crate::lexer::Symbol;
use crate::Error;
use anyhow::Result;
use std::fmt::Display;

#[derive(Debug)]
pub enum Ast<'a> {
    NumberLiteral(i32),
    StringLiteral(String),
    Identifier(String),
    Call { name: &'a str, args: Vec<Ast<'a>> },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Signed32(i32),
    String(String),
}

impl Value {
    fn is_true(&self) -> bool {
        match self {
            Self::Signed32(n) => *n != 0,
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Signed32(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
        }
    }
}

fn arithmetic_op<F: FnMut(i32, i32) -> i32>(
    args: Vec<Ast>,
    ctx: &mut Context,
    op: F,
) -> Result<Value> {
    let values = args.into_iter().map(|a| a.eval(ctx)).collect::<Vec<_>>();
    let mut numbers = Vec::new();

    for v in values {
        match v? {
            Value::Signed32(n) => numbers.push(n),
            _ => return Err(Error::Expected("Value::Signed32").into()),
        }
    }

    Ok(Value::Signed32(
        numbers
            .into_iter()
            .reduce(op)
            .ok_or(Error::Expected("to reduce list"))?,
    ))
}

fn builtin_echo<'a>(args: impl Iterator<Item = Ast<'a>>, ctx: &mut Context) -> Result<Value> {
    for arg in args {
        let arg = arg.eval(ctx)?;
        print!("{} ", arg);
    }
    print!("\n");
    Ok(Value::Signed32(0))
}

fn builtin_if_else<'a>(
    mut args: impl Iterator<Item = Ast<'a>>,
    ctx: &mut Context,
) -> Result<Value> {
    let cond = args.next().ok_or(Error::UnexpectedArgN(3, 0))?.eval(ctx)?;
    let if_true = args.next().ok_or(Error::UnexpectedArgN(3, 1))?;
    let if_false = args.next().ok_or(Error::UnexpectedArgN(3, 2))?;

    if cond.is_true() {
        if_true.eval(ctx)
    } else {
        if_false.eval(ctx)
    }
}

fn builtin_define_variable<'a>(
    mut args: impl Iterator<Item = Ast<'a>>,
    ctx: &mut Context,
) -> Result<Value> {
    let ident = match args.next().ok_or(Error::UnexpectedArgN(3, 0))? {
        Ast::Identifier(i) => i,
        _ => return Err(Error::Expected("Ast::Identifier").into()),
    };
    let value = args.next().ok_or(Error::UnexpectedArgN(3, 1))?.eval(ctx)?;
    ctx.scope_variables.insert(ident.clone(), value.clone());
    let ret = args.next().ok_or(Error::UnexpectedArgN(3, 2))?.eval(ctx);
    ctx.scope_variables.remove(&ident);
    ret
}

struct Function {}

use std::collections::HashMap;

#[derive(Default)]
pub struct Context {
    _functions: HashMap<String, Function>,
    scope_variables: HashMap<String, Value>,
}

impl<'a> Ast<'a> {
    pub fn from_tree(tree: &'a Tree) -> Result<Self> {
        match tree {
            Tree::Branch(_) => ast_from_branch(tree),
            Tree::Leaf(_) => ast_from_leaf(tree),
        }
    }

    pub fn eval(self, ctx: &mut Context) -> Result<Value> {
        match self {
            Ast::NumberLiteral(n) => Ok(Value::Signed32(n)),
            Ast::StringLiteral(s) => Ok(Value::String(s)),
            Ast::Identifier(ident) => Ok(ctx
                .scope_variables
                .get(&ident)
                .expect("Unknown Identifier")
                .clone()),
            Ast::Call { name, args } => {
                if args.is_empty() {
                    return Err(Error::Unimplemented("void functions").into());
                }
                match name {
                    "echo" => builtin_echo(args.into_iter(), ctx),
                    "if-else" => builtin_if_else(args.into_iter(), ctx),
                    "let" => builtin_define_variable(args.into_iter(), ctx),
                    "+" => arithmetic_op(args, ctx, i32::wrapping_add),
                    "-" => arithmetic_op(args, ctx, i32::wrapping_sub),
                    "*" => arithmetic_op(args, ctx, i32::wrapping_mul),
                    "/" => arithmetic_op(args, ctx, i32::wrapping_div),
                    _ => Err(Error::UnknownFunction(name.to_string()).into()),
                }
            }
        }
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
