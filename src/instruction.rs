use std::vec;

use crate::ast_to_bytecode;
use crate::ast::Node;
use crate::scope::Scope;


#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add, 
    Mull,
    Push {value: u64},
    Assign {name: String},
    Declare {name: String},
    PrintLn,
    Load {id: String},
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Mul,
    Assign {name: String},
    Declare {name: String},
}

pub fn eval (ast: Vec<Node>, scope: &mut Scope) -> Result<Option<u64>, String> {
    let ops = &mut vec![];

    for a in ast {
        ast_to_bytecode(a, ops);
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
            Op::Mull {} => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs * rhs);
            }
            Op::Assign {name} => {
                let val = stack.pop().unwrap();
                scope.set_var(name.clone(), val);
            }
            Op::Declare {name} => {
                let val = stack.pop().unwrap();
                scope.dec_var(name.clone(), val);
            }
            Op::PrintLn => {
                println!("{}", stack.pop().unwrap());
            }
            Op::Load {id} => {
                if let Some(value) = scope.get_var(id.clone()) {
                    stack.push(value.clone());
                } else {
                    return Err(format!("Variable '{}' not found", id.clone()))
                }
            }
        }
    }

    return Ok(stack.pop());
}