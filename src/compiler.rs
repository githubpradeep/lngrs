use crate::code;
use crate::{
    ast::{Expression, Program, Statement},
    code::{Definition, Definitions, Instructions, OpCode},
    object::Object,
    parser::Error,
};
pub struct Compiler {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
    pub definitions: Definitions,
}

pub struct ByteCode<'a> {
    pub instructions: &'a Instructions,
    pub constants: &'a Vec<Object>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            instructions: vec![],
            constants: vec![],
            definitions: Definitions::new(),
        }
    }

    pub fn compile_program(&mut self, program: Program) -> Result<(), Error> {
        for statement in program.statements.iter() {
            self.compile_statement(statement)?;
        }
        Ok(())
    }

    pub fn compile_statement(&mut self, statement: &Statement) -> Result<(), Error> {
        match statement {
            Statement::ExpressionStatement {
                token: _,
                expression,
            } => self.compile_expression(expression),
            _ => Err("not yet impl".to_string()),
        }
    }

    pub fn compile_expression(&mut self, expression: &Expression) -> Result<(), Error> {
        match expression {
            Expression::InfixExpression {
                left,
                operator,
                right,
                token: _,
            } => {
                self.compile_expression(left.as_ref())?;
                self.compile_expression(right.as_ref())?;
                match operator.as_str() {
                    "+" => self.emit(code::OpAdd, vec![]),
                    _ => {
                        return Err("not yet impl!".to_string());
                    }
                };
                Ok(())
            }
            Expression::IntegerLiteral { token: _, value } => {
                let value = Object::Integer { value: *value };
                let operands = vec![self.add_constant(value) as i64];
                self.emit(code::OpConstant, operands);
                Ok(())
            }
            _ => Err("not yet impl".to_string()),
        }
    }

    pub fn byte_code(&self) -> ByteCode {
        ByteCode {
            instructions: &self.instructions,
            constants: &self.constants,
        }
    }

    fn add_constant(&mut self, object: Object) -> usize {
        self.constants.push(object);
        self.constants.len() - 1
    }

    pub fn emit(&mut self, opcode: OpCode, operands: Vec<i64>) -> usize {
        let mut instruction = self.definitions.make(opcode, operands);
        let position = self.add_instruction(&mut instruction);
        position
    }

    pub fn add_instruction(&mut self, instruction: &mut Instructions) -> usize {
        let position = self.instructions.len();
        self.instructions.append(instruction);
        position
    }
}
