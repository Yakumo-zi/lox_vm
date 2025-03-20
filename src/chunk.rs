

use crate::common::ValueArray;
use crate::common::Value;
use anyhow::Ok;
use anyhow::Result;
pub enum OpCode {
    Return,
    Constant(usize),
}

pub struct  Chunk{
    pub code:Vec<OpCode>,
    pub constants:ValueArray
}

impl Chunk{
    pub fn new()->Chunk{
        Chunk { code: Vec::new() ,constants:ValueArray::new()}
    }
    pub fn write(&mut self,op_code:OpCode){
        self.code.push(op_code);
    }
    pub fn add_constant(&mut self,value:Value)->usize{
        self.constants.write(value)
    }
    pub fn disassemble(&self,name:&str)->Result<()>{
        println!("== {} ==",name);
        for (idx,op_code) in self.code.iter().enumerate(){
            println!("{:0>4} {}",idx,self.disassemble_op_code(op_code)?)
        }
        Ok(())
    }
    fn disassemble_op_code(&self,op_code:&OpCode)->Result<String>{
        use OpCode::*;
        match op_code {
            Return=>{
                Ok("OpReturn".to_string())
            },
            Constant(idx)=>{
                let constant=self.constants.get_constants(*idx)?;
                Ok(format!("OpConstant {}",constant))
            }
        }
    }

    
}