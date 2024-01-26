use anyhow::Result;
use rust_lisp_parser::lexer::*;
use std::ops::RangeInclusive;

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

fn report_error(tokens: &[Token], range: RangeInclusive<usize>, code: &str) {
    let sub = tokens.get(range).expect("Expected valid range");
    let line = sub[0].pos.line;    
    let prefix = format!("Error on line {line}: ");
    let text = code.lines().nth(line);
    println!("{prefix}{text}");
}

fn main() -> Result<()> {
    let src = read_one_of(&[
	"test.pj",
	"../test.pj"
    ]).expect("Expected to read file");
    
    let code = preprocess(&src);   
    let lexer = Lexer::new(&code);
    let tokens: Vec<_> = lexer.collect();

    report_error(&tokens, 1..=1, &code);
    
    Ok(())
}
