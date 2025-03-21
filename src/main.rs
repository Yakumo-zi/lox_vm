mod chunk;
mod common;
mod vm;
mod scanner;
mod compiler;
use anyhow::Result;
use std::io::{self, BufRead, Write};
use vm::VM;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        0 => println!("Usage: lox_vm [path]"),
        1 => repl()?,
        2 => {
            let path = &args[1];
             run_file(path)?;
        }
        _ => println!("Too many arguments"),
    }
    Ok(())
}

fn repl() -> Result<()> {
    let mut vm = VM::new();
    let mut reader = io::BufReader::new(io::stdin());
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;  // Handle flush errors

        match reader.read_line(&mut input) {
            Ok(bytes) if bytes > 0 => {
                vm.interpret(&input)?;
                input.clear();
            }
            Ok(_) => break,  // Empty input (Ctrl+D)
            Err(e) => return Err(anyhow::anyhow!("Error reading input: {}", e)),
        }
    }
    Ok(())
}

fn run_file(path: &str) -> Result<()> {
    let contents = std::fs::read_to_string(path)?;
    let mut vm = VM::new();
    vm.interpret(&contents)?;
    Ok(())
}
