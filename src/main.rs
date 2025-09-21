use std::env;
use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

mod ast;

lrlex_mod!("coconut.l");
lrpar_mod!("coconut.y");

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = &args[1];
        match from_str(input) {
            Ok(r) => println!("{:?}", r),
            _ => eprintln!("Unable to evaluate input"),
        }
    }
    else {
        println!("Please provide an input string to evaluate");
    }
}

pub fn from_str (input: &String) -> Result<u64, String> {
    let lexer_def = coconut_l::lexerdef();
        let lexer = lexer_def.lexer(&input);
        let (res, errs) = coconut_y::parse(&lexer);

        for e in errs {
            println!("Error: {}", e);
        }

        match res {
            Some(Ok(r)) => eval(r),
            _ => Err("Unable to evaluate input".to_string()),
        }
}

pub fn eval (ast: Vec<ast::Node>) -> Result<u64, String> {
    for node in ast {
        return eval_exp(node);
    }
    return Err(String::from("Couldn't evaluate AST!"));
}

fn eval_exp (exp: ast::Node) -> Result<u64, String> {
    match exp {
        ast::Node::Add {lhs, rhs} => eval_exp(*lhs)?
            .checked_add(eval_exp(*rhs)?)
            .ok_or("Overflow".to_string()),

        ast::Node::Mul {lhs, rhs} => eval_exp(*lhs)?
            .checked_mul(eval_exp(*rhs)?)
            .ok_or("Overflow".to_string()),

        ast::Node::Number {value} => Ok(value)
    }
}

#[test]
fn eval_expressions() {
    assert_eq!(
        from_str(&"1 + 2 * 3 + 4".to_string()).unwrap(),
        11,
        "expected 11"
    );

    assert_eq!(
        from_str(&"(1 + 2) * 3 + 4".to_string()).unwrap(),
        13,
        "expected 13"
    );
}