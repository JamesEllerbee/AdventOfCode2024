#[derive(Debug)]
pub enum Operation {
    Multiply,
}

#[derive(Debug)]
pub struct Instruction {
    pub operation: Operation,
    pub operands: Vec<i32>,
}

impl Instruction {
    pub fn execute(&self) -> i32 {
        match self.operation {
            Operation::Multiply => {
                if (self.operands.len() > 0) {
                    let mut iter = self.operands.iter();
                    let mut ret = *iter.next().expect("first operand not found");
                    for operand in iter {
                        ret *= *operand;
                    }

                    ret
                } else {
                    0
                }
            }
        }
    }
}
