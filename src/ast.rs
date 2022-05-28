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
    VarDec,
    CallStmt,
    While,
    If, Elif, Else,
    
    // Expressions
    ExprList,
    
    // Expressions- operators
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And, Or, Xor,
    Eq, Ne, Gt, Lt, Ge, Le,
    
    // Expressions- literals
    Id,
    IntLiteral,
    CharLiteral,
    StringLiteral,
    BoolLiteral(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Void,
    I8, U8,
    I16, U16,
    I32, U32,
    I64, U64,
    String,
    Char,
    Bool,
}

#[derive(Clone)]
pub struct AstFile {
    name : String,
    functions : Vec<AstFunction>,
}

#[derive(Clone)]
pub struct AstFunction {
    name : String,
    block : AstStatement,
}

#[derive(Clone)]
pub struct AstStatement {
    ast_type : AstType,
    name : String,
    data_type : DataType,
    expr : AstExpression,
    statements : Vec<AstStatement>,     // For blocks
    branches : Vec<AstStatement>,       // For conditionals
}

#[derive(Clone)]
pub struct AstExpression {
    ast_type : AstType,
    int_value : u64,
    char_value : char,
    string_value : String,
    
    // This should only be used by an expression list
    list : Vec<AstExpression>,
    
    // Only for binary operators
    args : Vec<AstExpression>,
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
    
    //
    // Setter functions
    //
    pub fn add_function(&mut self, func : AstFunction) {
        self.functions.push(func);
    }
    
    //
    // Getter functions
    //
    pub fn get_functions(&self) -> &Vec<AstFunction> {
        &self.functions
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
    
    //
    // Setter functions
    //
    pub fn set_block(&mut self, block : AstStatement) {
        self.block = block;
    }
    
    //
    // Getter functions
    //
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    
    pub fn get_block(&self) -> &AstStatement {
        &self.block
    }
}

impl AstStatement {
    pub fn print(&self, index : i32) {
        if self.ast_type == AstType::Block {
            for stmt in &self.statements {
                for _i in 0 .. index { print!(" "); }
                stmt.print(index);
            }
            //for _i in 0 .. index { print!(" "); }
            //println!("end");
        } else {
            for _i in 0 .. index {
                print!(" ");
            }
            print!("{:?} {:?} {} ", self.ast_type, self.data_type, self.name);
            
            //if self.expr.ast_type != AstType::None {
                self.expr.print();
            //}
            
            println!("");
            
            match &self.ast_type {
                AstType::While => {
                    let block = self.get_block();
                    block.print(index+2);
                    for _i in 0 .. index { print!(" "); }
                    println!("end");
                },
                
                AstType::If | AstType::Elif | AstType::Else => {
                    let block = self.get_block();
                    block.print(index+2);
                    for br in self.get_branches() {
                        br.print(index);
                    }
                    if self.ast_type == AstType::If {
                        for _i in 0 .. index { print!(" "); }
                        println!("end");
                    }
                },
                _ => {},
            }
        }
    }
    
    //
    // Setter functions
    //
    pub fn set_name(&mut self, name : String) {
        self.name = name;
    }
    
    pub fn set_data_type(&mut self, data_type : DataType) {
        self.data_type = data_type;
    }
    
    pub fn set_expression(&mut self, expr : AstExpression) {
        self.expr = expr;
    }
    
    pub fn add_statement(&mut self, stmt : AstStatement) {
        self.statements.push(stmt);
    }
    
    pub fn add_sub_block(&mut self, stmt : AstStatement) {
        self.statements.push(stmt);
    }
    
    pub fn add_branch(&mut self, stmt : AstStatement) {
        self.branches.push(stmt);
    }
    
    //
    // Getter functions
    //
    pub fn get_type(&self) -> AstType {
        self.ast_type.clone()
    }
    
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    
    pub fn get_data_type(&self) -> DataType {
        self.data_type.clone()
    }
    
    pub fn get_expression(&self) -> &AstExpression {
        &self.expr
    }
    
    pub fn get_statements(&self) -> &Vec<AstStatement> {
        &self.statements
    }
    
    pub fn get_block(&self) -> &AstStatement {
        &self.statements[0]
    }
    
