mod common;
mod chunk;
use chunk::OpCode;
use anyhow::Result;
fn main()->Result<()> {
    let mut chunk =chunk::Chunk::new();
    let idx = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant(idx),123);
    chunk.write(OpCode::Return,123);
    chunk.disassemble("test chunk")?;
    Ok(())
}
