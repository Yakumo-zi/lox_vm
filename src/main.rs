mod chunk;
mod common;
mod vm;
use anyhow::Result;
use std::io::{self, BufRead, Write};
use vm::VM;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        0 => println!("Usage: lox_vm [path]"),
        1 => repl(),
        2 => {
            let path = &args[1];
            run_file(path);
        }
        _ => println!("Too many arguments"),
    }
    Ok(())
}

fn repl() {
    let mut vm = VM::new();
    let mut reader = io::BufReader::new(io::stdin());
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        if let Ok(bytes) = reader.read_line(&mut input) {
            if bytes > 0 {
               
            }
        }
    }
}

fn run_file(path: &str) {}
