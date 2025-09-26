use crate::ast::Node;
use crate::instruction::Op;

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
        Node::Declare {id, rhs} => {
            if let Some(val) = rhs {
                ast_to_bytecode(*val, ops);
            }
            ops.push(Op::Declare { name: id.clone() });
        }
        Node::Assign { id, rhs } => {
            ast_to_bytecode(*rhs, ops);
            ops.push(Op::Declare { name: id.clone() });
        }
        Node::Id { value } => ops.push(Op::Load { id: value }),
        Node::PrintLn { rhs } => {
            ast_to_bytecode(*rhs, ops);
            ops.push(Op::PrintLn {})
        }
        Node::Empty {} => {}
    }
}