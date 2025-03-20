mod common;
mod chunk;
use chunk::OpCode;
use anyhow::Result;
fn main()->Result<()> {
    let mut chunk =chunk::Chunk::new();
    let idx = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant(idx));
    chunk.write(OpCode::Return);
    chunk.disassemble("test chunk")?;
    Ok(())
}
