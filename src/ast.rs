//
// Contains the AST for our python interpreter
//

//
// The type definitions
//
#[derive(Debug, Clone, PartialEq)]
pub enum AstType {
    None,

    // Statements
    Block,
    CallStmt,
    
    // Expressions
    //ExprList,
    
    // Expressions- operators
    Assign,
    
    // Expressions- literals
    StringLiteral,
}

#[derive(Clone)]
pub struct AstFile {
    pub name : String,
    pub functions : Vec<AstFunction>,
}

#[derive(Clone)]
pub struct AstFunction {
    pub name : String,
    pub block : AstStatement,
}

#[derive(Clone)]
pub struct AstStatement {
    pub ast_type : AstType,
    pub name : String,
    pub expr : AstExpression,
    pub statements : Vec<AstStatement>,     // For blocks
}

#[derive(Clone)]
pub struct AstExpression {
    pub ast_type : AstType,
    pub string_value : String,
    
    // This should only be used by an expression list
    pub list : Vec<AstExpression>,
    
    // Only for binary operators
    pub args : Vec<AstExpression>,
}

//
// Function implementations for the structuress
//
impl AstFile {
    pub fn print(&self) {
        println!("FILE {}", self.name);
        println!("");
        
        for func in &self.functions {
            func.print();
        }
    }
    
    pub fn add_function(&mut self, func : AstFunction) {
        self.functions.push(func);
    }
}

impl AstFunction {
    pub fn print(&self) {
        println!("func {} is", self.name);
        for stmt in &self.block.statements {
            stmt.print(2);
        }
        println!("end");
    }
}

impl AstStatement {
    pub fn print(&self, index : i32) {
        for _i in 0 .. index {
            print!(" ");
        }
        print!("{:?} {} ", self.ast_type, self.name);
        
        if self.expr.ast_type != AstType::None {
            self.expr.print();
        }
        
        println!("");
    }
    
    pub fn set_name(&mut self, name : String) {
        self.name = name;
    }
    
    pub fn set_expression(&mut self, expr : AstExpression) {
        self.expr = expr;
    }
    
    pub fn add_statement(&mut self, stmt : AstStatement) {
        self.statements.push(stmt);
    }
}

impl AstExpression {
    pub fn print(&self) {
        match self.ast_type {
            //
            // Generic expressions
            //
            /*AstType::ExprList => {
                print!("(");
                let mut index : usize = 0;
                for item in &self.list {
                    item.print();
                    if index + 1 < self.list.len() {
                        print!(", ");
                    }
                    index += 1;
                }
                print!(")");
            },*/
            
            //
            // Binary operators
            //
            AstType::Assign => {
                //self.lval.print();
                print!(" = ");
                //self.rval.print();
            },
            
            //
            // Literal expressions
            //
            AstType::StringLiteral => {
                print!("{:?}", self.string_value);
            },
            
            _ => { print!("??"); },
        }
    }
    
    /*pub fn add_list_item(&mut self, item : AstExpression) {
        self.list.push(item);
    }*/
}

//
// Helper functions for the user
//
pub fn ast_new_file(name : String) -> AstFile {
    AstFile {
        name : name,
        functions : Vec::new(),
    }
}

pub fn ast_new_function(name : String) -> AstFunction {
    AstFunction {
        name : name,
        block : ast_new_statement(AstType::Block),
    }
}

pub fn ast_new_statement(ast_type : AstType) -> AstStatement {
    AstStatement {
        ast_type : ast_type,
        name : String::new(),
        expr : ast_new_expression(AstType::None),
        statements : Vec::new(),
    }
}

pub fn ast_new_expression(ast_type : AstType) -> AstExpression {
    AstExpression {
        ast_type : ast_type,
        string_value : String::new(),
        list : Vec::new(),
        args : Vec::new(),
    }
}
