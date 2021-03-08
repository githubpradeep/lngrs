

use std::{cell::RefCell, collections::HashMap};
use std::rc::Rc;

use crate::{ast::{Expression, Program, Statement}, environment::Environment, object::{Object}};

pub trait Evaluator {
    fn evaluate(&self, environment: Rc<RefCell<Environment>>) -> Object;
}

fn evaluate_prefix_expression(operator: &String, right: Object) -> Object {
    match operator.as_str() {
        "!" => evaluate_bang_operator_expression(right),
        "-" => evaluate_minus_prefix_operator_expression(right),
        _ => Object::Null
    }
}

fn evaluate_minus_prefix_operator_expression(right: Object) -> Object {
    if let Object::Integer{value} = right {
        return Object::new_int(-value)
    }
    Object::Null
}

fn evaluate_bang_operator_expression(right: Object) -> Object {
    match right {
        Object::Boolean{
            value
        } => {
            Object::new_bool(!value)
        },
        Object::Null => Object::new_bool(false),
        _ => Object::new_bool(false)
    }
}

fn evaluate_infix_expression(operator: &String, left: Object, right: Object) -> Object{
    if let  Object::Integer{value: left} = left {
        if let  Object::Integer{value: right} = right {
            match operator.as_str() {
                "+" => return Object::new_int(left+right),
                "-" => return Object::new_int(left-right),
                "*" => return Object::new_int(left*right),
                "/" => return Object::new_int(left/right),
                "<" => return Object::new_bool(left < right),
                ">" => return Object::new_bool(left > right),
                "<=" => return Object::new_bool(left <= right),
                ">=" => return Object::new_bool(left >= right),
                "==" => return Object::new_bool(left == right),
                "!=" => return Object::new_bool(left != right),
               _=> return Object::Null
            } 
        }
    } else if let  Object::Boolean{value: left} = left {
        if let  Object::Boolean{value: right} = right {
            match operator.as_str() {
                "==" => return Object::new_bool(left == right),
                "!=" => return Object::new_bool(left != right),
               _=> return Object::Null
            } 
        }
    } else if let  Object::String{value: left} = left {
        if let  Object::String{value: right} = right {
            match operator.as_str() {
                "+" => {
                    let mut s = left.to_owned();
                    s.push_str(right.as_str());
                    return Object::new_string(s)
                }

               _=> return Object::Null
            } 
        }
    }

    return Object::Null;
    
}

fn is_truthy(object: Object) -> bool {
    match object {
        Object::Boolean{value} => {
            value
        },
        _ => false
    }
}

fn eval_if_expression(condition: &Box<Expression> ,
     consequence: &Box<Statement>,
      alternative: &Option<Box<Statement>>,
      environment: Rc<RefCell<Environment>>) -> Object {
    let condition = condition.evaluate(environment.clone());
    if is_truthy(condition) {
        return consequence.evaluate(environment.clone());
    } else if alternative.is_some() {
        return alternative.as_ref().unwrap().evaluate(environment);
    }
    Object::Null
}

fn eval_identifier(name: &String, environment: Rc<RefCell<Environment>>) -> Object {
    if let Some(value) = environment.borrow_mut().get(&name) {
        return value.clone();
    }
    Object::Null
} 

fn eval_expressions(arguments: &Vec<Expression>, environment: Rc<RefCell<Environment>>) -> Vec<Object> {
    let mut expressions = vec!();
    for arg in arguments {
        let res = arg.evaluate(environment.clone());
        expressions.push(res);
    }
    expressions
}

fn apply_function(function: Object, args: Vec<Object>) -> Object {
    match function {
        Object::Builtin(builtin) => {
               (builtin.as_ref().func)(args)
        }
        Object::Function {
            parameters,
            body,
            environment
        } => {
            let child = environment.clone().borrow().clone().create_child();
            let environment = Rc::new(RefCell::new(child));

            for (idx, param) in parameters.iter().enumerate() {
                if let Expression::Identifier {
                    token:_,
                    value: name
                } = param {
                    (*environment.borrow_mut()).set(name.as_str().to_string(), args[idx].clone());
                }
            
            }
            body.evaluate(environment.clone())
        },
        _ => Object::Null
    }
}

