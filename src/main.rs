mod evaluator;
mod expr;
mod parser;

use std::io::{self, Write};

fn main() {
    println!("Welcome to Rust Lisp, Ruli!");

    loop {
        print!("ruli> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        println!("Input: {}", input.trim());
    }
}
