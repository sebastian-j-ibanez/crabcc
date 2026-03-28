// Copyright (c) 2026 Sebastian Ibanez

use crate::tokens::{Token, TokenType};
use regex::Regex;

use crate::error::Error;

/// Tokenize input.
pub fn lex_input(input_chars: &mut Vec<char>) -> Result<Vec<Token>, Error> {
    let original_input = input_chars.clone();
    let mut cursor = 0;
    let mut tokens: Vec<Token> = Vec::new();
    while !input_chars.is_empty() {
        // Trim any leading whitespace.
        let whitespace_count = trim_whitespace(input_chars);
        cursor += whitespace_count;

        // Catch when input_chars is only whitespace,
        // causing while loop to run on empty input.
        if input_chars.is_empty() {
            break;
        };

        // Ignore single line comments.
        loop {
            if input_chars.starts_with(&['/', '/']) {
                // Throw away line..
                let newline_index = input_chars
                    .iter()
                    .position(|c| *c == '\n')
                    .unwrap_or(input_chars.len());
                input_chars.drain(..=newline_index);
                cursor += newline_index + 1;

                // Trim any leading whitespace.
                let whitespace_count = trim_whitespace(input_chars);
                cursor += whitespace_count;
            } else {
                break;
            }
        }

        // Ignore multi line comments.
        loop {
            if input_chars.starts_with(&['/', '*']) {
                let mut comment_end_index: Option<usize> = None;
                for i in 0..input_chars.len() - 1 {
                    if input_chars[i] == '*' && input_chars[i + 1] == '/' {
                        comment_end_index = Some(i + 1);
                    }
                }

                if let Some(index) = comment_end_index {
                    input_chars.drain(..=index);
                    cursor += index + 1;
                } else {
                    eprintln!("unfinished multi-line comment at: {}", cursor - 1);
                    return Err(Error::UnfinishedMultilineComment);
                }

                // Trim any leading whitespace.
                let whitespace_count = trim_whitespace(input_chars);
                cursor += whitespace_count;
            } else {
                break;
            }
        }

        // Find the longest match to token type
        let token_regex_map = token_regex_map();
        let (token_type, index): (TokenType, usize) = token_regex_map
            .iter()
            .filter_map(|tr| tr.longest_match(input_chars))
            .max_by_key(|(_, index)| *index)
            .ok_or_else(|| {
                eprintln!(
                    "unexpected token at index {}: {}",
                    cursor, original_input[cursor]
                );
                eprintln!("rest of input: {:?}", input_chars);
                Error::LexError
            })?;

        let value: String = input_chars[0..index + 1].iter().collect();
        let token_type = match value.as_str() {
            "int" => TokenType::IntKeyword,
            "void" => TokenType::VoidKeyword,
            "return" => TokenType::ReturnKeyword,
            _ => token_type,
        };

        // Add substring to tokens.
        let token = Token::new(&value, token_type);

        // Remove matched substring from chars.
        input_chars.drain(..=index);
        cursor += index + 1;

        match token_type {
            TokenType::Identifier | TokenType::Constant => {
                if let Some(&next_char) = input_chars.first() {
                    if next_char.is_ascii_alphanumeric() || next_char == '_' {
                        eprintln!("invalid identifier or constant");
                        return Err(Error::LexError);
                    }
                }
            }
            _ => {}
        }

        tokens.push(token);
    }
    Ok(tokens)
}

/// Trim leading whitespace from `Vec<char>`
/// returning number of bytes trimmed.
fn trim_whitespace(chars: &mut Vec<char>) -> usize {
    let first_alphanum_index = chars
        .iter()
        .position(|b| !b.is_ascii_whitespace())
        .unwrap_or(chars.len());
    chars.drain(..first_alphanum_index);

    first_alphanum_index
}

/// Map token to corresponding regex.
struct TokenMap(TokenType, Regex);

impl TokenMap {
    /// Return end index of longest possible match.
    fn longest_match(&self, chars: &Vec<char>) -> Option<(TokenType, usize)> {
        let mut longest_match_index = None;
        for i in 0..chars.len() {
            let temp: String = chars[0..=i].iter().collect();
            if self.1.captures(&temp).is_some() {
                longest_match_index = Some(temp.len() - 1);
            } else {
                break;
            }
        }
        longest_match_index.map(|idx| (self.0, idx))
    }
}

fn token_regex_map() -> Vec<TokenMap> {
    let mut v = Vec::new();
    v.push(TokenMap(TokenType::OpenParen, Regex::new("^\\($").unwrap()));
    v.push(TokenMap(
        TokenType::CloseParen,
        Regex::new("^\\)$").unwrap(),
    ));
    v.push(TokenMap(TokenType::OpenBrace, Regex::new("^\\{$").unwrap()));
    v.push(TokenMap(TokenType::CloseBrace, Regex::new("^}$").unwrap()));
    v.push(TokenMap(TokenType::SemiColon, Regex::new("^;$").unwrap()));
    v.push(TokenMap(
        TokenType::Identifier,
        Regex::new("^[a-zA-Z_]\\w*\\b$").unwrap(),
    ));
    v.push(TokenMap(
        TokenType::Constant,
        Regex::new("^[0-9]+\\b$").unwrap(),
    ));
    v
}
