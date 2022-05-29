use crate::lex::*;
use crate::ast::*;

//
// The parser context
//
pub struct Parser {
    pub file_name : String,
    pub ast : AstFile,
    scanner : Scanner,
    local_consts : Vec<AstArg>,
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
                Token::Struct => self.build_struct_def(),
                
                Token::Const => {
                    let c = self.build_const();
                    self.ast.add_const(c);
                },
                
                Token::Import => {
                    let mut path = String::new();
                    let mut token = self.scanner.get_next();
                    while token != Token::SemiColon {
                        match token {
                            Token::Id(val) => path.push_str(&val.clone()),
                            Token::Dot => path.push('/'),
                            
                            _ => {},
                        }
                        
                        token = self.scanner.get_next();
                    }
                    
                    self.ast.add_import(path);
                },
                
                _ => {
                    println!("Error: Unknown token in global scope.");
                    println!("-> {:?}", token);
                },
            }
            
            token = self.scanner.get_next();
        }
    }
    
    //
    // Builds a structure definition
    //
    pub fn build_struct_def(&mut self) {
        let mut token = self.scanner.get_next();
        let struct_name : String;
        match token {
            Token::Id(name) => struct_name = name,
            
            _ => {
                println!("Error: Expected structure name.");
                return;
            },
        }
        
        token = self.scanner.get_next();
        if token != Token::Is {
            println!("Error: Expected \"is\".");
            return;
        }
        
        // Create the element
        let mut ast_struct = ast_new_struct(struct_name);
        
        // Now, parse the block
        token = self.scanner.get_next();
        while token != Token::End && token != Token::Eof {
            // First token is name
            let name : String;
            match token {
                Token::Id(val) => name = val,
                
                _ => {
                    println!("Error: Expected item name.");
                    return;
                },
            }
            let colon_token = self.scanner.get_next();
            let data_type = self.build_data_type();
            
            if colon_token != Token::Colon {
                println!("Error: Expected \':\' in structure item.");
                return;
            }
            
            // This is for the assignment operation
            let mut lval = ast_new_expression(AstType::Id);
            lval.set_name(name.clone());
            
            let mut expr = self.build_expression(Token::SemiColon);
            expr.set_lval(lval);
            
            // Build the AST element
            let mut arg = ast_new_arg(name, data_type);
            arg.set_expression(expr);
            ast_struct.add_item(arg);
            
            // Get the next token
            token = self.scanner.get_next();
        }
        
        // Add the structure to the tree
        self.ast.add_struct(ast_struct)
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
        let mut args : Vec<AstArg> = Vec::new();
        if token == Token::LParen {
            while token != Token::RParen {
                let name_token = self.scanner.get_next();
                let colon_token = self.scanner.get_next();
                let data_type = self.build_data_type();
                token = self.scanner.get_next();
                
                if colon_token != Token::Colon {
                    println!("Error: Expected colon in function argument.");
                    println!(" -> {:?}", colon_token);
                    return;
                }
                
                let name : String;
                match &name_token {
                    Token::Id(val) => name = val.clone(),
                    _ => {
                        println!("Error: Expected argument name.");
                        return;
                    },
                }
                
                if token != Token::Comma && token != Token::RParen {
                    println!("Error: Expected \',\' or \'(\' after argument.");
                    return;
                }
                
                let arg = ast_new_arg(name, data_type);
                args.push(arg);
            }
            
            token = self.scanner.get_next();
        }
        
        // Check function return
        let data_type : DataType;
        if token == Token::Arrow {
            data_type = self.build_data_type();
            token = self.scanner.get_next();
        } else {
            data_type = DataType::Void;
        }
        
        // Finally, a block start
        if token != Token::Is {
            println!("Error: Expected \"is\".");
            return;
        }
        
        // Build the block
        let block = self.build_block();
        
        // Build the AST element
        let mut func : AstFunction = ast_new_function(function_name);
        func.set_data_type(data_type);
        func.set_block(block);
        for arg in args { func.add_arg(arg); }
        for c in self.local_consts.clone() { func.add_const(c); }
        self.ast.add_function(func);
        self.local_consts.clear();
    }
    
    //
    // Builds a statement block
    //
    fn build_block(&mut self) -> AstStatement {
        let mut block = ast_new_statement(AstType::Block);
        let mut token = self.scanner.get_next();
        
        while token != Token::End && token != Token::Eof {
            match token {
                Token::Return => {
                    let expr = self.build_expression(Token::SemiColon);
                    let mut stmt = ast_new_statement(AstType::Return);
                    stmt.set_expression(expr);
                    block.add_statement(stmt);
                },
            
                Token::Var => {
                    let stmt = self.build_variable_dec();
                    block.add_statement(stmt);
                },
                
                Token::Const => {
                    let c = self.build_const();
                    self.local_consts.push(c);
                },
                
                Token::Struct => {
                    token = self.scanner.get_next();
                    let var_name : String;
                    match token {
                        Token::Id(val) => var_name = val,
                        
                        _ => {
                            println!("Error: Expected variable name in structure declaration.");
                            var_name = String::new();
                            //return;
                        },
                    }
                    
                    token = self.scanner.get_next();
                    if token != Token::Colon {
                        println!("Error: Expected \':\' between structure variable name and structure name.");
                        //return;
                    }
                    
                    // Create the AST elements
                    let struct_id = self.build_expression(Token::SemiColon);        // TODO: Make sure this is only ID
                    let mut stmt = ast_new_statement(AstType::StructDec);
                    stmt.set_name(var_name);
                    stmt.set_expression(struct_id);
                    block.add_statement(stmt);
                },
            
                Token::Id(name) => {
                    token = self.scanner.get_next();
                    let mut sub_expr : AstExpression;
                    let ast_type : AstType;
                    if token == Token::LBracket {
                        sub_expr = self.build_expression(Token::RBracket);
                        ast_type = AstType::ArrayAcc;
                    } else if token == Token::Dot {
                        token = self.scanner.get_next();
                        let item_name : String;
                        match token {
                            Token::Id(val) => item_name = val,
                            
                            _ => {
                                println!("Error: Expected item name in structure access.");
                                item_name = String::new();
                                //return;
                            },
                        }
                        
                        sub_expr = ast_new_expression(AstType::Id);
                        sub_expr.set_name(item_name);
                        ast_type = AstType::StructAcc;
                    } else {
                        self.scanner.unget(token);
                        sub_expr = ast_new_expression(AstType::None);
                        ast_type = AstType::Id;
                    }
                
                    let mut expr = self.build_expression(Token::SemiColon);
                    if expr.get_type() == AstType::Assign {
                        let mut lval = ast_new_expression(ast_type);
                        lval.set_name(name);
                        lval.set_arg(sub_expr);
                        expr.set_lval(lval);
                        
                        let mut stmt = ast_new_statement(AstType::ExprStmt);
                        stmt.set_expression(expr);
                        block.add_statement(stmt);
                    } else {
                        let mut stmt = ast_new_statement(AstType::CallStmt);
                        stmt.set_name(name);
                        stmt.set_expression(expr);
                        block.add_statement(stmt);
                    }
                },
                
                Token::While => {
                    let expr = self.build_expression(Token::Do);
                    let mut stmt = ast_new_statement(AstType::While);
                    stmt.set_expression(expr);
                    
                    let sub_block = self.build_block();
                    stmt.add_sub_block(sub_block);
                    
                    block.add_statement(stmt);
                },
                
                Token::If => {
                    let expr = self.build_expression(Token::Then);
                    let mut stmt = ast_new_statement(AstType::If);
                    stmt.set_expression(expr);
                    
                    let sub_block = self.build_block();
                    for br in sub_block.get_branches() {
                        stmt.add_branch(br.clone());
                    }
                    stmt.add_sub_block(sub_block);
                    
                    block.add_statement(stmt);
                },
                
                Token::Elif => {
                    let expr = self.build_expression(Token::Then);
                    let mut stmt = ast_new_statement(AstType::Elif);
                    stmt.set_expression(expr);
                    
                    let sub_block = self.build_block();
                    for br in sub_block.get_branches() {
                        stmt.add_branch(br.clone());
                    }
                    stmt.add_sub_block(sub_block);
                    
                    block.add_branch(stmt);
                    return block;
                },
                
                Token::Else => {
                    let mut stmt = ast_new_statement(AstType::Else);
                    let sub_block = self.build_block();
                    /*for br in sub_block.get_branches() {
                        stmt.add_branch(br.clone());
                    }*/
                    stmt.add_sub_block(sub_block);
                    
                    block.add_branch(stmt);
                    return block;
                },
                
                Token::Break | Token::Continue => {
                    let keyword = token;
                    token = self.scanner.get_next();
                    if token != Token::SemiColon {
                        println!("Error: Expected terminator.");
                        println!("{:?}", token);
                    }
                    
                    if keyword == Token::Break {
                        let stmt = ast_new_statement(AstType::Break);
                        block.add_statement(stmt);
                    } else if keyword == Token::Continue {
                        let stmt = ast_new_statement(AstType::Continue);
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
        
        let data_type = self.build_data_type();
        
        token = self.scanner.get_next();
        if token == Token::LBracket {
            let mut stmt = ast_new_statement(AstType::ArrayDec);
            stmt.set_name(name.clone());
            stmt.set_data_type(data_type);
            
            let expr = self.build_expression(Token::RBracket);
            stmt.set_expression(expr);
            
            token = self.scanner.get_next();
            if token != Token::SemiColon {
                println!("Error: Expected terminator.");
                return ast_new_statement(AstType::None);
            }
            
            stmt
        } else {
            self.scanner.unget(token);
        
            let mut stmt = ast_new_statement(AstType::VarDec);
            stmt.set_name(name.clone());
            stmt.set_data_type(data_type);
            
            // This is for the assignment operation
            //
            let mut lval = ast_new_expression(AstType::Id);
            lval.set_name(name);
            
            let mut expr = self.build_expression(Token::SemiColon);
            expr.set_lval(lval);
            stmt.set_expression(expr);
            
            stmt
        }
    }
    
    // Builds a constant declaration
    fn build_const(&mut self) -> AstArg {
        let mut token = self.scanner.get_next();
        let name : String;
        match token {
            Token::Id(value) => name = value,
            _ => {
                println!("Error: Expected name in variable declaration.");
                name = String::new();
                //return ast_new_statement(AstType::None);
            },
        }
        
        token = self.scanner.get_next();
        if token != Token::Colon {
            println!("Error: Expected colon.");
            //return ast_new_statement(AstType::None);
        }
        
        let data_type = self.build_data_type();
        
        token = self.scanner.get_next();
        if token != Token::Assign {
            println!("Error: Expected assignment operator.");
            //return ast_new_statement(AstType::None);
        }

        let expr = self.build_expression(Token::SemiColon);
        let mut c = ast_new_arg(name, data_type);
        c.set_expression(expr);
        c
    }
    
    //
    // Builds an expression
    //
    fn process_expression(&mut self, stack : &mut Vec<AstExpression>, op_stack : &mut Vec<AstExpression>) {
        while op_stack.len() > 0 {
            let mut op = op_stack.pop().unwrap();
            if op.get_type() == AstType::Assign {
                let rval = stack.pop().unwrap();
                op.set_rval(rval);
                stack.push(op);
            } else {
                let rval = stack.pop().unwrap();
                let lval = stack.pop().unwrap();
                op.set_lval(lval);
                op.set_rval(rval);
                stack.push(op);
            }
        }
    }
    
    fn build_expression(&mut self, stop : Token) -> AstExpression {
        let mut stack : Vec<AstExpression> = Vec::new();
        let mut op_stack : Vec<AstExpression> = Vec::new();
        let mut token = self.scanner.get_next();
        
        let mut list_expr = ast_new_expression(AstType::ExprList);
        let mut is_list = false;
        
        while token != stop && token != Token::Eof {
            match token {
                Token::LParen => {
                    let expr = self.build_expression(Token::RParen);
                    stack.push(expr);
                },
                
                Token::Comma => {
                    is_list = true;
                    self.process_expression(&mut stack, &mut op_stack);
                    list_expr.add_list_item(stack.pop().unwrap());
                },
                
                
                //
                // Literals
                //
                Token::Id(val) => {
                    token = self.scanner.get_next();
                    if token == Token::LParen {
                        let sub_expr = self.build_expression(Token::RParen);
                        let mut expr = ast_new_expression(AstType::Call);
                        expr.set_name(val);
                        expr.set_arg(sub_expr);
                        stack.push(expr);
                    } else if token == Token::LBracket {
                        let sub_expr = self.build_expression(Token::RBracket);
                        let mut expr = ast_new_expression(AstType::ArrayAcc);
                        expr.set_name(val);
                        expr.set_arg(sub_expr);
                        stack.push(expr);
                    } else if token == Token::Dot {
                        token = self.scanner.get_next();
                        let item_name : String;
                        match token {
                            Token::Id(val) => item_name = val,
                            
                            _ => {
                                println!("Error: Expected item name in structure access.");
                                item_name = String::new();
                                //return;
                            },
                        }
                        
                        let mut sub_expr = ast_new_expression(AstType::Id);
                        sub_expr.set_name(item_name);
                        let mut expr = ast_new_expression(AstType::StructAcc);
                        expr.set_name(val);
                        expr.set_arg(sub_expr);
                        stack.push(expr);
                    } else {
                        self.scanner.unget(token);
                        let mut expr = ast_new_expression(AstType::Id);
                        expr.set_name(val);
                        stack.push(expr);
                    }
                },
                
                Token::IntL(val) => {
                    let mut expr = ast_new_expression(AstType::IntLiteral);
                    expr.set_int(val);
                    stack.push(expr);
                },
                
                Token::StringL(val) => {
                    let mut expr = ast_new_expression(AstType::StringLiteral);
                    expr.set_string(val);
                    stack.push(expr);
                },
                
                Token::CharL(val) => {
                    let mut expr = ast_new_expression(AstType::CharLiteral);
                    expr.set_char(val);
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
                
                Token::LGAnd => op_stack.push(ast_new_expression(AstType::LGAnd)),
                Token::LGOr => op_stack.push(ast_new_expression(AstType::LGOr)),
                
                _ => {
                    println!("Error: Invalid token in expression.");
                    println!("{:?}", token);
                },
            }
        
            token = self.scanner.get_next();
        }
        
        if stop == Token::RParen && is_list {
            if stack.len() > 0 {
                self.process_expression(&mut stack, &mut op_stack);
                list_expr.add_list_item(stack.pop().unwrap());
            }
        }
        
        // Processing
        self.process_expression(&mut stack, &mut op_stack);
        
        // Return the final expression
        if is_list {
            return list_expr;
        }
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
        local_consts : Vec::new(),
    }
}

