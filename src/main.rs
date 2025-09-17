use std::env;
use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

lrlex_mod!("coconut.l");
lrpar_mod!("coconut.y");

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = &args[1];
        let lexer_def = coconut_l::lexerdef();
        let lexer = lexer_def.lexer(&input);
        let (res, errs) = coconut_y::parse(&lexer);

        for e in errs {
            println!("Error: {}", e);
        }

        match res {
            Some(Ok(r)) => println!("{:?}", r),
            _ => eprintln!("Unable to evaluate input"),
        }
    }
    else {
        println!("Please provide an input string to evaluate");
    }
}
