use crate::lexer::*;
use std::fmt::Debug;

// FIXME
#[derive(Debug)]
pub enum Tree<T: Debug> {
    Nil,
    Leaf(T),
    Branch(Box<[Tree<T>]>),
}

fn expr_inside<'a>(tokens: &'a [Token<'a>]) -> Option<&'a [Token<'a>]> {
    if tokens.len() <= 2 {
        return None;
    }
    tokens.get(1..tokens.len() - 1)
}

fn is_empty_expr(tokens: &[Token]) -> bool {
    match tokens
        .into_iter()
        .map(Token::kind)
        .collect::<Vec<_>>()
        .as_slice()
    {
        [TokenKind::Open, TokenKind::Close] => true,
        _ => false,
    }
}

pub fn build_syntax_tree<'a>(tokens: &'a [Token<'a>]) -> Tree<Token<'a>> {
    Tree::Branch(
        ExprTaker::new(tokens)
            .map(|expr| match expr_inside(expr) {
                Some(inside) => build_syntax_tree(inside),
                None => match is_empty_expr(expr) {
                    true => Tree::Nil,
                    false => Tree::Leaf(expr[0]),
                },
            })
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    )
}

struct ExprTaker<'a> {
    tokens: &'a [Token<'a>],
}

impl<'a> ExprTaker<'a> {
    fn new(tokens: &'a [Token<'a>]) -> Self {
        Self { tokens }
    }
}

impl<'a> Iterator for ExprTaker<'a> {
    type Item = &'a [Token<'a>];
    fn next(&mut self) -> Option<Self::Item> {
        let mut scope = 0;
        for (i, token) in self.tokens.iter().enumerate() {
            match token.kind() {
                TokenKind::Open => scope += 1,
                TokenKind::Close => scope -= 1,
                _ => {}
            }
            if scope == 0 {
                let ret = &self.tokens.get(..=i)?;
                self.tokens = &self.tokens.get(i + 1..)?;
                return Some(ret);
            }
        }
        None
    }
}
