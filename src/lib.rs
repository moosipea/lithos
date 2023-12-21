pub mod ast;
pub mod lexer;

#[derive(Debug, PartialEq)]
pub enum Symbol<'a> {
    Ident(&'a str),
    Number(i32),
}

#[derive(Debug)]
pub enum Token<'a> {
    Open,
    Close,
    Symbol(Symbol<'a>),
}