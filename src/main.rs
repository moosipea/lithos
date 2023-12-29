fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let path = args.nth(1).ok_or("Not enough args")?;
    let content = std::fs::read_to_string(path)?;
    
    let tokens = rust_lisp_parser::lexer::lex(&content)?;
    let tree = rust_lisp_parser::ast::Tree::try_construct(&tokens)?;
    
    let _ = rust_lisp_parser::codegen::make_ast_token(tree)?;

    Ok(())
}
