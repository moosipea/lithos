use crate::lexer::Symbol;
use crate::lexer::Token;
use crate::Error;
use anyhow::Result;

fn take_expr<'a>(toks: &'a [Token]) -> Result<(&'a [Token<'a>], &'a [Token<'a>])> {
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
            Err(Error::UnmatchedOpenExpr.into())
        }
        _ => Err(Error::Expected("( or symbol)").into()),
    }
}

fn take_toplevel_exprs<'a>(toks: &'a [Token]) -> Result<Vec<&'a [Token<'a>]>> {
    if toks.is_empty() {
        return Err(Error::Expected("nonempty list").into());
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
pub enum Tree<'a> {
    Branch(Vec<Tree<'a>>),
    Leaf(&'a Symbol<'a>),
}

impl Tree<'_> {
    pub fn try_construct<'a>(toks: &'a [Token]) -> Result<Tree<'a>> {
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

    pub fn branch(&self) -> Option<&[Tree]> {
        match self {
            Tree::Branch(children) => Some(children),
            _ => None,
        }
    }

    pub fn leaf(&self) -> Option<&Symbol> {
        match self {
            Tree::Leaf(sym) => Some(sym),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Ast<'a> {
    // TODO: Value literal?
    NumberLiteral(u64),
    StringLiteral(String),
    Identifier(String),
    Call { name: &'a str, args: Vec<Ast<'a>> },
}

impl<'a> Ast<'a> {
    fn from_tree(tree: &'a Tree) -> Result<Self> {
        match tree {
            Tree::Branch(_) => ast_from_branch(tree),
            Tree::Leaf(_) => ast_from_leaf(tree),
        }
    }
}

fn ast_from_leaf<'a>(tree: &'a Tree) -> Result<Ast<'a>> {
    match tree {
        Tree::Leaf(leaf) => match leaf {
            Symbol::Number(n) => Ok(Ast::NumberLiteral(*n)),
            Symbol::StringLiteral(s) => Ok(Ast::StringLiteral(s.to_string())),
            Symbol::Ident(ident) => Ok(Ast::Identifier(ident.to_string())),
        },
        _ => Err(Error::Expected("Tree::Leaf").into()),
    }
}

fn ast_from_branch<'a>(tree: &'a Tree) -> Result<Ast<'a>> {
    let branch = tree.branch().ok_or(Error::Expected("Tree::Branch"))?;

    let name = match branch
        .first()
        .ok_or(Error::Expected("nonempty branch"))?
        .leaf()
        .ok_or(Error::Expected("Tree::Leaf"))?
    {
        Symbol::Ident(s) => *s,
        _ => return Err(Error::Expected("Symbol::Ident").into()),
    };

    let args = match branch.get(1..) {
        Some(rst) => rst.into_iter().map(Ast::from_tree).collect(),
        None => Ok(Vec::new()),
    }?;

    Ok(Ast::Call { name, args })
}

pub fn make_tree<'a>(tree: &'a Tree) -> Result<Box<[Ast<'a>]>> {
    match tree {
        Tree::Branch(children) => children.into_iter().map(Ast::from_tree).collect(),
        _ => Ast::from_tree(tree).map(|ok| vec![ok].into_boxed_slice()),
    }
}
