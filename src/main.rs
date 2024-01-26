use anyhow::Result;
use rust_lisp_parser::lexer::*;

fn main() -> Result<()> {
    let src = "(add 34 35)";
    let code = remove_comments(src);   
    let lexer = Lexer::new(&code);
    let tokens: Vec<_> = lexer.collect();

    for token in tokens {
	println!("{token:?}");
    }
    
    Ok(())
}
