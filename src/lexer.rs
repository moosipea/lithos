use super::*;
use std::error::Error;

fn should_stop_taking(c: char) -> bool {
    match c {
        '(' | ')' => true,
        c if c.is_whitespace() => true,
        _ => false,
    }
}

fn remove_leading_whitespace(s: &str) -> &str {
    for (i, c) in s.char_indices() {
        match c {
            c if c.is_whitespace() => {}
            _ => return &s[i..]
        }
    }
    s
}

fn read(src: &str) -> Result<(&str, &str), Box<dyn Error>> {
    let src = remove_leading_whitespace(src);
    let first_str = &src[0..1];
    match first_str {
        "(" | ")" => return Ok((first_str, &src[1..])),
        _ => {}
    }
    
    let indices: Vec<_> = src
        .char_indices()
        .take_while(|(_, c)| !should_stop_taking(*c))
        .map(|(i, _)| i)
        .collect();

    let start = *indices.first().ok_or("Ident with length 0")?;
    let end = *indices.last().ok_or("Ident with length 0")?;

    Ok((&src[start..=end], &src[end+1..]))
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
