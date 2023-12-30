use std::error::Error;

use crate::ast::Tree;
use crate::lexer::Symbol;
// use crate::lexer::Token;

#[derive(Debug)]
pub enum Ast<'a> {
    NumberLiteral(i32),
    Call { name: &'a str, args: Vec<Ast<'a>> },
}

impl<'a> Ast<'a> {
    pub fn from_tree(tree: &'a Tree) -> Self {
        match make_call(tree) {
            Ok(call) => call,
            Err(_) => make_number(tree).expect("Expected to construct number literal"),
        }
    }

    pub fn eval(self) -> i32 {
        match self {
            Ast::NumberLiteral(n) => n,
            Ast::Call { name, args } => {
                if args.is_empty() {
                    panic!("Void function not supported");
                }

                match name {
                    "+" => args
                        .into_iter()
                        .map(Ast::eval)
                        .reduce(|xs, x| xs + x)
                        .expect("Operation failed"),
                    "-" => args
                        .into_iter()
                        .map(Ast::eval)
                        .reduce(|xs, x| xs - x)
                        .expect("Operation failed"),
                    "*" => args
                        .into_iter()
                        .map(Ast::eval)
                        .reduce(|xs, x| xs * x)
                        .expect("Operation failed"),
                    "/" => args
                        .into_iter()
                        .map(Ast::eval)
                        .reduce(|xs, x| xs / x)
                        .expect("Operation failed"),
                    "echo" => {
                        for arg in args.into_iter().map(Ast::eval) {
                            print!("{arg} ");
                        }
                        print!("\n");
                        0
                    }
                    "if-else" => {
                        if args.len() != 3 {
                            panic!("Expected 3 arguments, got {}", args.len());
                        }

                        // TODO: clean up
                        let mut iargs = args.into_iter();
                        let cond = iargs.next().unwrap().eval();
                        let if_true = iargs.next().unwrap();
                        let if_false = iargs.next().unwrap();

                        if cond != 0 {
                            if_true.eval()
                        } else {
                            if_false.eval()
                        }
                    }
                    _ => panic!("Unknown function {name}"),
                }
            }
        }
    }
}

fn make_number<'a>(tree: &'a Tree) -> Result<Ast<'a>, Box<dyn Error>> {
    match tree {
        Tree::Leaf(leaf) => match leaf {
            Symbol::Number(n) => Ok(Ast::NumberLiteral(*n)),
            _ => Err("Expected Symbol::Number".into()),
        },
        _ => Err("Expected Tree::Leaf".into()),
    }
}

fn make_call<'a>(tree: &'a Tree) -> Result<Ast<'a>, Box<dyn Error>> {
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
