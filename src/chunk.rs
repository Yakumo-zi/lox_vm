pub enum OpCode {
    OpReturn
}
impl OpCode{
    pub fn disassemble(&self)->&str{
        use OpCode::*;
        match self {
            OpReturn=>{
                "OpReturn"
            }
        }
    }
}

pub struct  Chunk{
    pub code:Vec<OpCode>
}

impl Chunk{
    pub fn new()->Chunk{
        Chunk { code: Vec::new() }
    }
    pub fn write(&mut self,op_code:OpCode){
        self.code.push(op_code);
    }
    pub fn disassemble(&self,name:&str){
        println!("== {} ==",name);
        for (idx,op_code) in self.code.iter().enumerate(){
            println!("{:0>4} {}",idx,op_code.disassemble())
        }

    }

    
}