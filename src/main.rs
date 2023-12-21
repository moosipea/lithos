fn main() {

}

#[cfg(test)]
mod tests {
    use rust_lisp_parser::*;
    use rust_lisp_parser::ast::Tree;
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
