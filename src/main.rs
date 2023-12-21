use std::error::Error;

#[derive(Debug, PartialEq)]
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

fn take_expr<'a>(toks: &'a [Token]) -> Result<(&'a [Token<'a>], &'a [Token<'a>]), Box<dyn Error>> {
    match toks[0] {
        Token::Symbol(_) => Ok((&toks[0..1], &toks[1..])),
        Token::Open => {
            let mut scope = 0;
            for (i, t) in toks.iter().enumerate() {
                match t {
                    Token::Open => scope += 1,
                    Token::Close => scope -= 1,
                    _ => {}
                }
                if scope == 0 {
                    return Ok((&toks[0..i + 1], &toks[i + 1..]));
                }
            }
            Err("Unmatched '('".into())
        }
        _ => Err("Expected '(' or symbol".into()),
    }
}

fn take_toplevel_exprs<'a>(toks: &'a [Token]) -> Result<Vec<&'a [Token<'a>]>, Box<dyn Error>> {
    if toks.is_empty() {
        return Err("Empty list".into());
    }

    let mut exprs = Vec::new();
    let mut tail = toks;
    while !tail.is_empty() {
        let head;
        (head, tail) = take_expr(tail)?;
        exprs.push(head);
    }
    Ok(exprs)
}

fn slice_middle<'a, T>(slc: &'a [T]) -> Option<&'a [T]> {
    if slc.len() < 3 {
        return None;
    }
    Some(&slc[1..slc.len() - 1])
}

#[derive(Debug, PartialEq)]
enum Tree<'a> {
    Branch(Vec<Tree<'a>>),
    Leaf(&'a Symbol<'a>),
}

impl Tree<'_> {
    pub fn try_construct<'a>(toks: &'a [Token]) -> Result<Tree<'a>, Box<dyn Error>> {
        match toks {
            [Token::Symbol(sym)] => Ok(Tree::Leaf(sym)),
            _ => {
                let expressions = take_toplevel_exprs(toks)?;
                let tree: Result<Vec<_>, _> = expressions
                    .into_iter()
                    .map(|e| match slice_middle(e) {
                        Some(middle) => Self::try_construct(middle),
                        None => Self::try_construct(e),
                    })
                    .collect();
                Ok(Tree::Branch(tree?))
            }
        }
    }
}

fn main() {
    let sample_data = &[
        Token::Open,
        Token::Symbol(Symbol::Number(1)),
        Token::Symbol(Symbol::Number(2)),
        Token::Open,
            Token::Symbol(Symbol::Ident("+")),
            Token::Symbol(Symbol::Number(1)),
            Token::Symbol(Symbol::Number(2)),
        Token::Close,
        Token::Close,
    ];

    let tree = Tree::try_construct(sample_data).expect("Expected to construct tree");
    dbg!(tree);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let sample = &[
            Token::Open,
            Token::Symbol(Symbol::Number(1)), 
            Token::Symbol(Symbol::Number(2)), 
            Token::Symbol(Symbol::Number(3)), 
            Token::Close,
        ];
        
        let expected = Tree::Branch(vec![
            Tree::Branch(vec![
                Tree::Leaf(&Symbol::Number(1)),
                Tree::Leaf(&Symbol::Number(2)),
                Tree::Leaf(&Symbol::Number(3)),
            ]),
        ]);

        assert_eq!(Tree::try_construct(sample).unwrap(), expected);
    }
}
