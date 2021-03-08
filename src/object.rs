use crate::{ast::{Expression, Statement}, environment::Environment};
use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};
use std::fmt;
use std::hash::{ Hasher};





pub struct Builtin{ pub func: Box<dyn Fn(Vec<Object>) -> Object> }

impl fmt::Debug for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<builtin>")
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Integer {
        value: i64
    },
    Boolean {
        value: bool
    },
    Return {
        value: Box<Object>
    },
    Function {
        parameters: Vec<Expression>,
        body: Statement,
        environment: Rc<RefCell<Environment>>
    },
    String {
        value: String
    },
    Array {
        elements: Vec<Object>
    },
    Builtin(Rc<Builtin>),
    HashM {
        pairs: HashMap<Object, Object>
    },
    Null
}

impl Object {
    pub fn new_int(value: i64) -> Self {
        Object::Integer{
            value
        }
    }

    pub fn new_bool(value: bool) -> Self {
        Object::Boolean{
            value
        }
    }

    pub fn new_string(value: String) -> Self {
        Object::String{
            value
        }
    }
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Object::Array{elements} => {
                elements.hash(state);
            }
            Object::Integer{value} => {
                value.hash(state);
            }
            Object::Boolean{value} => {
                value.hash(state);
            }
            Object::String{value} => {
                value.hash(state);
            }
            _ => {
                panic!("no impl for hash");
            }
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Object) -> bool {
        match self {
            Object::Array{elements} => {
                if let Object::Array{elements: elements1} = other {
                    return elements == elements1;
                }
                false
            }
            Object::Integer{value} => {
                if let Object::Integer{value: v1} = other {
                    return value == v1;
                }
                false
            }
            Object::Boolean{value} => {
                if let Object::Boolean{value: v1} = other {
                    return value == v1;
                }
                false
            }
            Object::String{value} => {
                if let Object::String{value: v1} = other {
                    return value == v1;
                }
                false
            }
            _ => {
                panic!("no impl for hash");
            }
        }
    }
}

impl Eq for Object {}