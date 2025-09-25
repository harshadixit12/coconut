use std::{env, fs, io::{stdin, stdout, Write}};
use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

mod ast;
mod instruction;
use ast::Node;
use instruction::Op;

lrlex_mod!("coconut.l");
lrpar_mod!("coconut.y");

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = &args[1];
        if input.ends_with(".cnt".clone()) {
            eval_file(args[1].clone());
        }
        else {
            eval(&input);
        }
    }
    else {
        repl();
    }
}

fn eval_file(input: String) {
    match fs::read_to_string(input) {
        Ok(content) => {
            eval(&content);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}

pub fn repl () {
    loop {
        print!(">");
        stdout().flush().unwrap();
        match stdin().lines().next() {
            Some(Ok(input)) => {
                if input.trim() == "exit" {
                    break;
                }
                if input.trim().is_empty() {
                    continue;
                }
                eval(&input);
            }
            _ => {}
        }
    }
}

pub fn from_str (input: &String) -> Result<Option<u64>, String> {
    let lexer_def = coconut_l::lexerdef();
        let lexer = lexer_def.lexer(&input);
        let (res, errs) = coconut_y::parse(&lexer);

        for e in errs {
            println!("Error: {}", e);
        }

        match res {
            Some(Ok(r)) => Ok(eval_bytecode(r)),
            _ => Err("Unable to evaluate input".to_string()),
        }
}

pub fn eval (input: &String) {
    match from_str(input) {
        Ok(Some(result)) => println!("{}", result),
        _ => println!("Error: {}", "Unable to evaluate input"),
    }
}

pub fn ast_to_bytecode (node: Node, ops: &mut Vec<Op>) {
    match node {
        Node::Add { lhs, rhs } => {
            ast_to_bytecode(*lhs, ops);
            ast_to_bytecode(*rhs, ops);
            ops.push(Op::Add{});
        }
        Node::Mul {lhs, rhs} => {
            ast_to_bytecode(*lhs, ops);
            ast_to_bytecode(*rhs, ops);
            ops.push(Op::Mull{});
        }
        Node::Number {value} => {
            ops.push(Op::Push{value});
        }
    }
}

pub fn eval_bytecode (ast: Vec<Node>) -> Option<u64> {
    let ops = &mut vec![];

    for node in ast {
        ast_to_bytecode(node, ops);
    }

    let mut stack: Vec<u64> = vec![];

    for instruction in ops {
        match instruction {
            Op::Push { value } => stack.push(*value),
            Op::Add => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs + rhs);
            }
            Op::Mull => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs * rhs);
            }
        }
    }

    return stack.pop();

}

#[test]
fn eval_expressions() {
    assert_eq!(
        from_str(&"1 + 2 * 3 + 4".to_string()).unwrap(),
        Some(11),
        "expected 11"
    );

    assert_eq!(
        from_str(&"(1 + 2) * 3 + 4".to_string()).unwrap(),
        Some(13),
        "expected 13"
    );
}

#[test]
fn eval_comments() {
    assert_eq!(
        from_str(&"// 2+2\n5+7".to_string()).unwrap(),
        Some(12),
        "expected 12"
    );
}