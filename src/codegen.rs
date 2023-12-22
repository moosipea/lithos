use std::error::Error;
use crate::ast::Tree;

trait PushLine {
    fn push_line(&mut self, s: &str);
}

impl PushLine for String {
    fn push_line(&mut self, s: &str) {
        self.push_str(s);
        self.push('\n');
    }
}

fn prologue(s: &mut String) {
    s.push_line("section .text");
    s.push_line("global _start");
    s.push_line("_start:")
}

pub fn codegen(ast: Tree) -> Result<String, Box<dyn Error>> {
    let mut res = String::new();
    prologue(&mut res);

    Ok(res)
}