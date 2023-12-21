fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let src_path = args.nth(1).ok_or("Not enough args")?;
    let src = std::fs::read_to_string(src_path)?;
    let tokens = rust_lisp_parser::lexer::lex(&src)?;
    let ast = rust_lisp_parser::ast::Tree::try_construct(&tokens)?;
    dbg!(ast);
    Ok(())
}