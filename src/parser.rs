//! Parser

use crate::lexer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedError,
}

/// Abstract Syntax Tree (AST) types
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AST {
    pub program: Program,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Const(i64),
}

/// TokenStore is internal implementation.
/// It is used to store slice of tokens and a position.
/// Position indicates where I am in the slice.
struct TokenStore<'a> {
    pos: usize,
    tokens: &'a [lexer::Token],
}

impl<'a> TokenStore<'a> {
    /// Creates new TokenStore from slice of tokens.
    fn new(tokens: &'a [lexer::Token]) -> Self {
        Self { pos: 0, tokens }
    }

    /// Returns current token.
    fn peek(&self) -> Option<&lexer::Token> {
        self.tokens.get(self.pos)
    }

    /// Returns current token and move the position to next.
    fn next(&mut self) -> Option<lexer::Token> {
        let token = self.tokens.get(self.pos)?;
        self.pos += 1;
        Some(token.clone())
    }

    /// Checks if current token matches the expected one using a lambda function.
    /// Also, moves the position to next.
    fn expect(&mut self, expected: fn(&lexer::Token) -> bool) -> Result<lexer::Token, ParseError> {
        match self.next() {
            Some(token) if expected(&token) => Ok(token),
            Some(_) => Err(ParseError::UnexpectedError),
            None => Err(ParseError::UnexpectedError),
        }
    }
}

/// Main parser function
pub fn parse(tokens: Vec<lexer::Token>) -> Result<AST, ParseError> {
    let mut ts = TokenStore::new(&tokens);

    let program = parse_program(&mut ts)?;

    Ok(AST { program })
}

// Helper functions

fn parse_program(ts: &mut TokenStore) -> Result<Program, ParseError> {
    let function = parse_function(ts)?;
    Ok(Program {
        functions: vec![function],
    })
}

fn parse_function(ts: &mut TokenStore) -> Result<Function, ParseError> {
    // Expect int keyword
    ts.expect(|token| match token {
        lexer::Token::Keyword(lexer::Keyword::Int) => true,
        _ => false,
    })?;

    // Expect main keyword
    ts.expect(|token| match token {
        lexer::Token::Ident(ident) => ident == "main",
        _ => false,
    })?;

    // Expect open paranthesis
    ts.expect(|token| match token {
        lexer::Token::Punctuation(lexer::Punctuation::OpenParanthesis) => true,
        _ => false,
    })?;

    // Expect close paranthesis
    ts.expect(|token| match token {
        lexer::Token::Punctuation(lexer::Punctuation::CloseParanthesis) => true,
        _ => false,
    })?;

    // Expect open brace
    ts.expect(|token| match token {
        lexer::Token::Punctuation(lexer::Punctuation::OpenBrace) => true,
        _ => false,
    })?;

    let body = parse_statement(ts)?;

    // Expect close brace
    ts.expect(|token| match token {
        lexer::Token::Punctuation(lexer::Punctuation::CloseBrace) => true,
        _ => false,
    })?;

    Ok(Function {
        name: "main".to_string(),
        body: vec![body],
    })
}

fn parse_statement(ts: &mut TokenStore) -> Result<Statement, ParseError> {
    // Expect return keyword
    ts.expect(|token| match token {
        lexer::Token::Keyword(lexer::Keyword::Return) => true,
        _ => false,
    })?;

    let expression = parse_expression(ts)?;

    // Expect Semi colon
    ts.expect(|token| match token {
        lexer::Token::Punctuation(lexer::Punctuation::Semicolon) => true,
        _ => false,
    })?;

    Ok(Statement::Return(expression))
}

fn parse_expression(ts: &mut TokenStore) -> Result<Expression, ParseError> {
    // Expect an integer
    let ret = ts.expect(|token| match token {
        lexer::Token::IntLit(_) => true,
        _ => false,
    })?;

    let value = match ret {
        lexer::Token::IntLit(lit) => lit,
        _ => unreachable!(),
    };

    Ok(Expression::Const(value))
}

fn main() {
    let ex1 = lexer::lex("int main() { return 2; }");
    let ast = parse(ex1);
    println!("{:?}", ast);

    let ex2 = lexer::lex("int main() { }");
    let ast = parse(ex2);
    println!("{:?}", ast);
}
