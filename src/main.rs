use std::env;

mod ast;
mod lex;
mod parser;
mod unwriter;

use crate::ast::*;

fn main() {
    let mut ast_debug = false;
    let mut input = String::new();

    let args : Vec<String> = env::args().collect();
    let mut index = 0;
    for arg in args {
        if arg == "--ast" {
            ast_debug = true;
        } else {
            if index > 0 {
                input = arg;
            }
        }
        index += 1;
    }
    
    // Make sure we actually have an input file
    if input.len() == 0 {
        println!("Error: No input file!");
        return;
    }
    
    /*let mut scanner = lex::lex_new("./first.tl".to_string());
    scanner.init();
    scanner.debug();
    
    let mut token = scanner.get_next();
    while token != Token::Eof {
        println!("{:?}", token);
        token = scanner.get_next();
    }
    println!("{:?}", token);*/
    
    let mut parser = parser::parser_new(input);
    parser.init();
    parser.run();
    
    if ast_debug {
        parser.debug();
    } else {
        // Currently, we use an unwriter to print
        let file : AstFile = parser.get_file();
        unwriter::unwrite(file);
    }
}


