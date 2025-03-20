use crate::{chunk::{Chunk, OpCode}, common::Value};
use anyhow::{Ok, Result};
const MAX_STACK_SIZE:usize = 256;
pub struct VM {
    chunk:Chunk,
    ip:usize,
    stack:Vec<Value>,
    top:usize,
}
pub enum InterpretResult{
    Ok,
    CompileError,
    RuntimeError,
}

impl VM{
    pub fn new()->VM{
        VM { chunk: Chunk::new(),ip:0,stack:Vec::new(),top:0}
    }
    pub fn interpret(&mut self,chunk:Chunk)->Result<InterpretResult>{
        self.chunk=chunk;
        self.ip=0;
        let ret=self.run()?;
        Ok(ret)
    }
    fn run(&mut self)->Result<InterpretResult>{
        loop{
            let op = self.chunk.code[self.ip];
            self.ip+=1;
            #[cfg(feature = "debug-trace")]
            {
                println!("{}",self.chunk.disassemble_op_code(&op)?);
            }
            match op {
                OpCode::Return => {
                    return Ok(InterpretResult::Ok)
                },
                OpCode::Constant(v) => {
                    println!("{:<4}",self.chunk.constants.get_constants(v)?);
                },
            }
        }
    }
}