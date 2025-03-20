mod common;
mod vm;
mod chunk;
use chunk::OpCode;
use anyhow::Result;
use vm::VM;
fn main()->Result<()> {
    let mut chunk =chunk::Chunk::new();
    let idx = chunk.add_constant(10.0);
    chunk.write(OpCode::Constant(idx),123);
    chunk.write(OpCode::Negate, 123);
    let idx=chunk.add_constant(20.0);
    chunk.write(OpCode::Constant(idx), 123);
    chunk.write(OpCode::Add, 123);
    chunk.write(OpCode::Return,123);
    #[cfg(feature = "debug-trace")]
    {
         chunk.disassemble("test chunk")?;
    }
    let mut vm = VM::new();
    vm.interpret(chunk)?;
    Ok(())
}
