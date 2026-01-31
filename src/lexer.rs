//! Lexer

use std::path::Path;

/// Keywords: int, return
#[derive(Debug, Clone, PartialEq, Eq)]
enum Keyword {
    Int,
    Return,
}

/// Punctuation: { }, ( ), ;
#[derive(Debug, Clone, PartialEq, Eq)]
enum Punctuation {
    OpenBrace,
    CloseBrace,
    OpenParanthesis,
    CloseParanthesis,
    Semicolon,
}

/// Token types
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Keyword(Keyword),
    Ident(String),
    IntLit(i64),
    Punctuation(Punctuation),
}

/// Main lexer function
fn lex(file: &str) -> Vec<Token> {
    let bytes = file.as_bytes();
    let mut tokens: Vec<Token> = vec![];

    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];

        if is_whitespace(b) {
            i += 1;
            continue;
        }

        match b {
            b'{' | b'}' | b'(' | b')' | b';' => {
                tokens.push(punctuation_token_from_byte(b));
                i += 1;
                continue;
            }
            b if is_alpha_or_underscore(b) => {
                let st = i;
                i += 1;
                while i < bytes.len() && is_alnum_or_underscore(bytes[i]) {
                    i += 1;
                }
                let id = &file[st..i];

                match id {
                    "int" => tokens.push(Token::Keyword(Keyword::Int)),
                    "return" => tokens.push(Token::Keyword(Keyword::Return)),
                    _ => tokens.push(Token::Ident(id.to_string())),
                }
                continue;
            }
            b if is_digit(b) => {
                let st = i;
                i += 1;
                while i < bytes.len() && is_digit(bytes[i]) {
                    i += 1;
                }
                let lit = &file[st..i];
                let lit = lit.parse::<i64>().unwrap();
                tokens.push(Token::IntLit(lit));
                continue;
            }
            _ => {
                panic!("Invalid character: {}", b as char);
            }
        }
    }

    tokens
}

// Helper functions

fn is_whitespace(c: u8) -> bool {
    c == b' ' || c == b'\t' || c == b'\n' || c == b'\r'
}

fn is_digit(b: u8) -> bool {
    b >= b'0' && b <= b'9'
}

fn is_alpha_or_underscore(b: u8) -> bool {
    (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z') || b == b'_'
}

fn is_alnum_or_underscore(b: u8) -> bool {
    is_alpha_or_underscore(b) || is_digit(b)
}

fn punctuation_token_from_byte(b: u8) -> Token {
    return match b {
        b'{' => Token::Punctuation(Punctuation::OpenBrace),
        b'}' => Token::Punctuation(Punctuation::CloseBrace),
        b'(' => Token::Punctuation(Punctuation::OpenParanthesis),
        b')' => Token::Punctuation(Punctuation::CloseParanthesis),
        b';' => Token::Punctuation(Punctuation::Semicolon),
        _ => panic!("Invalid character: {}", b as char),
    };
}

/// For testing purposes
fn main() {
    let ex1 = lex("int main() { return 2; }");
    println!("{:?}", ex1);
}
