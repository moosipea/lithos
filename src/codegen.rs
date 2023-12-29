use std::error::Error;

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

impl AstToken<'_> {
    fn codegen(self) -> Result<String, Box<dyn Error>> {
        match self {
            AstToken::Call { name, args } => {
                // See: http://6.s081.scripts.mit.edu/sp18/x86-64-architecture-guide.html
                if args.len() > 6 {
                    return Err("Call with more than 6 arguments not supported".into());
                }

                let registers = &["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];

                let mut result = format!("// Calling '{name}' with args: {args:?}\n");

                for (i, value) in args.iter().enumerate() {
                    result.push_str(&format!("mov {reg}, {value}", reg = registers[i]));
                }

                result.push_str(&format!("call {name}"));

                Ok(result)
            }
            _ => Err("Unimplemented".into()),
        }
    }
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
        Some(rst) => rst
            .into_iter()
            .filter_map(|e| match e {
                Tree::Leaf(sym) => match *sym {
                    Symbol::Number(n) => Some(*n),
                    _ => None,
                },
                _ => None,
            })
            .collect(),
        None => Vec::new(),
    };

    Ok(AstToken::Call { name, args })
}

// trait SequenceExt {
//     fn sequence(&mut self) -> Option<impl Iterator>;
// }
// 
// impl SequenceExt for dyn Iterator<Item = Option<i32>> {
//     fn sequence<'a>(&'a mut self) -> Option<impl Iterator + 'a> {
//         Some(self)
//     }
// }

trait SequenceExt {
    fn sequence<B: FromIterator<i32>>(self) -> Option<B>;
}

impl<T: Iterator<Item = Option<i32>>> SequenceExt for T {
    fn sequence<B: FromIterator<i32>>(mut self) -> Option<B> {
        if self.all(|opt| opt.is_some()) {
            return Some(B::from_iter(self.map(Option::unwrap)));
        }
        None
    }
}

pub fn make_ast_token(tree: Tree) -> Result<(), Box<dyn Error>> {
    match tree {
        Tree::Branch(children) => {
            let calls = children.iter().filter_map(|e| {
                make_call(e).map_or_else(
                    |err| {
                        eprintln!("(error/nonfatal) {err}");
                        None
                    },
                    |call| Some(call),
                )
            }).map(AstToken::codegen);

            Ok(())
        }
        _ => Err("Expected Tree::Branch".into()),
    }
}
