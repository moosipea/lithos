pub mod ast;
pub mod lexer;

#[derive(Debug, PartialEq)]
pub enum Symbol<'a> {
    Ident(&'a str),
    Number(i32),
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Open,
    Close,
    Symbol(Symbol<'a>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::lexer::*;

    #[test]
    fn lexer_basic() {
        let sample = "(1, 2, 3)";
        let expected = &[
            Token::Open,
            Token::Symbol(Symbol::Number(1)),
            Token::Symbol(Symbol::Number(2)),
            Token::Symbol(Symbol::Number(3)),
            Token::Close,
        ];
        assert_eq!(lex(sample).expect("Lexing failed!"), expected)
    }

    #[test]
    fn ast_basic() {
        let sample = &[
            Token::Open,
            Token::Symbol(Symbol::Number(1)), 
            Token::Symbol(Symbol::Number(2)), 
            Token::Symbol(Symbol::Number(3)), 
            Token::Close,
        ];
        let expected = Tree::Branch(vec![
            Tree::Branch(vec![
                Tree::Leaf(&Symbol::Number(1)),
                Tree::Leaf(&Symbol::Number(2)),
                Tree::Leaf(&Symbol::Number(3)),
            ]),
        ]);
        assert_eq!(Tree::try_construct(sample).unwrap(), expected);
    }
}