//use crate::Error;
use std::str::Chars;
//use anyhow::Result;

#[derive(Debug)]
pub struct Positioning {
    start: usize,
    length: usize,
}

impl Positioning {
    fn new(start: usize, length: usize) -> Self {
	Self { start, length }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind<'a> {
    Open,
    Close,
    Symbol(&'a str),
}

#[derive(Debug)]
pub struct Token<'a> {
    kind: TokenKind<'a>,
    pos: Positioning,
}

impl<'a> Token<'a> {
    fn open_or_close(ch: char, cursor: usize) -> Option<Self> {
	let kind = match ch {
	    '(' => Some(TokenKind::Open),
	    ')' => Some(TokenKind::Close),
	    _ => None
	}?;
	Some(Self {
	    kind,
	    pos: Positioning::new(cursor, 1),
	})
    }

    fn symbol(content: &'a str, cursor: usize) -> Option<Self> {
	if content.is_empty() {
	    return None;
	}
	Some(Self {
	    kind: TokenKind::Symbol(content),
	    pos: Positioning::new(cursor, content.len()),
	})
    }
}

struct StrBuffer<'a> {
    slc: &'a str,
    start: usize,
    length: usize,
}

impl<'a> StrBuffer<'a> {
    fn new(slc: &'a str) -> Self {
	Self {
	    slc,
	    start: 0,
	    length: 0,
	}
    }

    fn reset(&mut self, start: usize) -> Option<()> {
	if start >= self.slc.len() {
	    return None;
	}
	self.start = start;
	self.length = 0;
	Some(())
    }

    fn extend_by_one(&mut self) -> Option<()> {
	if self.start + self.length + 1 >= self.slc.len() {
	    return None;
	}
	self.length += 1;
	Some(())
    }
    
    fn content(&self) -> Option<&'a str> {
	self.slc.get(self.start..self.start + self.length)
    }

    fn is_empty(&self) -> bool {
	self.length == 0
    }
}

pub struct Lexer<'a> {
    chars: Vec<char>,        
    buffer: StrBuffer<'a>,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            chars: src.chars().collect(),
	    buffer: StrBuffer::new(src),
	    cursor: 0,
        }
    }

}

// N.B! Positions of token may be incorrect because of this
pub fn remove_comments(string: &str) -> String {
    let mut comment = false;
    string.chars().filter(|ch| {
	match ch {
	    ';' => comment = true,
	    '\n' => comment = false,
	    _ => {}
	}
	!comment
    }).collect()
}

// TODO: See: https://crates.io/crates/unicode-segmentation

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
	loop {
	    let ch = *self.chars.get(self.cursor)?;
	    println!("{ch:?}");
	    match ch {
		'(' | ')' => if self.buffer.is_empty() {
		    let tok = Token::open_or_close(ch, self.cursor);
		    self.cursor += 1;
		    return tok;
		} else {
		    let tok = Token::symbol(self.buffer.content()?, self.buffer.start);
		    self.buffer.reset(self.cursor)?;
		    return tok;
		},
		w if w.is_whitespace() => if !self.buffer.is_empty() {
		    let tok = Token::symbol(self.buffer.content()?, self.buffer.start);
		    self.cursor += 1;
		    self.buffer.reset(self.cursor)?;
		    return tok;
		},
		_ => {
		    if self.buffer.is_empty() {
			self.buffer.reset(self.cursor)?;
		    }
		    self.buffer.extend_by_one()?;
		    self.cursor += 1;
		}
	    }
	}
    }
}

