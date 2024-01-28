pub mod ast;
pub mod lexer;

#[cfg(test)]
mod tests {
    use super::ast::*;
    use super::lexer::*;

    const COMMON_INPUT: &str = "; Comment\n(a b c (d 34 \"hello world\"))";

    #[test]
    fn scanner() {
        let lexemes: Vec<_> = Scanner::new(COMMON_INPUT)
            .map(|lexeme| lexeme.content())
            .collect();
        let expected = vec![
            "(",
            "a",
            "b",
            "c",
            "(",
            "d",
            "34",
            "\"hello world\"",
            ")",
            ")",
        ];
        assert_eq!(lexemes, expected);
    }

    #[test]
    fn evaluator() {
        use TokenKind::*;
        let tokens: Vec<_> = Scanner::new(COMMON_INPUT)
            .evaluate()
            .map(|token| token.kind())
            .collect();
        let expected = vec![
            Open,
            Identifier("a"),
            Identifier("b"),
            Identifier("c"),
            Open,
            Identifier("d"),
            NumberLiteral("34"),
            StringLiteral("hello world"),
            Close,
            Close,
        ];
        assert_eq!(tokens, expected);
    }
}
