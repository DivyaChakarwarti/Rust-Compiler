//! Generate Assembly

use crate::parser;

/// Generate assembly from AST
pub fn generate(ast: parser::AST) -> String {
    let return_value = extract_return_value_from_ast(ast);
    let asm = format!(
        ".globl _main\n\
         .p2align 2\n\
         _main:\n\
             mov w0, {}\n\
             ret\n",
        return_value
    );
    asm
}

/// Helper function to extract return value from the AST
fn extract_return_value_from_ast(ast: parser::AST) -> i64 {
    let return_statement = &ast.program.functions[0].body[0];
    match return_statement {
        parser::Statement::Return(expression) => match expression {
            parser::Expression::Const(lit) => *lit,
        },
    }
}

pub fn main() {

}