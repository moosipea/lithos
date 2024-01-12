use std::collections::HashMap;

struct StringBuilder(String);
impl StringBuilder {
    fn empty() -> Self {
        Self(String::new())
    }
    fn line(mut self, line: &str) -> Self {
        self.0.push_str(line);
        self.0.push('\n');
        self
    }

    fn finish(self) -> String {
        self.0
    }
}

enum Op {
    BinaryAdd,
    BinarySub,
    Load(u64),
}

impl Op {
    fn asm(&self) -> String {
        match self {
            Op::BinaryAdd => StringBuilder::empty()
                .line("pop rdx")
                .line("pop rax")
                .line("add rax, rdx")
                .finish(),
            Op::BinarySub => StringBuilder::empty()
                .line("pop rdx")
                .line("pop rax")
                .line("sub rax, rdx")
                .finish(),
            Op::Load(value) => format!("push {value}\n"),
        }
    }
}

pub fn codegen() -> String {
    let sample = &[Op::Load(34), Op::Load(35), Op::BinaryAdd];

    sample
        .into_iter()
        .map(Op::asm)
        .reduce(|result, chunk| format!("{result}{chunk}"))
        .unwrap()
}
