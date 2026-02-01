mod lexer;
mod parser;
mod generate;

use std::{env, fs, process};

pub fn main() {
    // Read CLI args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprint!("Usage: rustcc <filename.c>");
        process::exit(1);
    }

    let input_path: &String = &args[1];

    // Read source file
    let source = fs::read_to_string(input_path).expect("Failed to read file");

    // Lex source
    let tokens = lexer::lex(&source);

    // Parse tokens
    let ast = parser::parse(tokens).expect("Parsing failed");

    // Generate assembly
    let asm = generate::generate(ast);

    // Write assembly to file
    let asm_path = format! {"{}.s", input_path};
    fs::write(&asm_path, asm).expect("Failed to write assembly file");

    let output_exe = input_path.trim_end_matches(".c");

    // Compile assembly
    let status = process::Command::new("gcc")
        .args([&asm_path, "-o", output_exe])
        .status()
        .expect("Failed to compile");

    if !status.success() {
        eprintln!("gcc failed");
        process::exit(1);
    }
}

/// Helper function to extract return value from tokens - Temprory usage only!
fn extract_return_value(tokens: Vec<lexer::Token>) -> i64 {
    let mut i = 0;
    while i < tokens.len() - 1 {
        if let lexer::Token::Keyword(lexer::Keyword::Return) = tokens[i] {
            if let lexer::Token::IntLit(lit) = tokens[i + 1] {
                return lit;
            }
        }
        i += 1;
    }
    panic!("No return value found");
}