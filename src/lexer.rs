#[derive(Debug, PartialEq)]
pub struct Lexeme<'a> {
    content: &'a str,
    index: usize,
}

impl<'a> Lexeme<'a> {
    fn new(content: &'a str, index: usize) -> Self {
        Self { content, index }
    }

    pub fn content(&self) -> &'a str {
        self.content
    }

    fn positioning(&self) -> Positioning {
        Positioning {
            index: self.index,
            length: self.content.len(),
        }
    }
}

pub fn preprocess(src: &str) -> String {
    src.replace("\r\n", "\n")
}

pub struct Scanner<'a> {
    src: &'a str,
    index: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, index: 0 }
    }
}

impl<'a> Scanner<'a> {
    pub fn evaluate(self) -> Evaluator<'a, Self> {
        Evaluator { lexemes: self }
    }

    fn take(&mut self, n: usize) -> Option<&'a str> {
        let head = self.src.get(..n)?;
        self.src = self.src.get(n..)?;
        self.index += n;
        Some(head)
    }

    fn take_until<F: Fn(char) -> bool>(
        &mut self,
        predicate: F,
        inclusive: bool,
    ) -> Option<&'a str> {
        for (i, ch) in self.src.chars().enumerate().skip(1) {
            if predicate(ch) {
                return self.take(match inclusive {
                    true => i + 1,
                    false => i,
                });
            }
        }
        None
    }

    fn take_one(&mut self) -> Option<&'a str> {
        self.take(1)
    }

    fn take_symbol(&mut self) -> Option<&'a str> {
        self.take_until(|ch| ch == '(' || ch == ')' || ch.is_whitespace(), false)
    }

    fn take_string_literal(&mut self) -> Option<&'a str> {
        self.take_until(|ch| ch == '"', true)
    }

    fn skip_comment(&mut self) -> Option<()> {
        self.take_until(|ch| ch == '\n', false)?;
        Some(())
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Lexeme<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ch) = self.src.chars().next() {
            let index = self.index;
            match ch {
                ';' => self.skip_comment()?,
                '(' | ')' => {
                    return self
                        .take_one()
                        .and_then(|content| Some(Lexeme::new(content, index)))
                }
                '"' => {
                    return self
                        .take_string_literal()
                        .and_then(|content| Some(Lexeme::new(content, index)))
                }
                ch if !ch.is_whitespace() => {
                    return self
                        .take_symbol()
                        .and_then(|content| Some(Lexeme::new(content, index)))
                }
                _ => {
                    self.take_one()?;
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Positioning {
    index: usize,
    length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind<'a> {
    Open,
    Close,
    NumberLiteral(&'a str),
    StringLiteral(&'a str),
    Identifier(&'a str),
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    kind: TokenKind<'a>,
    pos: Positioning,
}

fn shrink_str<'a>(string: &'a str) -> &'a str {
    if string.len() <= 2 {
        return string;
    }
    &string[1..string.len() - 1]
}

impl<'a> Token<'a> {
    pub fn kind(&self) -> TokenKind<'a> {
        self.kind
    }

    fn open(pos: Positioning) -> Self {
        Self {
            kind: TokenKind::Open,
            pos,
        }
    }

    fn close(pos: Positioning) -> Self {
        Self {
            kind: TokenKind::Close,
            pos,
        }
    }

    fn number_literal(content: &'a str, pos: Positioning) -> Self {
        Self {
            kind: TokenKind::NumberLiteral(content),
            pos,
        }
    }

    fn string_literal(content: &'a str, pos: Positioning) -> Self {
        Self {
            kind: TokenKind::StringLiteral(shrink_str(content)),
            pos,
        }
    }

    fn identifier(content: &'a str, pos: Positioning) -> Self {
        Self {
            kind: TokenKind::Identifier(content),
            pos,
        }
    }
}

pub struct Evaluator<'a, T: Iterator<Item = Lexeme<'a>>> {
    lexemes: T,
}

fn is_number_literal(string: &str) -> bool {
    string.chars().all(|ch| ch.is_ascii_digit())
}

fn is_string_literal(string: &str) -> bool {
    string.starts_with('"') && string.ends_with('"')
}

impl<'a, T: Iterator<Item = Lexeme<'a>>> Iterator for Evaluator<'a, T> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let lexeme = self.lexemes.next()?;
        let content = lexeme.content;
        let pos = lexeme.positioning();

        Some(match content {
            "(" => Token::open(pos),
            ")" => Token::close(pos),
            content if is_number_literal(content) => Token::number_literal(content, pos),
            content if is_string_literal(content) => Token::string_literal(content, pos),
            _ => Token::identifier(content, pos),
        })
    }
}
