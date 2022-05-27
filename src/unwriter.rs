use crate::ast::*;

pub fn unwrite(file : AstFile) {
    for func in &file.functions {
        unwrite_function(func);
    }
}

fn unwrite_function(func : &AstFunction) {
    println!("func {} is", func.name);
    unwrite_block(&func.block, 0);
    println!("");
}

fn unwrite_block(block : &AstStatement, indent : i32) {
    for stmt in &block.statements {
        unwrite_statement(&stmt, indent+4);
    }
    
    for _i in 0 .. indent {
        print!(" ");
    }
    println!("end");
}

fn unwrite_statement(stmt : &AstStatement, indent : i32) {
    for _i in 0 .. indent {
        print!(" ");
    }
    
    match &stmt.ast_type {
        AstType::VarDec => {
            print!("var {} : ", stmt.name);
            unwrite_data_type(&stmt.data_type);
            print!(" := ");
            unwrite_expression(&stmt.expr, true);
        },
    
        AstType::CallStmt => {
            print!("{}", stmt.name);
            //if stmt.expr.ast_type != AstType::ExprList {
                print!("(");
                unwrite_expression(&stmt.expr, false);
                print!(")");
            /*} else {
                unwrite_expression(&stmt.expr);
            }*/
        },
        
        _ => {},
    }
    
    println!(";");
}

fn unwrite_expression(expr : &AstExpression, ignore_lval : bool) {
    match &expr.ast_type {
        //
        // Binary operators
        //
        AstType::Assign => {
            if !ignore_lval {
                unwrite_expression(&expr.args[0], false);
                print!(" := ");
            }
            unwrite_expression(&expr.args[1], false);
        },
        
        AstType::Add | AstType::Sub
        | AstType::Mul | AstType::Div | AstType::Mod
        | AstType::And | AstType::Or | AstType::Xor 
        | AstType::Eq | AstType::Ne
        | AstType::Gt | AstType::Ge | AstType::Lt | AstType::Le => {
            unwrite_expression(&expr.args[0], false);
            match &expr.ast_type {
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
            unwrite_expression(&expr.args[1], false);
        },
        
        //
        // Literals and primary expressions
        //
        AstType::Id => print!("{}", expr.string_value),
        AstType::IntLiteral => print!("{}", expr.int_value),
        AstType::StringLiteral => print!("{:?}", expr.string_value),
        AstType::CharLiteral => print!("{:?}", expr.char_value),
        AstType::BoolLiteral(val) => print!("{}", val),
        
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