fn eval_index_expression(left: Object, index: Object) -> Object {
    if let Object::Array{elements} = left {
        if let Object::Integer{value} = index {
            if value >= 0 && (value as usize) < elements.len() {
                return elements[value as usize].clone();
            }
            return Object::Null;
        }
        return Object::Null;
    }
    if let Object::HashM{pairs} = left {
        if let Some(result) = pairs.get(&index) {
            return result.clone();
        }
        return Object::Null
    }
    Object::Null
}

impl Evaluator for Expression {

    fn evaluate(&self,environment: Rc<RefCell<Environment>>) -> Object {
        match self {
            Expression::IntegerLiteral{
                 token:_, value
            } => Object::new_int(*value),
            Expression::Boolean {
                token:_, value
            } => Object::new_bool(*value),
            Expression::StringLiteral{
                token: _,
                value
            } => Object::new_string(value.clone()),
            Expression::PrefixExpression {
                token:_,
                operator,
                right
            } => {
                let right = right.evaluate(environment);
                evaluate_prefix_expression(operator, right)
            },
            Expression::InfixExpression {
                token:_,
                left,
                operator,
                right
            } => {
                let left = left.evaluate(environment.clone());
                let right = right.evaluate(environment.clone());
                evaluate_infix_expression(operator, left, right)
            },
            Expression::IfExpression {
                token:_,
                condition,
                consequence,
                alternative
            } => {
                eval_if_expression(condition, consequence, alternative, environment)
            },
            Expression::Identifier {token:_, value} => {
                eval_identifier(value, environment)
            },
            Expression::FunctionLiteral {
                token:_,
                parameters,
                body
            } => {
                Object::Function {
                    parameters : parameters.clone(),
                    body: body.as_ref().clone(),
                    environment: environment.clone()
                }
            },
            Expression::CallExpression {
                token:_,
                function,
                arguments
            } => {
                let func = function.evaluate(environment.clone());
                let args = eval_expressions(arguments, environment.clone());
                apply_function(func, args)
            },
            Expression::ArrayLiteral {
                token: _,
                elements
            } => {
                let elements = eval_expressions(elements, environment);
                Object::Array {
                    elements
                }
            },
            Expression::IndexExpression {
                token: _,
                left,
                index
            } => {
                let left = left.evaluate(environment.clone());
                let index = index.evaluate(environment.clone());
                eval_index_expression(left, index)
            },
            Expression::HashLiteral{
                token: _,
                pairs
            } => {
                let mut hash_pairs = HashMap::new();
                for (key, value) in pairs {
                    let key = key.evaluate(environment.clone());
                    let value = value.evaluate(environment.clone());
                    hash_pairs.insert(key, value);
                }
                Object::HashM {
                    pairs: hash_pairs
                }
            }
            _ => Object::Null
        }
    }
}

impl Evaluator for Program {
    fn evaluate(&self, environment: Rc<RefCell<Environment>>) -> Object {
        let mut result = Object::Null;
        for st in &self.statements {
            result = st.evaluate(environment.clone());
            if let Object::Return{value} = result {
                return *value;
            }
        }
        result
    }
}

impl Evaluator for Statement {
    fn evaluate(&self, environment: Rc<RefCell<Environment>>) -> Object {
        match self {
            Statement::ExpressionStatement{
                token:_,
                expression
            } => expression.evaluate(environment),
            Statement::BlockStatment{token:_, statements} => {
                eval_block_statements(statements, environment)
            },
            Statement::ReturnStatement{token:_, return_value} => {
                let value = return_value.evaluate(environment);
                Object::Return {
                    value: Box::new(value)
                }
            },
            Statement::LetStatement {token:_, name, value} => {
                let value = value.evaluate(environment.clone());
                if let Expression::Identifier{token:_, value:name} = name {
                    (*environment.borrow_mut()).set(name.as_str().to_string(), value);
                }
                
                Object::Null
            }
            _ => Object::Null
        }
    }
}

fn eval_block_statements(statements: &Vec<Statement>, environment: Rc<RefCell<Environment>>) -> Object {
    let mut result = Object::Null;
    for st in statements {
        result = st.evaluate(environment.clone());
        if let Object::Return{value} = result {
            return *value;
        }
    }
    result
}