use std::{env, fs, io::{stdin, stdout, Write}};
use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

mod ast;
mod instruction;
mod scope;
mod bytecode;
use instruction::eval;
use bytecode::ast_to_bytecode;

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
            run(&input);
        }
    }
    else {
        repl();
    }
}

fn eval_file(input: String) {
    match fs::read_to_string(input) {
        Ok(content) => {
            run(&content);
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
                run(&input);
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
        let mut mainScope = scope::Scope::new();

        match res {
            Some(Ok(r)) => eval(r, &mut mainScope),
            _ => Err("Unable to evaluate input".to_string()),
        }
}

pub fn run (input: &String) {
    match from_str(input) {
        Ok(Some(result)) => println!("{}", result),
        _ => println!("Error: {}", "Unable to evaluate input"),
    }
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

#[test]
fn vars_declare_math () {
    assert_eq!(
        from_str(&"let x = 1; let y = 2; y+x;".to_string()).unwrap(),
        Some(3)
    )
}

#[test]
fn vars_declare_reassign_math () {
    assert_eq!(
        from_str(&"let x = 1; let y = 2; x=3; y+x;".to_string()).unwrap(),
        Some(5)
    )
}