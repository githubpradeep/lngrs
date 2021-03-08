use crate::code;
use crate::code::Instructions;
use crate::compiler;
use crate::compiler::ByteCode;
use crate::object;
use crate::object::Object;
use iota::iota;

iota! {
    pub const StackSize: usize = 2048;
}

struct VM<'a> {
    pub constants: &'a Vec<Object>,
    pub instructions: &'a Instructions,
    stack: Vec<Object>,
    sp: usize,
}

impl<'a> VM<'a> {
    pub fn new(byte_code: &'a ByteCode) -> Self {
        VM {
            constants: byte_code.constants,
            instructions: byte_code.instructions,
            stack: Vec::new(),
            sp: 0,
        }
    }

    pub fn stack_top(&self) -> &Object {
        if self.sp == 0 {
            return &Object::Null;
        }
        &self.stack[self.sp - 1]
    }

    pub fn run(&mut self) {
        let mut ip = 0;
        while ip < self.instructions.len() {
            let instruction = self.instructions[ip];
            let opcode = instruction as code::OpCode;
            match opcode {
                code::OpConstant => {
                    let number = &self.instructions[ip + 1..];
                    let mut a = [0;8];
                    for (i,v) in number.iter().enumerate() {
                        a[i] = *v;
                    }
                    let constant_index = i64::from_be_bytes(a);
                    self.push(self.constants[constant_index as usize].clone());
                    ip += 2;
                }
                _ => {}
            }
            ip += 1;
        }
    }

    pub fn push(&mut self, object: Object) {
        if self.sp >= StackSize {
            panic!("beyond stacks");
        }
        self.stack.push(object);
        self.sp += 1;
    }
}
