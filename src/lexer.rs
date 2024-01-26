use crate::Error;
use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Open,
    Close,
    Symbol(&'a str),
}

struct Lexer<'a> {
    src: &'a str,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            src: src.trim_end(),
            line: 0,
            column: 0,
        }
    }
}

impl Lexer<'_> {
    fn end<F: Fn(char) -> bool>(&mut self, when: F) -> Option<usize> {
        for (i, c) in self.src.chars().enumerate().skip(1) {
            match c {
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                }
                _ => self.column += 1,
            }

            if when(c) {
                return Some(i);
            }
        }
        None
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.src.chars().next()?;
        match first {
            w if w.is_whitespace() => match self.end(|c| !c.is_whitespace()) {
                Some(len) => {
                    self.src = &self.src[len..];
                    self.next()
                }
                None => Some(Err(Error::TrailingWhitespace.into())),
            },
            ';' => match self.end(|c| c == '\n') {
                Some(len) => {
                    self.src = &self.src[len..];
                    self.next()
                }
                None => Some(Err(Error::UndelimitedComment.into())),
            },
            '(' => {
                self.column += 1;
                self.src = &self.src[1..];
                Some(Ok(Token::Open))
            }
            ')' => {
                self.column += 1;
                self.src = &self.src[1..];
                Some(Ok(Token::Close))
            }
            '"' => match self.end(|c| c == '"') {
                Some(len) => {
                    let literal = Symbol::StringLiteral(&self.src[1..len]);
                    self.src = &self.src[len + 1..];
                    Some(Ok(Token::Symbol(literal)))
                }
                None => Some(Err(Error::UndelimitedString.into())),
            },
            _ => match self.end(|c| c.is_whitespace() || c == '(' || c == ')') {
                Some(len) => {
                    let sym = &self.src[..len];
                    self.src = &self.src[len..];

                    Some(Ok(Token::Symbol(match sym.parse::<u64>() {
                        Ok(num) => Symbol::Number(num),
                        _ => Symbol::Ident(sym),
                    })))
                }
                None => Some(Err(Error::Expected("delimited symbol?").into())),
            },
        }
    }
}

pub fn lex(src: &str) -> Result<Vec<Token>> {
    Lexer::new(src).collect()
}
