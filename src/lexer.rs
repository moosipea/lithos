use std::error::Error;

// fn should_stop_taking(c: char) -> bool {
//     match c {
//         '(' | ')' => true,
//         c if c.is_whitespace() => true,
//         _ => false,
//     }
// }
//
// fn remove_leading_whitespace(s: &str) -> &str {
//     for (i, c) in s.char_indices() {
//         match c {
//             c if c.is_whitespace() => {}
//             _ => return &s[i..]
//         }
//     }
//     s
// }
//
// fn read(src: &str) -> Result<(&str, &str), Box<dyn Error>> {
//     let src = remove_leading_whitespace(src);
//     let first_str = &src[0..1];
//     match first_str {
//         "(" | ")" => return Ok((first_str, &src[1..])),
//         _ => {}
//     }
//
//     let indices: Vec<_> = src
//         .char_indices()
//         .take_while(|(_, c)| !should_stop_taking(*c))
//         .map(|(i, _)| i)
//         .collect();
//
//     let start = *indices.first().ok_or("Ident with length 0")?;
//     let end = *indices.last().ok_or("Ident with length 0")?;
//
//     Ok((&src[start..=end], &src[end+1..]))
// }
//
// fn is_number(s: &str) -> bool {
//     s.chars().all(|c| c.is_ascii_digit())
// }
//
// fn read_number(s: &str) -> Result<i32, Box<dyn Error>> {
//     s.parse::<i32>().map_err(|err| err.into())
// }
//
// impl<'a> TryFrom<&'a str> for Token<'a> {
//     type Error = Box<dyn Error>;
//
//     fn try_from(value: &'a str) -> Result<Self, Self::Error> {
//         Ok(match value {
//             "(" => Token::Open,
//             ")" => Token::Close,
//             s if is_number(s) => Token::Symbol(Symbol::Number(read_number(s)?)),
//             _ => Token::Symbol(Symbol::Ident(value)),
//         })
//     }
// }

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

impl Lexer<'_> {
    fn new(src: &str) -> Self {
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
// 4. String literals

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rst.is_empty() {
            return None;
        }

        let first = self.rst[0];
        match first {
            '(' => {
            },
            ')' => {},
            _ => {}
        }

        todo!()
    }
}

pub fn lex(src: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut lexer = Lexer::new(src);
    let mut tokens = Vec::new();
    for maybe in lexer {
        match maybe {
            Ok(tok) => tokens.push(tok),
            _ => return maybe,
        }
    }
    Ok(tokens)
}

// pub fn lex(src: &str) -> Result<Vec<Token>, Box<dyn Error>> {
//     let mut tokens = Vec::new();
//     let mut src = src.trim_end();
//     while !src.is_empty() {
//         let tok;
//         (tok, src) = read(src)?;
//         tokens.push(tok.try_into()?);
//     }
//     Ok(tokens)
// }
