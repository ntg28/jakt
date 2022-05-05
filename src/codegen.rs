use crate::parser::{Block, Expression, Function, ParsedFile, Statement};

pub fn translate(file: &ParsedFile) -> String {
    let mut output = String::new();

    output.push_str("#include \"runtime/lib.h\"\n");
    output.push_str("#include<stdio.h>\n");

    for fun in &file.funs {
        let fun_output = translate_function(fun);

        output.push_str(&fun_output);
        output.push('\n');
    }

    output
}

fn translate_function(fun: &Function) -> String {
    let mut output = String::new();

    output.push_str("void ");
    output.push_str(&fun.name);
    output.push_str("()");

    let block = translate_block(&fun.block);
    output.push_str(&block);

    output
}

fn translate_block(block: &Block) -> String {
    let mut output = String::new();

    output.push_str("{\n");

    for stmt in &block.stmts {
        let stmt = translate_stmt(stmt);

        output.push_str(&stmt);
    }

    output.push_str("}\n");

    output
}

fn translate_stmt(stmt: &Statement) -> String {
    let mut output = String::new();
    match stmt {
        Statement::Expression(expr) => {
            let expr = translate_expr(&expr);
            output.push_str(&expr)
        }
        Statement::Defer(block) => {
            // NOTE: We let the preprocessor generate a unique name for the RAII helper.
            output.push_str("#define __SCOPE_GUARD_NAME __scope_guard_ ## __COUNTER__\n");
            output.push_str("ScopeGuard __SCOPE_GUARD_NAME  ([&] \n");
            output.push_str("#undef __SCOPE_GUARD_NAME\n");
            output.push_str(&translate_block(block));
            output.push_str(")");
        }
    }

    output.push_str(";\n");

    output
}

fn translate_expr(expr: &Expression) -> String {
    let mut output = String::new();

    match expr {
        Expression::QuotedString(qs) => {
            output.push('"');
            output.push_str(qs);
            output.push('"');
        }
        Expression::Int64(qs) => {
            output.push_str(&qs.to_string());
        }
        Expression::Call(call) => {
            if call.name == "print" {
                output.push_str("printf");
            } else {
                output.push_str(&call.name);
            }
            output.push('(');
            for param in &call.args {
                output.push_str(&translate_expr(&param.1));
            }
            output.push(')');
        }
    }

    output
}