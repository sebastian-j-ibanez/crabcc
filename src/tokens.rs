// Copyright (c) 2026 Sebastian Ibanez

use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Token {
    pub raw_string: String,
    pub token_type: TokenType,
    pub index: usize,
}

impl Token {
    pub fn new(raw_string: String, token_type: TokenType, index: usize) -> Self {
        Self {
            raw_string,
            token_type,
            index,
        }
    }
}

/// Source code token.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    Literal,
    IntKeyword,
    VoidKeyword,
    ReturnKeyword,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    SemiColon,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Identifier => "identifier",
            TokenType::Literal => "literal",
            TokenType::IntKeyword => "int",
            TokenType::VoidKeyword => "void",
            TokenType::ReturnKeyword => "return",
            TokenType::OpenParen => "'('",
            TokenType::CloseParen => "')'",
            TokenType::OpenBrace => "'{'",
            TokenType::CloseBrace => "'}'",
            TokenType::SemiColon => "';'",
        };
        f.write_str(msg)
    }
}
