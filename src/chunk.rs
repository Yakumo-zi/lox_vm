

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
    pub lines:Vec<i32>,
    pub constants:ValueArray
}

impl Chunk{
    pub fn new()->Chunk{
        Chunk { code: Vec::new() ,lines:Vec::new(), constants:ValueArray::new()}
    }
    pub fn write(&mut self,op_code:OpCode,line:i32){
        self.code.push(op_code);
        self.lines.push(line);
    }
    pub fn add_constant(&mut self,value:Value)->usize{
        self.constants.write(value)
    }
    pub fn disassemble(&self,name:&str)->Result<()>{
        println!("== {} ==",name);
        for (idx,op_code) in self.code.iter().enumerate(){
            let mut sep = self.lines[idx].to_string();
            if idx!=0 && self.lines[idx]==self.lines[idx-1]{
                sep="|".to_string()
            }
            println!("{:0>4} {:>4} {:}",idx,sep,self.disassemble_op_code(op_code)?)
        }
        Ok(())
    }
    fn disassemble_op_code(&self,op_code:&OpCode)->Result<String>{
        use OpCode::*;
        match op_code {
            Return=>{
                Ok(format!("{:<12}","OpReturn"))
            },
            Constant(idx)=>{
                let constant=self.constants.get_constants(*idx)?;
                Ok(format!("{:<12} {:<4} '{:}'","OpConstant",idx,constant))
            }
        }
    }

    
}