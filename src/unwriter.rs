use crate::ast::*;

pub fn unwrite(file : AstFile) {
    for func in file.get_functions() {
        unwrite_function(func);
    }
}

fn unwrite_function(func : &AstFunction) {
    println!("func {} is", func.get_name());
    unwrite_block(func.get_block(), 0);
    println!("end");
    //println!("");
}

fn unwrite_block(block : &AstStatement, indent : i32) {
    for stmt in block.get_statements() {
        unwrite_statement(&stmt, indent+4);
    }
    
    /*for _i in 0 .. indent {
        print!(" ");
    }
    println!("end");*/
}

fn unwrite_statement(stmt : &AstStatement, indent : i32) {
    for _i in 0 .. indent {
        print!(" ");
    }
    
    match stmt.get_type() {
        AstType::VarDec => {
            print!("var {} : ", stmt.get_name());
            unwrite_data_type(&stmt.get_data_type());
            print!(" := ");
            unwrite_expression(stmt.get_expression(), true);
            println!(";");
        },
        
        AstType::ArrayDec => {
            print!("var {} : ", stmt.get_name());
            unwrite_data_type(&stmt.get_data_type());
            print!("[");
            unwrite_expression(stmt.get_expression(), false);
            println!("];");
        },
    
        AstType::CallStmt => {
            print!("{}", stmt.get_name());
            //if stmt.expr.ast_type != AstType::ExprList {
                print!("(");
                unwrite_expression(stmt.get_expression(), false);
                print!(")");
            /*} else {
                unwrite_expression(&stmt.expr);
            }*/
            println!(";");
        },
        
        AstType::While => {
            print!("while ");
            unwrite_expression(stmt.get_expression(), false);
            println!(" do");
            
            unwrite_block(stmt.get_block(), indent);
            for _i in 0 .. indent { print!(" "); }
            println!("end");
        },
        
        AstType::If | AstType::Elif => {
            if stmt.get_type() == AstType::Elif { print!("elif "); }
            else { print!("if "); }
            unwrite_expression(stmt.get_expression(), false);
            println!(" then");
            
            unwrite_block(stmt.get_block(), indent);
            
            for br in stmt.get_branches() {
                unwrite_statement(br, indent);
            }
            
            if stmt.get_type() == AstType::If {
                for _i in 0 .. indent { print!(" "); }
                println!("end");
            }
        },
        
        AstType::Else => {
            println!("else");
            unwrite_block(stmt.get_block(), indent);
        },
        
        _ => { println!(""); },
    }
}

fn unwrite_expression(expr : &AstExpression, ignore_lval : bool) {
    match expr.get_type() {
        //
        // Binary operators
        //
        AstType::Assign => {
            if !ignore_lval {
                unwrite_expression(expr.get_lval(), false);
                print!(" := ");
            }
            unwrite_expression(expr.get_rval(), false);
        },
        
        AstType::Add | AstType::Sub
        | AstType::Mul | AstType::Div | AstType::Mod
        | AstType::And | AstType::Or | AstType::Xor 
        | AstType::Eq | AstType::Ne
        | AstType::Gt | AstType::Ge | AstType::Lt | AstType::Le => {
            unwrite_expression(expr.get_lval(), false);
            match expr.get_type() {
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
            unwrite_expression(expr.get_rval(), false);
        },
        
        //
        // Literals and primary expressions
        //
        AstType::Id => print!("{}", expr.get_name()),
        AstType::IntLiteral => print!("{}", expr.get_int()),
        AstType::StringLiteral => print!("{:?}", expr.get_string()),
        AstType::CharLiteral => print!("{:?}", expr.get_char()),
        AstType::BoolLiteral(val) => print!("{}", val),
        
        //
        // Generic expressions
        //
        AstType::ExprList => {
            let mut index : usize = 0;
            for item in expr.get_list() {
                unwrite_expression(&item, false);
                if index + 1 < expr.get_list_size() {
                    print!(", ");
                }
                index += 1;
            }
        },
        
        _ => {},
    }
}

fn unwrite_data_type(data_type : &DataType) {
    match &data_type {
        DataType::Void => print!("void"),
        DataType::I8 => print!("i8"),
        DataType::U8 => print!("u8"),
        DataType::I16 => print!("i16"),
        DataType::U16 => print!("u16"),
        DataType::I32 => print!("i32"),
        DataType::U32 => print!("u32"),
        DataType::I64 => print!("i64"),
        DataType::U64 => print!("u64"),
        DataType::String => print!("string"),
        DataType::Char => print!("char"),
        DataType::Bool => print!("bool"),
    }
}

