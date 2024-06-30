pub struct MemoryLocation(usize);
impl MemoryLocation {
    pub fn addr(&self) -> usize {
        self.0
    }
}

pub enum Instruction {
    Out(Value),
}

impl Instruction {
    pub fn execute(&self) {
        match self {
            Instruction::Out(v) => println!("{}", v.execute()),
        }
    }
}

pub enum Value {
    ILoad(u8),
    Add(Box<Value>, Box<Value>),
}

impl Value {
    pub fn execute(&self) -> u8 {
        match self {
            Value::ILoad(v) => *v,
            Value::Add(a, b) => a.execute() + b.execute(),
        }
    }
}
