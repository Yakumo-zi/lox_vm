mod common;
mod chunk;
fn main() {
    let mut chunk =chunk::Chunk::new();
    chunk.write(chunk::OpCode::OpReturn);
    chunk.disassemble("test chunk");
}
