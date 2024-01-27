#[derive(Debug, PartialEq)]
pub enum TokenKind<'a> {
    Open,
    Close,
    Symbol(&'a str),
}

//#[derive(Debug)]
//pub struct Token<'a> {
//    kind: TokenKind<'a>,
//    pos: Positioning,
//}

pub fn preprocess(src: &str) -> String {
    src.replace("\r\n", "\n")
}

// TODO: See: https://crates.io/crates/unicode-segmentation

pub struct Tokeniser<'a> {
    src: &'a str
}

impl<'a> Tokeniser<'a> {
    pub fn new(src: &'a str) -> Self {
	Self { src }
    }
}

impl<'a> Tokeniser<'a> {
    fn take(&mut self, n: usize) -> Option<&'a str> {
	let head = self.src.get(..n);
	self.src = self.src.get(n..)?;
	head
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

impl<'a> Iterator for Tokeniser<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
	for ch in self.src.chars() {
	    match ch {
		';' => {
		    self.skip_comment()?;
		    return self.next();
		},
		'(' | ')' => return self.take_one(),		
		ch if !ch.is_whitespace() => return self.take_symbol(),
		_ => {
		    self.take_one()?;
		    return self.next();
		}
	    }
	}	
	None
    }
}

