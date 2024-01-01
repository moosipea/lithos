use crate::ast::Tree;
use crate::lexer::Symbol;
use crate::Error;

use std::collections::HashMap;
use std::fmt::Display;

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
    String(String),
    List(Vec<Value>),
    Ident(String),
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
            // TODO: clean up
            Self::List(elements) => write!(
                f,
                "({})",
                elements.iter().map(|e| format!("{e} ")).collect::<String>()
            ),
            Self::Ident(ident) => write!(f, "{ident}"),
        }
    }
}

fn arithmetic_op<'a, F: FnMut(i32, i32) -> i32>(
    args: Vec<Ast<'a>>,
    ctx: &mut Context<'a>,
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

fn builtin_echo<'a>(args: impl Iterator<Item = Ast<'a>>, ctx: &mut Context<'a>) -> Result<Value> {
    for arg in args {
        let arg = arg.eval(ctx)?;
        print!("{} ", arg);
    }
    print!("\n");
    Ok(Value::Signed32(0))
}

fn builtin_if_else<'a>(
    mut args: impl Iterator<Item = Ast<'a>>,
    ctx: &mut Context<'a>,
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
    ctx: &mut Context<'a>,
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

pub struct FunctionDefinition<'a> {
    ast: Ast<'a>,
    args: Vec<String>,
}

fn builtin_define_function<'a>(
    mut args: impl Iterator<Item = Ast<'a>>,
    ctx: &mut Context<'a>,
) -> Result<Value> {
    let name = match args.next().ok_or(Error::UnexpectedArgN(3, 0))? {
        Ast::Identifier(ident) => ident,
        _ => return Err(Error::Expected("Ast::Identifier").into()),
    };

    let fn_args = match args.next().ok_or(Error::UnexpectedArgN(3, 1))?.eval(ctx)? {
        Value::List(list) => list
            .iter()
            .cloned()
            .map(|e| match e {
                Value::Ident(ident) => Some(ident),
                _ => None,
            })
            .collect::<Option<Vec<_>>>(),
        _ => return Err(Error::Expected("Value::List").into()),
    }
    .ok_or(Error::Expected("all arguments to be identifiers"))?;

    let body = args.next().ok_or(Error::UnexpectedArgN(3, 2))?;

    ctx.functions.insert(
        name.clone(),
        FunctionDefinition {
            ast: body,
            args: fn_args,
        },
    );

    Ok(Value::String(name))
}

fn builtin_make_list<'a>(
    args: impl Iterator<Item = Ast<'a>>,
    ctx: &mut Context<'a>,
) -> Result<Value> {
    let values: Result<_> = args.map(|a| a.eval(ctx)).collect();
    Ok(Value::List(values?))
}

fn bultin_equality<'a>(
    args: impl Iterator<Item = Ast<'a>>,
    ctx: &mut Context<'a>,
) -> Result<Value> {
    // FIXME
    let evaluated: Result<Vec<_>> = args.map(|arg| arg.eval(ctx)).collect();
    let numbers = evaluated?
        .into_iter()
        .map(|value| match value {
            Value::Signed32(n) => Some(n),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()
        .ok_or(Error::Expected("all arguments to be Signed32"))?;

    if numbers.len() < 2 {
        return Err(Error::Expected("at least 2 arguments").into());
    }

    // FIXME
    Ok(
        match numbers.windows(2).map(|xs| xs[0] == xs[1]).all(|x| x) {
            true => Value::Signed32(1),
            false => Value::Signed32(0),
        },
    )
}

fn call_user_function<'a>(
    name: &str,
    args: impl Iterator<Item = Ast<'a>>,
    ctx: &mut Context<'a>,
) -> Result<Value> {
    let args: Vec<_> = args.collect();
    match ctx.functions.get(name) {
        Some(func) => {
            let n_args = args.len();
            let n_expected = func.args.len();
            if n_args != n_expected {
                return Err(Error::UnexpectedArgN(n_expected, n_args).into());
            }

            let function = func.ast.clone();
            let prev_state = ctx.scope_variables.clone();

            for (arg, ast) in func.args.clone().into_iter().zip(args) {
                let value = ast.eval(ctx)?;
                ctx.scope_variables.insert(arg, value);
            }

            let ret = function.eval(ctx);
            ctx.scope_variables = prev_state;

            ret
        }
        None => Err(Error::UnknownFunction(name.to_string()).into()),
    }
}

#[derive(Default)]
pub struct Context<'a> {
    functions: HashMap<String, FunctionDefinition<'a>>,
    scope_variables: HashMap<String, Value>,
}

impl<'a> Ast<'a> {
    pub fn from_tree(tree: &'a Tree) -> Result<Self> {
        match tree {
            Tree::Branch(_) => ast_from_branch(tree),
            Tree::Leaf(_) => ast_from_leaf(tree),
        }
    }

    pub fn eval(self, ctx: &mut Context<'a>) -> Result<Value> {
        match self {
            Ast::NumberLiteral(n) => Ok(Value::Signed32(n)),
            Ast::StringLiteral(s) => Ok(Value::String(s)),
            // FIXME
            Ast::Identifier(ident) => Ok(match ctx.scope_variables.get(&ident) {
                Some(value) => value.clone(),
                None => Value::Ident(ident),
            }),
            Ast::Call { name, args } => {
                if args.is_empty() {
                    return Err(Error::Unimplemented("functions with no arguments").into());
                }
                match name {
                    "echo" => builtin_echo(args.into_iter(), ctx),
                    "if-else" => builtin_if_else(args.into_iter(), ctx),
                    "let" => builtin_define_variable(args.into_iter(), ctx),
                    "fn" => builtin_define_function(args.into_iter(), ctx),
                    "_" => builtin_make_list(args.into_iter(), ctx),
                    "+" => arithmetic_op(args, ctx, i32::wrapping_add),
                    "-" => arithmetic_op(args, ctx, i32::wrapping_sub),
                    "*" => arithmetic_op(args, ctx, i32::wrapping_mul),
                    "/" => arithmetic_op(args, ctx, i32::wrapping_div),
                    "=" => bultin_equality(args.into_iter(), ctx),
                    _ => call_user_function(name, args.into_iter(), ctx),
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
