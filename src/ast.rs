use crate::lexer::Symbol;
use crate::lexer::Token;
use std::error::Error;

fn take_expr<'a>(toks: &'a [Token]) -> Result<(&'a [Token<'a>], &'a [Token<'a>]), Box<dyn Error>> {
    match toks[0] {
        Token::Symbol(_) => Ok((&toks[0..1], &toks[1..])),
        Token::Open => {
            let mut scope = 0;
            for (i, t) in toks.iter().enumerate() {
                match t {
                    Token::Open => scope += 1,
                    Token::Close => scope -= 1,
                    _ => {}
                }
                if scope == 0 {
                    return Ok((&toks[0..i + 1], &toks[i + 1..]));
                }
            }
            Err("Unmatched '('".into())
        }
        _ => Err("Expected '(' or symbol".into()),
    }
}

fn take_toplevel_exprs<'a>(toks: &'a [Token]) -> Result<Vec<&'a [Token<'a>]>, Box<dyn Error>> {
    if toks.is_empty() {
        return Err("Empty list".into());
    }

    let mut exprs = Vec::new();
    let mut tail = toks;
    while !tail.is_empty() {
        let head;
        (head, tail) = take_expr(tail)?;
        exprs.push(head);
    }
    Ok(exprs)
}

fn slice_middle<'a, T>(slc: &'a [T]) -> Option<&'a [T]> {
    if slc.len() < 3 {
        return None;
    }
    Some(&slc[1..slc.len() - 1])
}

#[derive(Debug, PartialEq)]
pub enum Tree<'a> {
    Branch(Vec<Tree<'a>>),
    Leaf(&'a Symbol<'a>),
}

impl Tree<'_> {
    pub fn try_construct<'a>(toks: &'a [Token]) -> Result<Tree<'a>, Box<dyn Error>> {
        match toks {
            [Token::Symbol(sym)] => Ok(Tree::Leaf(sym)),
            _ => {
                let expressions = take_toplevel_exprs(toks)?;
                let tree: Result<Vec<_>, _> = expressions
                    .into_iter()
                    .map(|e| match slice_middle(e) {
                        Some(middle) => Self::try_construct(middle),
                        None => Self::try_construct(e),
                    })
                    .collect();
                Ok(Tree::Branch(tree?))
            }
        }
    }

    pub fn branch(&self) -> Option<&[Tree]> {
        match self {
            Tree::Branch(children) => Some(children),
            _ => None,
        }
    }

    pub fn leaf(&self) -> Option<&Symbol> {
        match self {
            Tree::Leaf(sym) => Some(sym),
            _ => None,
        }
    }
}
