#![allow(unused)]
use std::ops::Neg;

use crate::{
    chunk::{Chunk, OpCode}, common::Value, compiler::Compiler, scanner::Scanner
};
use anyhow::{Result, anyhow};
pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::default(),
            ip: 0,
            stack: Vec::new(),
        }
    }
    pub fn interpret(&mut self, source: &str) -> Result<InterpretResult> {
        let mut compiler=Compiler::new(source);
        let res=compiler.compile();
        match res {
            Ok(chunk) => {
                self.chunk=chunk
            },
            Err(err) => {
                return Ok(InterpretResult::CompileError)
            },
        }
        return Ok(InterpretResult::Ok)
    }
    fn run(&mut self) -> Result<InterpretResult> {
        loop {
            let op = self.chunk.code[self.ip];
            self.ip += 1;
            #[cfg(feature = "debug-trace")]
            {
                println!();
                for val in &self.stack {
                    println!("[{:}]", val)
                }
                println!();
                println!("{}", self.chunk.disassemble_op_code(&op)?);
            }
            match op {
                OpCode::Return => {
                    if let Some(v) = self.stack.pop() {
                        println!("{:>4}", v);
                        return Ok(InterpretResult::Ok);
                    }
                    return Ok(InterpretResult::RuntimeError);
                }
                OpCode::Constant(v) => self.stack.push(self.chunk.constants.get_constants(v)?),
                OpCode::Negate => {
                    self.stack
                        .last_mut()
                        .ok_or_else(|| anyhow!("Stack overflow during negation"))
                        .map(|last| *last = last.neg())?;
                }
                OpCode::Add => self.binary_op(op)?,
                OpCode::Subtract => self.binary_op(op)?,
                OpCode::Multiply => self.binary_op(op)?,
                OpCode::Divide => self.binary_op(op)?,
            }
        }
    }
    fn binary_op(&mut self, op: OpCode) -> Result<()> {
        let (left, right) = self
            .stack
            .pop()
            .and_then(|right| self.stack.pop().map(|left| (left, right)))
            .ok_or_else(|| anyhow!("Stack overflow"))?;
        match op {
            OpCode::Add => {
                self.stack.push(left + right);
            }
            OpCode::Subtract => {
                self.stack.push(left - right);
            }
            OpCode::Multiply => {
                self.stack.push(left * right);
            }
            OpCode::Divide => {
                self.stack.push(left / right);
            }
            _ => return Err(anyhow!("Unknown binary opcode")),
        }
        return Ok(());
    }
}
