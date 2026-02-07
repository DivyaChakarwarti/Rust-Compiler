//! Generate Assembly

use crate::lexer;
use crate::parser;

/// Generate assembly from AST
pub fn generate(ast: parser::AST) -> String {
    let mut asm = String::from(
        ".globl _main\n\
         .p2align 2\n\
         _main:\n",
    );

    let func = &ast.program.functions[0];

    match &func.body[0] {
        parser::Statement::Return(expression) => {
            eval_expression(&mut asm, expression);
            asm.push_str("    ret\n");
        }
    }
    asm
}

/// Helper function to evaluate expression
fn eval_expression(asm: &mut String, expression: &parser::Expression) {
    match expression {
        parser::Expression::Const(lit) => {
            asm.push_str(&format!("    mov w0, #{}\n", lit));
        }
        parser::Expression::UnOp(op, inner_exp) => {
            eval_expression(asm, inner_exp);
            match op {
                lexer::Operator::Negation => {
                    asm.push_str("    neg w0, w0\n");
                }
                lexer::Operator::BitwiseNot => {
                    asm.push_str("    mvn w0, w0\n");
                }
                lexer::Operator::LogicalNot => {
                    asm.push_str("    cmp w0, #0\n");
                    asm.push_str("    cset w0, eq\n");
                }
            }
        }
    }
}

/// Helper function to extract return value from the AST
// fn eval_expression(ast: parser::AST) -> i64 {
//     let return_statement = &ast.program.functions[0].body[0];
//     match return_statement {
//         parser::Statement::Return(expression) => match expression {
//             parser::Expression::Const(lit) => *lit,
//             parser::Expression::UnOp(_, inner_exp) => {
//                 let inner_exp = *inner_exp;
//                 match inner_exp {
//                     parser::Expression::Const(lit) => *lit,
//                     _ => panic!("Unexpected expression"),
//                 }
//             }
//         },
//     }
// }

pub fn main() {}
