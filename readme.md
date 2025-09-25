# Interpreter in rust
Following along all parts of https://medium.com/@p3ld3v/writing-interpreter-in-rust-using-grmtools-7a6a0458b99f

## Why?
As a learning exercise.

## How to use
Run `cargo -q run "expression_string"`

Examples:
### 1. Pass as argument
```
cargo -q run "2+2"
```

### 2. Pass a file containing expression
```
cargo run test.cnt
```