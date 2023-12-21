use super::*;
use std::error::Error;

// TODO: lexing for idents (i forgor)
fn read(src: &str) -> Result<(&str, &str), Box<dyn Error>> {
    let mut start = 0;
    for (i, c) in src.char_indices() {
        match c {
            w if w.is_whitespace() => {
                start += 1;
                continue
            },
            '(' | ')' => return Ok((&src[0..=i], &src[i+1..])),
            _ => return Ok((&src[start..=i], &src[i+1..]))
        }
    }

    Err("Read failed".into())
}

fn is_number(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

fn read_number(s: &str) -> Result<i32, Box<dyn Error>> {
    s.parse::<i32>().map_err(|err| err.into())
}

impl<'a> TryFrom<&'a str> for Token<'a> {
    type Error = Box<dyn Error>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(match value {
            "(" => Token::Open,
            ")" => Token::Close,
            s if is_number(s) => Token::Symbol(Symbol::Number(read_number(s)?)),
            _ => Token::Symbol(Symbol::Ident(value)),
        })
    }
}

pub fn lex(mut src: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens = Vec::new();
    while !src.is_empty() {
        let tok;
        (tok, src) = read(src)?;
        tokens.push(tok.try_into()?);
    }
    Ok(tokens)
}