    pub fn get_branches(&self) -> &Vec<AstStatement> {
        &self.branches
    }
}

impl AstExpression {
    pub fn print(&self) {
        match self.ast_type {
            //
            // Generic expressions
            //
            AstType::ExprList => {
                print!("{{");
                let mut index : usize = 0;
                for item in &self.list {
                    item.print();
                    if index + 1 < self.list.len() {
                        print!(", ");
                    }
                    index += 1;
                }
                print!("}}");
            },
            
            //
            // Binary operators
            //
            AstType::Assign
            | AstType::Add | AstType::Sub
            | AstType::Mul | AstType::Div | AstType::Mod
            | AstType::And | AstType::Or | AstType::Xor 
            | AstType::Eq | AstType::Ne
            | AstType::Gt | AstType::Ge | AstType::Lt | AstType::Le => {
                print!("(");
                self.args[0].print();
                match self.ast_type {
                    AstType::Assign => print!(" := "),
                
                    AstType::Add => print!(" + "),
                    AstType::Sub => print!(" - "),
                    AstType::Mul => print!(" * "),
                    AstType::Div => print!(" / "),
                    AstType::Mod => print!(" % "),
                    AstType::And => print!(" & "),
                    AstType::Or => print!(" | "),
                    AstType::Xor => print!(" ^ "),
                    
                    AstType::Eq => print!(" = "),
                    AstType::Ne => print!(" != "),
                    AstType::Gt => print!(" > "),
                    AstType::Ge => print!(" >= "),
                    AstType::Lt => print!(" < "),
                    AstType::Le => print!(" <= "),
                    
                    _ => {},
                }
                self.args[1].print();
                print!(")");
            },
            
            //
            // Literal expressions
            //
            AstType::Id => {
                print!("ID({})", self.string_value);
            }
            
            AstType::IntLiteral => {
                print!("{}", self.int_value);
            },
            
            AstType::CharLiteral => {
                print!("\'{:?}\'", self.char_value);
            },
            
            AstType::StringLiteral => {
                print!("{:?}", self.string_value);
            },
            
            _ => { print!("??-> {:?}", self.ast_type); },
        }
    }
    
    //
    // Setter functions
    //
    pub fn set_lval(&mut self, item : AstExpression) {
        if self.args.len() > 0 {
            self.args.insert(0, item);
        } else {
            self.args.push(item);
        }
    }
    
    pub fn set_rval(&mut self, item : AstExpression) {
        self.args.push(item);
    }
    
    pub fn add_list_item(&mut self, item : AstExpression) {
        self.list.push(item);
    }
    
    pub fn set_int(&mut self, value : u64) {
        self.int_value = value;
    }
    
    pub fn set_char(&mut self, value : char) {
        self.char_value = value;
    }
    
    pub fn set_string(&mut self, value : String) {
        self.string_value = value;
    }
    
    pub fn set_name(&mut self, value : String) {
        self.string_value = value;
    }
    
    //
    // Getter functions
    //
    pub fn get_type(&self) -> AstType {
        self.ast_type.clone()
    }
    
    pub fn get_lval(&self) -> &AstExpression {
        &self.args[0]
    }
    
    pub fn get_rval(&self) -> &AstExpression {
        &self.args[1]
    }
    
    pub fn get_list(&self) -> &Vec<AstExpression> {
        &self.list
    }
    
    pub fn get_list_size(&self) -> usize {
        self.list.len()
    }
    
    pub fn get_int(&self) -> u64 {
        self.int_value
    }
    
    pub fn get_char(&self) -> char {
        self.char_value
    }
    
    pub fn get_string(&self) -> String {
        self.string_value.clone()
    }
    
    pub fn get_name(&self) -> String {
        self.string_value.clone()
    }
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
        data_type : DataType::Void,
        expr : ast_new_expression(AstType::None),
        statements : Vec::new(),
        branches : Vec::new(),
    }
}

pub fn ast_new_expression(ast_type : AstType) -> AstExpression {
    AstExpression {
        ast_type : ast_type,
        int_value : 0,
        char_value : 0 as char,
        string_value : String::new(),
        list : Vec::new(),
        args : Vec::new(),
    }
}

