use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum Symbol<'a> {
    Ident(&'a str),
    Number(i32),
    StringLiteral(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Open,
    Close,
    Symbol(Symbol<'a>),
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

// Need to support
// 1. Open/Close
// 2. Identifiers
// 3. Number literals
// 4. String literals (!)

impl Lexer<'_> {
    fn end<F: Fn(char) -> bool>(&mut self, when: F) -> Option<usize> {
        // for (i, c) in (1..).zip(self.src.chars()) {
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
    type Item = Result<Token<'a>, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.src.chars().next()?;
        match first {
            w if w.is_whitespace() => match self.end(|c| !c.is_whitespace()) {
                Some(len) => {
                    self.src = &self.src[len..];
                    self.next()
                }
                None => Some(Err("Trailing whitespace".into())),
            },
            ';' => match self.end(|c| c == '\n') {
                Some(len) => {
                    self.src = &self.src[len..];
                    self.next()
                }
                None => Some(Err("Undelimited comment".into())),
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
                None => Some(Err("Undelimited string literal".into())),
            },
            _ => match self.end(|c| c.is_whitespace() || c == '(' || c == ')') {
                Some(len) => {
                    let sym = &self.src[..len];
                    self.src = &self.src[len..];

                    Some(Ok(Token::Symbol(match sym.parse::<i32>() {
                        Ok(num) => Symbol::Number(num),
                        _ => Symbol::Ident(sym),
                    })))
                }
                None => Some(Err("Undelimited symbol?".into())),
            },
        }
    }
}

pub fn lex(src: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let lexer = Lexer::new(src);
    let mut tokens = Vec::new();
    for maybe in lexer {
        match maybe {
            Ok(tok) => tokens.push(tok),
            Err(err) => return Err(err),
        }
    }
    Ok(tokens)
}
