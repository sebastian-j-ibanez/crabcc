// Copyright (c) 2026 Sebastian Ibanez

use std::{fs::File, io::Read};

use crate::cli::{CliArgs, print_help};
use crate::{cli::CliFlag, error::Error};

mod cli;
mod error;
mod lexer;
mod parser;
pub mod tokens;

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
    let run_all = !lexer && !parser && !codegen; // Run all 3 stages by default.

    // Run lexer
    if lexer || run_all {
        let mut chars = read_file(args.get_file_name())?;
        let _tokens = lexer::lex_input(&mut chars)?;
        if args.find_flag(CliFlag::Debug) {
            dbg!(_tokens);
        }

        // Run parser
        if parser || run_all {
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
