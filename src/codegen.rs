use std::error::Error;
use std::fmt::Display;

use crate::ast::Tree;
use crate::lexer::Symbol;

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
) -> Option<Value> {
    let values = args.into_iter().map(|a| a.eval(ctx)).collect::<Vec<_>>();
    let mut numbers = Vec::new();

    for v in values {
        match v {
            Value::Signed32(n) => numbers.push(n),
            _ => return None,
        }
    }

    Some(Value::Signed32(numbers.into_iter().reduce(op)?))
}

fn builtin_echo<'a>(args: impl Iterator<Item = Ast<'a>>, ctx: &mut Context) -> Value {
    for arg in args.map(|arg| arg.eval(ctx)) {
        print!("{} ", arg);
    }
    print!("\n");
    Value::Signed32(0)
}

fn builtin_if_else<'a>(args: impl Iterator<Item = Ast<'a>>, ctx: &mut Context) -> Value {
    // TODO: Clean up
    let mut iargs = args.into_iter();
    let cond = iargs.next().unwrap().eval(ctx);
    let if_true = iargs.next().unwrap();
    let if_false = iargs.next().unwrap();

    if cond.is_true() {
        if_true.eval(ctx)
    } else {
        if_false.eval(ctx)
    }
}

// (let <ident> <expr> <rst>)
// where rst is the expr where <ident> is a valid var
fn builtin_define_variable<'a>(
    mut args: impl Iterator<Item = Ast<'a>>,
    ctx: &mut Context,
) -> Value {
    let ident = match args.next().unwrap() {
        Ast::Identifier(i) => i,
        _ => panic!("Expected identifier"),
    };
    let value = args.next().unwrap().eval(ctx);
    ctx.scope_variables.insert(ident.clone(), value.clone());
    let ret = args.next().unwrap().eval(ctx);
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
    pub fn from_tree(tree: &'a Tree) -> Self {
        match tree {
            Tree::Branch(_) => ast_from_branch(tree).unwrap(),
            Tree::Leaf(_) => ast_from_leaf(tree).unwrap(),
        }
    }

    pub fn eval(self, ctx: &mut Context) -> Value {
        match self {
            Ast::NumberLiteral(n) => Value::Signed32(n),
            Ast::StringLiteral(s) => Value::String(s),
            Ast::Identifier(ident) => ctx
                .scope_variables
                .get(&ident)
                .expect("Unknown Identifier")
                .clone(),
            Ast::Call { name, args } => {
                if args.is_empty() {
                    panic!("Void function not supported");
                }
                match name {
                    "echo" => builtin_echo(args.into_iter(), ctx),
                    "if-else" => builtin_if_else(args.into_iter(), ctx),
                    "let" => builtin_define_variable(args.into_iter(), ctx),
                    "+" => arithmetic_op(args, ctx, i32::wrapping_add).expect("Operation failed"),
                    "-" => arithmetic_op(args, ctx, i32::wrapping_sub).expect("Operation failed"),
                    "*" => arithmetic_op(args, ctx, i32::wrapping_mul).expect("Operation failed"),
                    "/" => arithmetic_op(args, ctx, i32::wrapping_div).expect("Operation failed"),
                    _ => panic!("Unknown function {name}"),
                }
            }
        }
    }
}

fn ast_from_leaf<'a>(tree: &'a Tree) -> Result<Ast<'a>, Box<dyn Error>> {
    match tree {
        Tree::Leaf(leaf) => match leaf {
            Symbol::Number(n) => Ok(Ast::NumberLiteral(*n)),
            Symbol::StringLiteral(s) => Ok(Ast::StringLiteral(s.to_string())),
            Symbol::Ident(ident) => Ok(Ast::Identifier(ident.to_string())),
        },
        _ => Err("Expected Tree::Leaf".into()),
    }
}

fn ast_from_branch<'a>(tree: &'a Tree) -> Result<Ast<'a>, Box<dyn Error>> {
    let branch = tree
        .branch()
        .ok_or(format!("Expected Tree::Branch, got {tree:?}"))?;

    let name = match branch
        .first()
        .ok_or("Empty branch")?
        .leaf()
        .ok_or("Expected Tree::Leaf")?
    {
        Symbol::Ident(s) => *s,
        _ => return Err("Expected Symbol::Ident".into()),
    };

    let args = match branch.get(1..) {
        Some(rst) => rst.into_iter().map(Ast::from_tree).collect(),
        None => Vec::new(),
    };

    Ok(Ast::Call { name, args })
}
