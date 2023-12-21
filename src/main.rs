use std::error::Error;

#[derive(Debug)]
enum Symbol<'a> {
    Ident(&'a str),
    Number(i32),
}

#[derive(Debug)]
enum Token<'a> {
    Open,
    Close,
    Symbol(Symbol<'a>),
}

#[derive(Debug)]
enum Tree<'a> {
    Branch(Vec<Tree<'a>>),
    Leaf(&'a Symbol<'a>),
}

fn take_next_expr<'a>(
    tokens: &'a [Token],
) -> Result<(&'a [Token<'a>], &'a [Token<'a>]), Box<dyn Error>> {
    println!("(!) Taking from {tokens:?}");
    if tokens.is_empty() {
        return Err("Empty list".into());
    }

    if let Token::Symbol(_) = tokens[0] {
        return Ok((&tokens[0..1], &tokens[2..]));
    }

    let mut scope = 0;
    for (i, t) in tokens.iter().enumerate() {
        match t {
            Token::Open => scope += 1,
            Token::Close => scope -= 1,
            _ => {}
        }

        if scope == 0 {
            return Ok((&tokens[1..i], &tokens[i..]))
        }
    }

    Err("Failed to take expr".into())
}

impl Tree<'_> {
    pub fn try_construct<'a>(tokens: &'a [Token]) -> Result<Tree<'a>, Box<dyn Error>> {
        let (head, tail) = take_next_expr(tokens)?;

        println!("Head: {head:?}");

        let next = match &head[0] {
            Token::Symbol(s) => Tree::Leaf(s),
            _ => Self::try_construct(head)?
        };

        match &tail[0] {
            Token::Close => {},
            _ => { Self::try_construct(tail)?; }
        }

        todo!()
    }
}

fn main() {
    let sample_data = &[
        Token::Open,
        Token::Open,
            Token::Symbol(Symbol::Number(1)),
        Token::Close,
        Token::Close,
    ];

    let tree = Tree::try_construct(sample_data).expect("Expected to construct tree");
    println!("{tree:?}");
}
