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
        AstType::Assign => {
            if !ignore_lval {
                unwrite_expression(&expr.args[0], false);
                print!(" := ");
            }
            unwrite_expression(&expr.args[1], false);
        },
    
        AstType::IntLiteral => print!("{}", expr.int_value),
        AstType::StringLiteral => print!("{:?}", expr.string_value),
        
        _ => {},
    }
}

fn unwrite_data_type(data_type : &DataType) {
    match &data_type {
        DataType::Void => print!("void"),
        DataType::I32 => print!("i32"),
    }
}

