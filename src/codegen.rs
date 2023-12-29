use std::error::Error;
use std::ops::RangeInclusive;

use crate::ast::Tree;
use crate::Symbol;

#[derive(Debug)]
enum AstToken<'a> {
    NumberLiteral(i32),
    Identifier(&'a str),
    Call {
        name: &'a str, // Or AstToken?
        // args: &'a [AstToken<'a>],
        args: Vec<i32>,
    },
}

// Branch
//   Leaf(identifier)
//   Leaf(a)
//   Leaf(b)
// ->
// Call {
//     name: identifier,
//     args: &[a, b]
// }

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
        Some(rst) => {
            rst.into_iter().filter_map(|e| match e {
                Tree::Leaf(sym) => match *sym {
                    Symbol::Number(n) => Some(*n),
                    _ => None
                },
                _ => None
            }).collect()
        },
        None => Vec::new(),
    };

    Ok(AstToken::Call { name, args })
}

pub fn make_ast_token(tree: Tree) -> Result<(), Box<dyn Error>> {
    match tree {
        Tree::Branch(children) => {
            let mut xs = Vec::new();
            for c in &children {
                match make_call(c) {
                    Ok(call) => xs.push(call),
                    Err(e) => println!("(error/nonfatal) {e}"),
                }
            }
            println!("{xs:#?}");
            Ok(())
        },
        _ => Err("Expected Tree::Branch".into())
    }
}
