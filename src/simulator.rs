use crate::ast::Tree;
use crate::lexer::Symbol;
use crate::Error;

use std::collections::HashMap;

use anyhow::Result;

#[derive(Debug, Clone)]
pub enum Ast<'a> {
    NumberLiteral(i32),
    StringLiteral(String),
    Identifier(String),
    Call { name: &'a str, args: Vec<Ast<'a>> },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Signed32(i32),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Signed32(n) => write!(f, "{}", *n),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

pub type Stack = Vec<Value>;

impl Op {
    pub fn eval(&self, arg_count: usize, stack: &mut Stack) -> Result<()> {
        let values = (1..=arg_count)
            .map(|i| {
                stack
                    .pop()
                    .ok_or(Error::UnexpectedArgN(arg_count, i).into())
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(|value| match value {
                Value::Signed32(n) => Ok(n),
            })
            .collect::<Result<Vec<_>>>()?;

        let op = match self {
            Op::Add => i32::wrapping_add,
            Op::Sub => i32::wrapping_sub,
            Op::Mul => i32::wrapping_mul,
            Op::Div => i32::wrapping_div,
        };

        let result = values
            .into_iter()
            .reduce(op)
            .ok_or(Error::Expected(".reduce() not to fail"))?;

        stack.push(Value::Signed32(result));

        Ok(())
    }
}

#[derive(Clone)]
pub enum Function {
    Builtin {
        name: String,
        inner: fn(&mut Stack) -> Result<()>,
    },
    User {
        name: String,
        args: Vec<String>,
        bytecode: Vec<Instruction>,
    },
}

impl Function {
    pub fn eval(
        &self,
        stack: &mut Stack,
        variables: &mut Variables,
        functions: &mut Functions,
    ) -> Result<()> {
        match self {
            Function::Builtin { inner, .. } => inner(stack),
            Function::User { args, bytecode, .. } => {
                // FIXME
                let prev_state = variables.clone();
                for (i, arg) in args.into_iter().rev().enumerate() {
                    let value = stack.pop().ok_or(Error::UnexpectedArgN(args.len(), i))?;
                    variables.insert(arg.to_string(), value);
                }
                interpert(&bytecode, stack, variables, functions)?;
                *variables = prev_state;
                Ok(())
            }
        }
    }
    fn name(&self) -> &str {
        match self {
            Function::Builtin { name, .. } => name,
            Function::User { name, .. } => name,
        }
    }
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Function::Builtin { name, .. } => write!(f, "Builtin function '{name}'"),
            Function::User { name, .. } => write!(f, "User-defined function '{name}'"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Load(Value),
    Operation(Op, usize),
    Call(String),
    ReadVar(String),
    StoreVar(String),
    ForgetVar(String),
    DefineFunction(Function),
}

impl Instruction {
    fn store_var(&self) -> Option<&str> {
        match self {
            Self::StoreVar(s) => Some(s),
            _ => None,
        }
    }
}

fn builtin_echo(stack: &mut Stack) -> Result<()> {
    let a = stack.pop().ok_or(Error::UnexpectedArgN(1, 0))?;
    println!("{}", a);
    Ok(())
}

fn create_user_function(args: &[Ast]) -> Result<Instruction> {
    let mut args = args.into_iter();
    let name = args
        .next()
        .ok_or(Error::UnexpectedArgN(3, 0))?
        .ident()
        .ok_or(Error::Expected("identifier"))?
        .to_string();

    let fn_args = args
        .next()
        .ok_or(Error::UnexpectedArgN(3, 1))?
        .call_list()
        .ok_or(Error::Expected("all arguments to be identifiers"))?;

    let body = args.next().ok_or(Error::UnexpectedArgN(3, 2))?.generate()?;

    Ok(Instruction::DefineFunction(Function::User {
        name: name.clone(),
        args: fn_args,
        bytecode: body,
    }))
}

fn match_call<'a>(name: &'a str, args: &'a [Ast]) -> Result<Instruction> {
    Ok(match name {
        "+" => Instruction::Operation(Op::Add, args.len()),
        "-" => Instruction::Operation(Op::Sub, args.len()),
        "*" => Instruction::Operation(Op::Mul, args.len()),
        "/" => Instruction::Operation(Op::Div, args.len()),
        "let" => Instruction::StoreVar(match &args[0] {
            Ast::Identifier(ident) => ident.to_string(),
            _ => return Err(Error::Expected("identifier").into()),
        }),
        "fn" => create_user_function(args)?,
        _ => Instruction::Call(name.to_string()),
    })
}

fn push_let_in(
    instruction: Instruction,
    args: &[Ast],
    instructions: &mut Vec<Instruction>,
) -> Result<()> {
    let name = instruction.store_var().unwrap().to_string();
    instructions.extend(args[1].generate()?); // Push variable value
    instructions.push(instruction); // Push Store
    instructions.extend(args[2].generate()?); // Push Expression
    instructions.push(Instruction::ForgetVar(name)); // Forget the variable
    Ok(())
}

fn push_normal_instruction(
    instruction: Instruction,
    args: &[Ast],
    instructions: &mut Vec<Instruction>,
) -> Result<()> {
    for arg in args.into_iter().rev() {
        let bytecode = arg.generate()?;
        instructions.extend(bytecode);
    }
    instructions.push(instruction);
    Ok(())
}

fn make_call(name: &str, args: &[Ast], instructions: &mut Vec<Instruction>) -> Result<()> {
    let instruction = match_call(name, args)?;
    match instruction {
        Instruction::StoreVar(_) => push_let_in(instruction, args, instructions)?,
        Instruction::DefineFunction(_) => instructions.push(instruction),
        _ => push_normal_instruction(instruction, args, instructions)?,
    }

    Ok(())
}

impl<'a> Ast<'a> {
    pub fn from_tree(tree: &'a Tree) -> Result<Self> {
        match tree {
            Tree::Branch(_) => ast_from_branch(tree),
            Tree::Leaf(_) => ast_from_leaf(tree),
        }
    }

    pub fn generate(&self) -> Result<Vec<Instruction>> {
        let mut instructions = Vec::new();
        match self {
            Ast::NumberLiteral(n) => instructions.push(Instruction::Load(Value::Signed32(*n))),
            Ast::Identifier(ident) => instructions.push(Instruction::ReadVar(ident.clone())),
            Ast::Call { name, args } => make_call(name, args, &mut instructions)?,
            _ => return Err(Error::Unimplemented("ast variant").into()),
        }
        Ok(instructions)
    }

    fn ident(&self) -> Option<&str> {
        match self {
            Self::Identifier(ident) => Some(ident),
            _ => None,
        }
    }

    fn call_list(&self) -> Option<Vec<String>> {
        match self {
            Self::Call { name, args } => {
                let mut accum = vec![name.to_string()];
                for arg in args {
                    match arg {
                        Self::Identifier(ident) => accum.push(ident.to_string()),
                        _ => return None,
                    }
                }
                Some(accum)
            }
            _ => None,
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

fn register_builtin(functions: &mut Functions, name: &str, f: fn(&mut Stack) -> Result<()>) {
    functions.insert(
        name.to_string(),
        Function::Builtin {
            name: name.to_string(),
            inner: f,
        },
    );
}

type Functions = HashMap<String, Function>;
type Variables = HashMap<String, Value>;

fn interpert(
    bytecode: &[Instruction],
    stack: &mut Stack,
    variables: &mut Variables,
    functions: &mut Functions,
) -> Result<()> {
    for instruction in bytecode {
        match instruction {
            Instruction::Load(value) => stack.push(value.clone()),
            Instruction::Operation(op, arg_count) => op.eval(*arg_count, stack)?,
            Instruction::Call(func_name) => {
                let f = functions
                    .get(func_name)
                    .cloned() // AAGGH this defeats the whole point
                    .ok_or(Error::UnknownFunction(func_name.to_string()))?;
                f.eval(stack, variables, functions)?;
            }
            Instruction::ReadVar(name) => {
                stack.push(variables.get(name).expect("Unknown variable").clone())
            }
            Instruction::StoreVar(name) => {
                variables.insert(
                    name.to_string(),
                    stack.pop().ok_or(Error::Expected("nonempty stack"))?,
                );
            }
            Instruction::ForgetVar(name) => {
                variables.remove(name);
            }
            Instruction::DefineFunction(func) => {
                functions.insert(func.name().to_string(), func.clone());
            }
        }
    }
    Ok(())
}

pub fn run(bytecode: Vec<Instruction>) -> Result<()> {
    let mut stack = Stack::new();

    let mut variables = Variables::new();
    let mut functions = Functions::new();

    register_builtin(&mut functions, "echo", builtin_echo);

    interpert(&bytecode, &mut stack, &mut variables, &mut functions)
}
