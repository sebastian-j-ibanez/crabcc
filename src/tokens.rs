// Copyright (c) 2026 Sebastian Ibanez

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
}

impl Token {
    pub fn new(value: &str, token_type: TokenType) -> Self {
        Self {
            value: value.to_string(),
            token_type,
        }
    }
}

/// Source code token.
#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    Identifier,
    Constant,
    IntKeyword,
    VoidKeyword,
    ReturnKeyword,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    SemiColon,
}
