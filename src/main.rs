// Copyright (c) 2026 Sebastian Ibanez

use std::process::ExitCode;
use std::{fs::File, io::Read};

use crate::cli::{CliArgs, print_help};
use crate::parser::parse_tokens;
use crate::{cli::CliFlag, error::Error};

mod cli;
mod error;
mod lexer;
mod parser;
pub mod tokens;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Run compiler.
fn run() -> Result<(), Error> {
    let args = CliArgs::collect_args()?;

    // Help flag is processed first and overrides all other arguments.
    if args.find_flag(CliFlag::Help) {
        print_help();
        return Ok(());
    }

    let lexer = args.find_flag(CliFlag::Lex);
    let parser = args.find_flag(CliFlag::Parse);
    let codegen = args.find_flag(CliFlag::Codegen);
    let debug = args.find_flag(CliFlag::Debug);
    let run_all = !lexer && !parser && !codegen; // Run all 3 stages by default.

    // Run lexer
    if lexer || parser || codegen || run_all {
        let mut chars = read_file(args.get_file_name())?;
        let mut tokens = lexer::lex_input(&mut chars)?;
        if debug {
            println!("[INFO] lexer output:");
            dbg!(&tokens);
        }

        // Run parser
        if parser || codegen || run_all {
            let ast = parse_tokens(&mut tokens)?;
            if debug {
                println!("[INFO] parser output:");
                dbg!(&ast);
            }

            // Run codegen
            if codegen || run_all {
                todo!();
            }
        }
    }

    Ok(())
}

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
