pub mod ast;
pub mod codegen;
pub mod lexer;

#[cfg(test)]
mod tests {
    use crate::ast::*;
    use crate::lexer::*;
    use Symbol as S;
    use Token as T;

    #[test]
    fn lexer_basic() {
        let sample = "(1 2 3)";
        let expected = &[
            T::Open,
            T::Symbol(S::Number(1)),
            T::Symbol(S::Number(2)),
            T::Symbol(S::Number(3)),
            T::Close,
        ];
        assert_eq!(lex(sample).expect("Lexing failed!"), expected)
    }

    #[test]
    fn lexer_ident() {
        let sample = "(add 1 2)";
        let expected = &[
            T::Open,
            T::Symbol(S::Ident("add")),
            T::Symbol(S::Number(1)),
            T::Symbol(S::Number(2)),
            T::Close,
        ];
        assert_eq!(lex(sample).expect("Lexing failed!"), expected)
    }

    #[test]
    fn lexer_string_literal() {
        let sample = "(print \"foo\" \"bar\" \"baz\")";
        let expected = &[
            T::Open,
            T::Symbol(S::Ident("print")),
            T::Symbol(S::StringLiteral("foo")),
            T::Symbol(S::StringLiteral("bar")),
            T::Symbol(S::StringLiteral("baz")),
            T::Close,
        ];
        assert_eq!(lex(sample).expect("Lexing failed!"), expected);
    }

    #[test]
    fn lexer_comment() {
        let sample = "; what does this do?\n(+ 34 35)";
        let expected = &[
            T::Open,
            T::Symbol(S::Ident("+")),
            T::Symbol(S::Number(34)),
            T::Symbol(S::Number(35)),
            T::Close,
        ];
        assert_eq!(lex(sample).expect("Lexing failed!"), expected);
    }

    #[test]
    fn ast_basic() {
        let sample = &[
            T::Open,
            T::Symbol(S::Number(1)),
            T::Symbol(S::Number(2)),
            T::Symbol(S::Number(3)),
            T::Close,
        ];
        let expected = Tree::Branch(vec![Tree::Branch(vec![
            Tree::Leaf(&S::Number(1)),
            Tree::Leaf(&S::Number(2)),
            Tree::Leaf(&S::Number(3)),
        ])]);
        assert_eq!(Tree::try_construct(sample).unwrap(), expected);
    }

    #[test]
    fn ast_nested() {
        let sample = &[
            T::Open,
            T::Symbol(S::Number(1)),
            T::Symbol(S::Number(2)),
            T::Open,
            T::Symbol(S::Ident("+")),
            T::Symbol(S::Number(1)),
            T::Symbol(S::Number(2)),
            T::Close,
            T::Close,
        ];
        let expected = Tree::Branch(vec![Tree::Branch(vec![
            Tree::Leaf(&S::Number(1)),
            Tree::Leaf(&S::Number(2)),
            Tree::Branch(vec![
                Tree::Leaf(&S::Ident("+")),
                Tree::Leaf(&S::Number(1)),
                Tree::Leaf(&S::Number(2)),
            ]),
        ])]);
        assert_eq!(Tree::try_construct(sample).unwrap(), expected);
    }

    #[test]
    fn ast_ident() {
        let sample = &[
            T::Open,
            T::Symbol(S::Ident("add")),
            T::Symbol(S::Number(1)),
            T::Symbol(S::Number(2)),
            T::Close,
        ];
        let expected = Tree::Branch(vec![Tree::Branch(vec![
            Tree::Leaf(&S::Ident("add")),
            Tree::Leaf(&S::Number(1)),
            Tree::Leaf(&S::Number(2)),
        ])]);
        assert_eq!(Tree::try_construct(sample).unwrap(), expected);
    }

    use std::error::Error;
    fn interpert<'a>(src: &'a str) -> Result<i32, Box<dyn Error>> {
        use crate::*;

        let tokens = lexer::lex(src)?;
        let tree = ast::Tree::try_construct(&tokens)?;
        let ast = codegen::AstToken::from_tree(&tree);

        Ok(ast.eval())
    }

    #[test]
    fn operations() {
        let pairs = &[
            ("+ (+ 1 2)", 3),
            ("+ (- 10 4)", 6),
            ("+ (* 6 7)", 42),
            ("+ (/ 16 2)", 8),
        ];

        for (sample, expected) in pairs {
            assert_eq!(interpert(sample).expect("Interperting failed"), *expected);
        }
    }
}
