// Copyright (c) 2026 Sebastian Ibanez

use crate::tokens::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum Node {
    Program(Box<Node>),
    Function(Function),
    Return(Box<Node>),
    Constant(Constant),
}

// Function nodes: [NAME, BODY]
#[derive(Debug, Clone)]
pub struct Function {
    return_type: Constant,
    name: String,
    body: Box<Node>,
}

#[derive(Debug, Clone, Copy)]
pub enum Constant {
    Integer(i32),
    Float(f32),
    Bool(bool),
    Void,
}

pub fn parse_tokens(tokens: Vec<Token>) -> Node {
    todo!()
}

/*
int main() {
    return 2;
}
*/
