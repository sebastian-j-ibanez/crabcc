// Copyright (c) 2026 Sebastian Ibanez

// CLI

use std::env;

use crate::error::Error;

pub enum CliFlag {
    Help,
    Lex,
    Parse,
    Codegen,
}

pub fn parse_args() -> Result<(CliFlag, String), Error> {
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

pub fn print_help() {
    println!("The crabby C compiler\n");
    println!("Usage:\n\tcrabcc [flags] <source files>");
    println!("Flags:");
    println!("\t-h, --help\t\tPrint this usage message.");
    println!("\t-l, --lex\t\tLex the provided source file.");
    println!("\t-p, --parse\t\tLex and parse the provided source file.");
    println!("\t-c, --codegen\t\tLex, parse, and generate assembly for the provided source file.");
}
