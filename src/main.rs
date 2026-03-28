// Copyright (c) 2026 Sebastian Ibanez

use std::{fs::File, io::Read};

use crate::{cli::CliFlag, error::Error};

mod cli;
mod error;
mod lexer;
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

fn run() -> Result<(), Error> {
    let (flag, file_name) = cli::parse_args()?;
    match flag {
        CliFlag::Help => cli::print_help(),
        CliFlag::Lex => {
            let mut chars = read_file(file_name)?;
            let _tokens = lexer::lex_input(&mut chars)?;
        }
        CliFlag::Parse | CliFlag::Codegen => return Err(Error::Unimplemented),
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
