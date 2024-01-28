use rust_lisp_parser::ast::*;
use rust_lisp_parser::lexer::*;

fn read_one_of(paths: &[&str]) -> Option<String> {
    use std::fs;
    for path in paths {
        match fs::read_to_string(path) {
            Ok(content) => return Some(content),
            Err(_) => {}
        }
    }
    None
}

fn main() {
    let src = read_one_of(&["test.pj", "../test.pj"]).expect("Expected to read file");
    let code = preprocess(&src);
    println!("Source:\n{}\n", &code);

    let tokens: Vec<_> = Scanner::new(&code).evaluate().collect();
    let tree = build_syntax_tree(&tokens);
    println!("{tree:#?}");
}
