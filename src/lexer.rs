#[derive(Debug)]
pub struct Lexeme<'a> {
    content: &'a str,
    index: usize,
}

impl<'a> Lexeme<'a> {
    fn new(content: &'a str, index: usize) -> Self {
	Self { content, index }
    }

    fn positioning(&self) -> Positioning {
	Positioning {
	    index: self.index,
	    length: self.content.len()
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

    fn take_until<F: Fn(char) -> bool>(&mut self, predicate: F) -> Option<&'a str> {
	for (i, ch) in self.src.chars().enumerate() {
	    if predicate(ch) {
		return self.take(i);
	    }
	}
	None
    }
    
    fn take_one(&mut self) -> Option<&'a str> {
	self.take(1)
    }

    fn take_symbol(&mut self) -> Option<&'a str> {
	self.take_until(|ch| ch == '(' || ch == ')' || ch.is_whitespace())
    }

    fn skip_comment(&mut self) -> Option<()> {
	self.take_until(|ch| ch == '\n')?;
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
		'(' | ')' => return self.take_one()
		    .and_then(|content| Some(Lexeme::new(content, index))),		
		ch if !ch.is_whitespace() => return self.take_symbol()
		    .and_then(|content| Some(Lexeme::new(content, index))),
		_ => { self.take_one()?; }
	    }
	}	
	None
    }
}

#[derive(Debug)]
pub struct Positioning {
    index: usize,
    length: usize,
}

#[derive(Debug)]
pub enum TokenKind<'a> {
    Open,
    Close,
    NumberLiteral(&'a str),
    //StringLiteral(&'a str),
    Identifier(&'a str),
}

#[derive(Debug)]
pub struct Token<'a> {
    kind: TokenKind<'a>,
    pos: Positioning,
}

impl<'a> Token<'a> {
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
	    pos
	}
    }

    fn identifier(content: &'a str, pos: Positioning) -> Self {
	Self {
	    kind: TokenKind::Identifier(content),
	    pos
	}
    }
}

pub struct Evaluator<'a, T: Iterator<Item = Lexeme<'a>>> {
    lexemes: T,
}

fn is_number_literal(string: &str) -> bool {
    string.chars().all(|ch| ch.is_ascii_digit())
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
	    _ => Token::identifier(content, pos)		
	})
    }
}
