pub mod ast;
pub mod lexer;
pub mod codegen;

#[cfg(test)]
mod tests {
    use crate::ast::*;
    use crate::lexer::*;

    #[test]
    fn lexer_basic() {
        let sample = "(1 2 3)";
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
    fn lexer_ident() {
        let sample = "(add 1 2)";
        let expected = &[
            Token::Open,
            Token::Symbol(Symbol::Ident("add")),
            Token::Symbol(Symbol::Number(1)),
            Token::Symbol(Symbol::Number(2)),
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

    #[test]
    fn ast_nested() {
        let sample = &[
            Token::Open,
            Token::Symbol(Symbol::Number(1)), 
            Token::Symbol(Symbol::Number(2)), 
            Token::Open,
                Token::Symbol(Symbol::Ident("+")),
                Token::Symbol(Symbol::Number(1)), 
                Token::Symbol(Symbol::Number(2)), 
            Token::Close,
            Token::Close,
        ];
        let expected = Tree::Branch(vec![
            Tree::Branch(vec![
                Tree::Leaf(&Symbol::Number(1)),
                Tree::Leaf(&Symbol::Number(2)),
                Tree::Branch(vec![
                    Tree::Leaf(&Symbol::Ident("+")),
                    Tree::Leaf(&Symbol::Number(1)),
                    Tree::Leaf(&Symbol::Number(2)),
                ]),
            ]),
        ]);
        assert_eq!(Tree::try_construct(sample).unwrap(), expected);
    }

    #[test]
    fn ast_ident() {
        let sample = &[
            Token::Open,
            Token::Symbol(Symbol::Ident("add")),
            Token::Symbol(Symbol::Number(1)), 
            Token::Symbol(Symbol::Number(2)), 
            Token::Close,
        ];
        let expected = Tree::Branch(vec![
            Tree::Branch(vec![
                Tree::Leaf(&Symbol::Ident("add")),
                Tree::Leaf(&Symbol::Number(1)),
                Tree::Leaf(&Symbol::Number(2)),
            ]),
        ]);
        assert_eq!(Tree::try_construct(sample).unwrap(), expected);
    }
}
