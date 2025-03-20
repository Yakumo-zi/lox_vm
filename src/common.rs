use anyhow::Result;

pub type Value=f64;

pub struct  ValueArray {
    values:Vec<Value>
}

impl ValueArray{
    pub fn new()->ValueArray{
        ValueArray { values: Vec::new() }
    }
    pub fn write(&mut self,value:Value)->usize{
        self.values.push(value);
        self.values.len()-1
    }
    pub fn get_constants(&self,idx:usize)->Result<Value>{
        if idx>=self.values.len(){
            return Err(anyhow::anyhow!("index out of range {}",idx));
        }
         Ok(self.values[idx])
    }
}

