#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add, 
    Mull,
    Push {value: u64},
}