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
        expr.set_lval(lval);
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
                Token::Id(val) => {
                    let mut expr = ast_new_expression(AstType::Id);
                    expr.string_value = val;
                    stack.push(expr);
                },
                
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
                
                Token::CharL(val) => {
                    let mut expr = ast_new_expression(AstType::CharLiteral);
                    expr.char_value = val;
                    stack.push(expr);
                },
                
                Token::True => stack.push(ast_new_expression(AstType::BoolLiteral(true))),
                Token::False => stack.push(ast_new_expression(AstType::BoolLiteral(false))),
                
                //
                // Operators
                //
                Token::Assign => op_stack.push(ast_new_expression(AstType::Assign)),
                Token::Add => op_stack.push(ast_new_expression(AstType::Add)),
                Token::Sub => op_stack.push(ast_new_expression(AstType::Sub)),
                Token::Mul => op_stack.push(ast_new_expression(AstType::Mul)),
                Token::Div => op_stack.push(ast_new_expression(AstType::Div)),
                Token::Mod => op_stack.push(ast_new_expression(AstType::Mod)),
                Token::And => op_stack.push(ast_new_expression(AstType::And)),
                Token::Or => op_stack.push(ast_new_expression(AstType::Or)),
                Token::Xor => op_stack.push(ast_new_expression(AstType::Xor)),
                
                Token::Eq => op_stack.push(ast_new_expression(AstType::Eq)),
                Token::Ne => op_stack.push(ast_new_expression(AstType::Ne)),
                Token::Gt => op_stack.push(ast_new_expression(AstType::Gt)),
                Token::Ge => op_stack.push(ast_new_expression(AstType::Ge)),
                Token::Lt => op_stack.push(ast_new_expression(AstType::Lt)),
                Token::Le => op_stack.push(ast_new_expression(AstType::Le)),
                
                _ => {
                    println!("Error: Invalid token in expression.");
                    println!("{:?}", token);
                },
            }
        
            token = self.scanner.get_next();
        }
        
        // Processing
        //println!("LEN: {} | STACK: {}", op_stack.len(), stack.len());
        while op_stack.len() > 0 {
            let mut op = op_stack.pop().unwrap();
            //println!("TYPE: {:?} | LEN: {} | STACK: {}", op.ast_type, op_stack.len(), stack.len());
            if op.ast_type == AstType::Assign {
                let rval = stack.pop().unwrap();
                //rval.print(); println!("");
                op.set_rval(rval);
                //op.print(); println!("");
                stack.push(op);
            } else {
                let rval = stack.pop().unwrap();
                let lval = stack.pop().unwrap();
                op.set_lval(lval);
                op.set_rval(rval);
                //op.print(); println!("");
                stack.push(op);
            }
        }
        
        // Return the final expression
        if stack.len() == 0 {
            return ast_new_expression(AstType::None);
        }
        let op = stack.pop().unwrap();
        //op.print();
        op
    }
    
    //
    // A utility function for building a data type
    //
    fn build_data_type(&mut self) -> DataType {
        let token = self.scanner.get_next();
        match token {
            Token::I8 => DataType::I8,
            Token::U8 => DataType::U8,
            Token::I16 => DataType::I16,
            Token::U16 => DataType::U16,
            Token::I32 => DataType::I32,
            Token::U32 => DataType::U32,
            Token::I64 => DataType::I64,
            Token::U64 => DataType::U64,
            Token::String => DataType::String,
            Token::Char => DataType::Char,
            Token::Bool => DataType::Bool,
            
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

