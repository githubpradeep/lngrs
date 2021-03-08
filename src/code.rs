use std::collections::HashMap;

use iota::iota;

pub type Instructions = Vec<u8>;

pub type OpCode = u8;

iota! {
    pub const OpConstant: OpCode = iota;
             ,OpAdd
}

pub struct Definition {
    pub name: String,
    pub operand_widths: Vec<i32>,
}

pub struct Definitions {
    pub definitions: HashMap<OpCode, Definition>,
}

impl Definitions {
    pub fn new() -> Self {
        let mut definitions = HashMap::new();
        let constant_definition = Definition {
            name: "OpConstant".to_string(),
            operand_widths: vec![2],
        };
        definitions.insert(OpConstant, constant_definition);

        let add_definition = Definition {
            name: "OpAdd".to_string(),
            operand_widths: vec![],
        };
        definitions.insert(OpAdd, add_definition);

        Definitions { definitions }
    }

    pub fn lookup(&self, opcode: OpCode) -> Option<&Definition> {
        self.definitions.get(&opcode)
    }

    pub fn make(&self, opcode: OpCode, operands: Vec<i64>) -> Instructions {
        let definition = self.definitions.get(&opcode);
        let mut instructions = vec![];
        if definition.is_none() {
            return instructions;
        }
        let definition = definition.unwrap();
        let mut instruction_len = 1;
        for width in definition.operand_widths.iter() {
            instruction_len += width;
        }

        instructions.push(opcode);
        for (index, operand) in operands.iter().enumerate() {
            let width = definition.operand_widths[index];
            match width {
                2 => {
                    let operand = *operand as u16;
                    let bytes = operand.to_be_bytes();
                    instructions.append(&mut bytes.to_vec());
                }
                _ => {}
            }
        }

        instructions
    }
}
