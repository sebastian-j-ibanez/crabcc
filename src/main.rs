// Copyright (c) 2026 Sebastian Ibanez

use std::{env, fs::File, io::Read};

use regex::Regex;

use crate::error::Error;

mod error;

fn main() {
    let exit_code = match run() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    };
    std::process::exit(exit_code);
}

fn run() -> Result<(), Error> {
    let (flag, file_name) = parse_args()?;
    match flag {
        CliFlag::Help => print_help(),
        CliFlag::Lex => {
            let mut chars = read_file(file_name)?;
            let _tokens = lex_input(&mut chars)?;
        }
        CliFlag::Parse | CliFlag::Codegen => return Err(Error::Unimplemented),
    }

    Ok(())
}

// CLI

enum CliFlag {
    Help,
    Lex,
    Parse,
    Codegen,
}

fn parse_args() -> Result<(CliFlag, String), Error> {
    let flag_string = env::args().nth(1).ok_or(Error::MissingCliFlag)?;
    let file_name = env::args().nth(2).ok_or(Error::MissingCliFlag)?;
    let flag = match flag_string.as_ref() {
        "-h" | "--help" => CliFlag::Help,
        "--lex" | "-l" => CliFlag::Lex,
        "--parse" | "-p" => CliFlag::Parse,
        "--codegen" | "-c" => CliFlag::Codegen,
        invalid_flag => {
            let error = Error::InvalidCliFlag(invalid_flag.to_string());
            eprintln!("error: {error}");
            println!("input 'crabcc --help' to see proper usage");
            return Err(error);
        }
    };
    Ok((flag, file_name))
}

fn print_help() {
    println!("The crabby C compiler\n");
    println!("Usage:\n\tcrabcc [flags] <source files>");
    println!("Flags:");
    println!("\t-h, ==help\t\tPrint this usage message.");
    println!("\t-l, --lex\t\tLex the provided source file.");
    println!("\t-p, --parse\t\tLex and parse the provided source file.");
    println!("\t-c, --codegen\t\tLex, parse, and generate assembly for the provided source file.");
}

// LEXER

/// Read source file.
fn read_file(file_name: String) -> Result<Vec<char>, Error> {
    let mut file = File::open(file_name).map_err(|_| Error::FileNotFound)?;
    let mut byte_buf: Vec<u8> = Vec::new();
    let _ = file
        .read_to_end(&mut byte_buf)
        .map_err(|_| Error::UnableToReadFile)?;
    let chars: Vec<char> = byte_buf.iter().map(|b| *b as char).collect();
    Ok(chars)
}

struct Token {
    _value: String,
    _token_type: TokenType,
}

impl Token {}

/// Tokenize input.
fn lex_input(input_chars: &mut Vec<char>) -> Result<Vec<Token>, Error> {
    let mut global_index = 0;
    let mut tokens: Vec<Token> = Vec::new();
    while !input_chars.is_empty() {
        // let is_identifier = Regex::new("^[a-zA-Z_]\\w*\\b").unwrap();
        // let temp = input_chars.clone().iter().collect::<String>();
        // println!("{}\nis identifier: {}", temp, is_identifier.is_match(&temp));

        // Trim any leading whitespace.
        let first_char_index = input_chars
            .iter()
            .position(|b| !b.is_ascii_whitespace())
            .unwrap_or(input_chars.len());
        input_chars.drain(..first_char_index);
        global_index += first_char_index;

        // Ignore single line comments.
        if input_chars.starts_with(&['/', '/']) {
            let newline_index = input_chars
                .iter()
                .position(|c| *c == '\n')
                .unwrap_or(input_chars.len());
            input_chars.drain(..newline_index + 1);
            global_index += newline_index;
        }

        // Ignore multi line comments.
        if input_chars.starts_with(&['/', '*']) {
            // TODO: fix this
            let mut comment_end_index: Option<usize> = None; // If comment is unfinished, comment length is the rest of the input.
            for i in 0..input_chars.len() - 1 {
                if input_chars[i] == '*' && input_chars[i + 1] == '/' {
                    comment_end_index = Some(i + 1);
                }
            }

            if let Some(index) = comment_end_index {
                input_chars.drain(..=index);
            } else {
                eprintln!("unfinished multi-line comment at: {}", global_index);
                return Err(Error::UnfinishedMultilineComment);
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
                    global_index, input_chars[0]
                );
                eprintln!("rest of input: {:?}", input_chars);
                Error::LexError
            })?;

        let value: String = input_chars[0..index + 1].iter().collect();
        let token_type = match value.as_str() {
            "int" => TokenType::Int,
            "void" => TokenType::Void,
            "return" => TokenType::Return,
            _ => token_type,
        };

        // Add substring to tokens.
        let token = Token {
            _value: value,
            _token_type: token_type,
        };
        tokens.push(token);

        // Remove matched substring from chars.
        input_chars.drain(..index + 1);
        global_index += index + 1;
    }
    Ok(tokens)
}

/// Source code token.
#[derive(Debug, Copy, Clone)]
enum TokenType {
    Identifier,
    Constant,
    // Comment,
    Int,
    Void,
    Return,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    SemiColon,
}

/// Map token to corresponding regex.
struct TokenMap(TokenType, Regex);

impl TokenMap {
    /// Return end index of longest possible match.
    fn longest_match(&self, chars: &Vec<char>) -> Option<(TokenType, usize)> {
        let mut longest_match_index = None;
        for i in 1..=chars.len() {
            let temp: String = chars[0..i].iter().collect();
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
    // v.push(TokenMap(
    //     TokenType::Return,
    //     Regex::new("return\\b").unwrap(),
    // ));
    // v.push(TokenMap(TokenType::Int, Regex::new("int\\b").unwrap()));
    // v.push(TokenMap(TokenType::Void, Regex::new("void\\b").unwrap()));
    // v.push(TokenMap(TokenType::Comment, Regex::new("^//.*$").unwrap()));
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
