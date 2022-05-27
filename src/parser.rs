use crate::lex::*;
use crate::ast::*;

//
// The parser context
//
pub struct Parser {
    pub file_name : String,
    pub ast : AstFile,
    scanner : Scanner,
}

impl Parser {
    // Handy utility functions
    pub fn init(&mut self) {
        self.scanner.init();
    }
    
    pub fn debug(&self) {
        self.scanner.debug();
        println!("=================");
        self.ast.print();
    }
    
    pub fn get_file(&self) -> AstFile {
        self.ast.clone()
    }
    
    //
    // The main run function
    // This operates on the global scope
    //
    pub fn run(&mut self) {
        let mut token = self.scanner.get_next();
        while token != Token::Eof {
            match token {
                Token::Func => self.build_function(),
                
                _ => {
                    println!("Error: Unknown token in global scope.");
                    println!("-> {:?}", token);
                },
            }
            
            token = self.scanner.get_next();
        }
    }
    
    //
    // Builds a function
    //
    pub fn build_function(&mut self) {
        let mut token = self.scanner.get_next();
        let function_name : String;
        match token {
            Token::Id(name) => function_name = name,
            
            _ => {
                println!("Error: Expected function name.");
                return;
            },
        }
        
        // Function arguments
        token = self.scanner.get_next();
        if token == Token::LParen {
            // TODO
            while token != Token::RParen {
                token = self.scanner.get_next();
            }
            
            token = self.scanner.get_next();
        }
        
        // Check function return
        // TODO
        
        // Finally, a block start
        if token != Token::Is {
            println!("Error: Expected \"is\".");
            return;
        }
        
        // Build the block
        let block = self.build_block();
        
        // Build the AST element
        let mut func : AstFunction = ast_new_function(function_name);
        func.block = block;
        self.ast.add_function(func);
    }
    
    //
    // Builds a statement block
    //
    fn build_block(&mut self) -> AstStatement {
        let mut block = ast_new_statement(AstType::Block);
        let mut token = self.scanner.get_next();
        
        while token != Token::End && token != Token::Eof {
            match token {
                Token::Var => {
                    let stmt = self.build_variable_dec();
                    block.add_statement(stmt);
                },
            
                Token::Id(name) => {
                    let expr = self.build_expression(Token::SemiColon);
                    if expr.ast_type == AstType::Assign {
                        // TODO: Variable assignment
                    } else {
                        let mut stmt = ast_new_statement(AstType::CallStmt);
                        stmt.set_name(name);
                        stmt.set_expression(expr);
                        block.add_statement(stmt);
                    }
                },
                
                _ => {
                    println!("Error: Invalid token statement.");
                    println!("{:?}", token);
                },
            }
            
            token = self.scanner.get_next();
        }
        
        block
    }
    
    // Builds a variable declaration
    fn build_variable_dec(&mut self) -> AstStatement {
        let mut token = self.scanner.get_next();
        let name : String;
        match token {
            Token::Id(value) => name = value,
            _ => {
                println!("Error: Expected name in variable declaration.");
                return ast_new_statement(AstType::None);
            },
        }
        
        token = self.scanner.get_next();
        if token != Token::Colon {
            println!("Error: Expected colon.");
            return ast_new_statement(AstType::None);
        }
    
        let mut stmt = ast_new_statement(AstType::VarDec);
        stmt.name = name.clone();
        stmt.data_type = self.build_data_type();
        
        // This is for the assignment operation
        //
        let mut lval = ast_new_expression(AstType::Id);
        lval.string_value = name;
        
        let mut expr = self.build_expression(Token::SemiColon);
        expr.args.insert(0, lval);
        stmt.set_expression(expr);
        
        stmt
    }
    
    //
    // Builds an expression
    //
    fn build_expression(&mut self, stop : Token) -> AstExpression {
        let mut stack : Vec<AstExpression> = Vec::new();
        let mut op_stack : Vec<AstExpression> = Vec::new();
        let mut token = self.scanner.get_next();
        
        while token != stop && token != Token::Eof {
            match token {
                Token::LParen => {
                    let expr = self.build_expression(Token::RParen);
                    stack.push(expr);
                },
                
                
                //
                // Literals
                //
                Token::IntL(val) => {
                    let mut expr = ast_new_expression(AstType::IntLiteral);
                    expr.int_value = val;
                    stack.push(expr);
                },
                
                Token::StringL(val) => {
                    let mut expr = ast_new_expression(AstType::StringLiteral);
                    expr.string_value = val;
                    stack.push(expr);
                },
                
                //
                // Operators
                //
                Token::Assign => {
                    op_stack.push(ast_new_expression(AstType::Assign));
                },
                
                _ => {
                    println!("Error: Invalid token in expression.");
                    println!("{:?}", token);
                },
            }
        
            token = self.scanner.get_next();
        }
        
        // Processing
        while op_stack.len() > 0 {
            let mut op = op_stack.pop().unwrap();
            if op.ast_type == AstType::Assign {
                let rval = stack.pop().unwrap();
                op.args.push(rval);
                stack.push(op);
            } else {
                // TODO
            }
        }
        
        // Return the final expression
        if stack.len() == 0 {
            return ast_new_expression(AstType::None);
        }
        stack.pop().unwrap()
    }
    
    //
    // A utility function for building a data type
    //
    fn build_data_type(&mut self) -> DataType {
        let token = self.scanner.get_next();
        match token {
            Token::I32 => DataType::I32,
            
            _ => {
                println!("Error: Unknown data type token.");
                println!("{:?}", token);
                
                DataType::Void
            },
        }
    }
}

//
// A helper function to create the parser
//
pub fn parser_new(file_name : String) -> Parser {
    Parser {
        file_name : file_name.clone(),
        ast : ast_new_file(file_name.clone()),
        scanner : lex_new(file_name),
    }
}

