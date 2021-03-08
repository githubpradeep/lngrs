
use std::{cell::RefCell, io::{self, Write}};
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod object;
pub mod evaluator;
pub mod environment;
pub mod builtins;
pub mod code;
pub mod compiler;
pub mod vm;
use builtins::Builtins;
use code::Definitions;
use evaluator::Evaluator;
use lexer::{Lexer};
use parser::Parser;
use std::rc::Rc;
use std::env;
use std::fs;



fn main() {
    let args: Vec<String> = env::args().collect();
    //let definitions = Definitions::new();
    //let inst = definitions.make(code::OpConstant, vec![25]);
    //println!("{:?}", inst);

    let b = Builtins::new();
    let environment = b.builtins.create_child();
    let env= Rc::new(RefCell::new(environment));
    if args.len() == 2 {
        let contents = fs::read_to_string(args[1].clone())
        .expect("Something went wrong reading the file");
        let lexer = Lexer::new(contents);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        if program.is_ok() {
            let ev = program.unwrap();
            println!("{:#?}", ev.evaluate(env.clone()));
        } else {
            println!("{:#?}", program.err());
        }
    } else {
        loop {
            print!(">> ");
            io::stdout().flush().unwrap();
    
            let mut buffer = String::new();
            let stdin = io::stdin(); // We get `Stdin` here.
            stdin.read_line(&mut buffer).unwrap();
            let lexer = Lexer::new(buffer);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            if program.is_ok() {
                let ev = program.unwrap();
                println!("{:#?}", ev.evaluate(env.clone()));
            } else {
                println!("{:#?}", program.err());
            }
            
        }
    }
}

