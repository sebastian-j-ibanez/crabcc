// Copyright (c) 2026 Sebastian Ibanez

use std::collections::VecDeque;

use crate::{
    error::Error,
    tokens::{Token, TokenType},
};

#[derive(Debug, Clone)]
pub enum Node {
    Program(Box<Node>),
    Function(Function),
    Return(Box<Node>),
    Literal(Literal),
}

// Function nodes: [NAME, BODY]
#[derive(Debug, Clone)]
pub struct Function {
    return_type: ReturnType,
    name: String,
    body: Box<Node>,
}

#[derive(Debug, Clone, Copy)]
pub enum Literal {
    Integer(i32),
    Float(f32),
    Bool(bool),
}

#[derive(Debug, Clone, Copy)]
pub enum ReturnType {
    Integer,
    Float,
    Bool,
}

impl Literal {
    pub fn from(token_string: String) -> Result<Self, Error> {
        if let Ok(int_val) = token_string.parse::<i32>() {
            return Ok(Self::Integer(int_val));
        }

        if let Ok(float_val) = token_string.parse::<f32>() {
            return Ok(Self::Float(float_val));
        }

        if let Ok(bool_val) = token_string.parse::<bool>() {
            return Ok(Self::Bool(bool_val));
        }

        let msg = format!("unable to parse '{}' into literal", token_string);
        Err(Error::ParserError(msg))
    }
}

pub fn parse_tokens(tokens: &mut VecDeque<Token>) -> Result<Node, Error> {
    let func_node = try_parse_function(tokens)?;
    let program_node = Node::Program(Box::new(func_node));
    Ok(program_node)
}

fn try_parse_function(tokens: &mut VecDeque<Token>) -> Result<Node, Error> {
    expect(TokenType::IntKeyword, tokens)?;
    let func_name_token = expect(TokenType::Identifier, tokens)?;
    expect(TokenType::OpenParen, tokens)?;
    expect(TokenType::CloseParen, tokens)?;
    expect(TokenType::OpenBrace, tokens)?;
    expect(TokenType::ReturnKeyword, tokens)?;
    let return_val_token = expect(TokenType::Literal, tokens)?;
    expect(TokenType::SemiColon, tokens)?;
    expect(TokenType::CloseBrace, tokens)?;

    // Construct return AST node.
    let return_val_literal = Literal::from(return_val_token.raw_string)?;
    let return_type = match return_val_literal {
        Literal::Integer(_) => ReturnType::Integer,
        Literal::Float(_) => ReturnType::Float,
        Literal::Bool(_) => ReturnType::Bool,
    };

    let return_val_node = Node::Literal(return_val_literal);

    // Construct function AST node.
    let body = Box::new(Node::Return(Box::new(return_val_node)));
    let func_node = Node::Function(Function {
        return_type,
        name: func_name_token.raw_string,
        body,
    });

    Ok(func_node)
}

/// Check if next token matches expected type.
fn expect(expected_type: TokenType, tokens: &mut VecDeque<Token>) -> Result<Token, Error> {
    let token = tokens.pop_front().ok_or({
        let msg = format!("expected {}, got unexpected end of file", expected_type);
        Error::ParserError(msg)
    })?;

    if token.token_type != expected_type {
        let msg = format!(
            "at index {}: expected {}, but got {}",
            token.index, expected_type, token.token_type
        );
        return Err(Error::ParserError(msg));
    }

    Ok(token)
}

/*
int main() {
    return 2;
}
*/
