use std::{vec};
use std::rc::Rc;

use crate::{environment::Environment, object::{Builtin, Object}};

pub struct Builtins {
    pub builtins: Environment
}


impl Builtins {
    pub fn new() -> Self {
        let mut builtins = Environment::new();
        let len = Rc::new(Builtin{
            func: Box::new(|args| {
                let s = args[0].clone();
                if let Object::String{
                    value
                } = s {
                    return Object::Integer{value: value.len() as i64};
                }
                if let Object::Array{elements} = s {
                    return Object::Integer{value: elements.len() as i64};
                }
                Object::Null
            })
        });

        let first = Rc::new(Builtin{
            func: Box::new(|args| {
                let s = args[0].clone();
                if let Object::Array{elements} = s {
                    if elements.len() > 0 {
                        return elements.first().unwrap().clone();
                    }
                    return Object::Null;
                }
                Object::Null
            })
        });

        let last = Rc::new(Builtin{
            func: Box::new(|args| {
                let s = args[0].clone();
                if let Object::Array{elements} = s {
                    if elements.len() > 0 {
                        return elements.last().unwrap().clone();
                    }
                    return Object::Null;
                }
                Object::Null
            })
        });

        let rest = Rc::new(Builtin{
            func: Box::new(|args| {
                let mut s = args[0].clone();
                if let Object::Array{elements} = &mut s {
                    
                    if elements.len() > 0 {
                        elements.remove(0);
                        let new_elements = std::mem::replace(elements, vec![]);
                        return Object::Array {
                            elements: new_elements
                        };
                    }
                    return Object::Null;
                }
                Object::Null
            })
        });

        let push = Rc::new(Builtin{
            func: Box::new(|args| {
                let mut s = args[0].clone();
                if args.len() != 2 {
                    return Object::Null;
                }
                let element = args[1].clone();
                if let Object::Array{elements} = &mut s {
                    elements.push(element);
                }
                s
            })
        });

        let puts = Rc::new(Builtin{
            func: Box::new(|args| {
                println!("{:#?}", args);
                Object::Null
            })
        });
    
        builtins.set("len".to_string(), Object::Builtin(len));
        builtins.set("first".to_string(), Object::Builtin(first));
        builtins.set("last".to_string(), Object::Builtin(last));
        builtins.set("rest".to_string(), Object::Builtin(rest));
        builtins.set("push".to_string(), Object::Builtin(push));
        builtins.set("puts".to_string(), Object::Builtin(puts));
    
        Builtins {
            builtins
        }

    }
}


