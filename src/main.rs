use crate::repl::start;
use std::io;

mod ast;
mod lexer;
mod lexer2;
mod parser;
mod repl;
mod token;

fn main() {
    println!("Hello! This is the Rollok Programming Language!");

    start(io::stdin(), io::stdout());
}
