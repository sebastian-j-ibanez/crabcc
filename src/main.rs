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
            let tokens = lex_input(&mut chars)?;
        }
        CliFlag::Parse => todo!(),
        CliFlag::Codegen => todo!(),
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
    let file_name = env::args().nth(1).ok_or(Error::MissingCliFlag)?;
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
fn read_file(file_name: String) -> Result<Vec<u8>, Error> {
    let mut file = File::open(file_name).map_err(|_| Error::FileNotFound)?;
    let mut buf: Vec<u8> = Vec::new();
    let _ = file
        .read_to_end(&mut buf)
        .map_err(|_| Error::UnableToReadFile)?;
    Ok(buf)
}

/// Tokenize input.
fn lex_input(chars: &mut Vec<u8>) -> Result<Vec<char>, Error> {
    todo!()
}

// TODO: finish lex_input, proably delete below code

/// Source code token.
enum TokenType {
    Identifier,
    Constant,
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
type TokenMap = (TokenType, Regex);

fn regex_token_map() -> Vec<TokenMap> {
    let mut v = Vec::new();
    v.push((
        TokenType::Identifier,
        Regex::new("[a-zA-Z_]\\w*\\b").unwrap(),
    ));
    v.push((TokenType::Constant, Regex::new("[0-9]+\\b").unwrap()));
    v.push((TokenType::Int, Regex::new("int\\b").unwrap()));
    v.push((TokenType::Void, Regex::new("void\\b").unwrap()));
    v.push((TokenType::Return, Regex::new("return\\b").unwrap()));
    v.push((TokenType::OpenParen, Regex::new("\\(").unwrap()));
    v.push((TokenType::CloseParen, Regex::new("\\)").unwrap()));
    v.push((TokenType::OpenBrace, Regex::new("{").unwrap()));
    v.push((TokenType::CloseBrace, Regex::new("}").unwrap()));
    v.push((TokenType::SemiColon, Regex::new(";").unwrap()));
    v
}
