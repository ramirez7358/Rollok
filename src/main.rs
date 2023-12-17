use std::io;
use crate::repl::start;

mod lexer;
mod lexer2;
mod token;
mod repl;
mod ast;
mod parser;

fn main() {
    println!("Hello! This is the Rollok Programming Language!");
    start(io::stdin(), io::stdout());
}
