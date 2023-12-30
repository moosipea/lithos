use std::error::Error;

use crate::ast::Tree;
use crate::lexer::Symbol;
// use crate::lexer::Token;

#[derive(Debug)]
pub enum AstToken<'a> {
    NumberLiteral(i32),
    Call {
        name: &'a str,
        args: Vec<AstToken<'a>>,
    },
}

impl<'a> AstToken<'a> {
    pub fn from_tree(tree: &'a Tree) -> Self {
        match make_call(tree) {
            Ok(call) => call,
            Err(_) => {
                make_number(tree).expect("Expected to construct number literal")
            }
        }
    }

    pub fn eval(self) -> i32 {
        match self {
            AstToken::NumberLiteral(n) => n,
            AstToken::Call { name, args } => {
                if args.is_empty() {
                    panic!("Void function not supported");
                }

                let evaluated_args = args.into_iter().map(AstToken::eval);

                match name {
                    "+" => evaluated_args
                        .reduce(|xs, x| xs + x)
                        .expect("Operation failed"),
                    "-" => evaluated_args
                        .reduce(|xs, x| xs - x)
                        .expect("Operation failed"),
                    "*" => evaluated_args
                        .reduce(|xs, x| xs * x)
                        .expect("Operation failed"),
                    "/" => evaluated_args
                        .reduce(|xs, x| xs / x)
                        .expect("Operation failed"),
                    "echo" => {
                        for arg in evaluated_args {
                            print!("{arg} ");
                        }
                        0
                    }
                    _ => panic!("Unknown function {name}"),
                }
            }
        }
    }
}

fn make_number<'a>(tree: &'a Tree) -> Result<AstToken<'a>, Box<dyn Error>> {
    match tree {
        Tree::Leaf(leaf) => match leaf {
            Symbol::Number(n) => Ok(AstToken::NumberLiteral(*n)),
            _ => Err("Expected Symbol::Number".into()),
        },
        _ => Err("Expected Tree::Leaf".into()),
    }
}

fn make_call<'a>(tree: &'a Tree) -> Result<AstToken<'a>, Box<dyn Error>> {
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
        Some(rst) => rst.into_iter().map(AstToken::from_tree).collect(),
        None => Vec::new(),
    };

    Ok(AstToken::Call { name, args })
}
