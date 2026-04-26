// Copyright (c) 2026 Sebastian Ibanez

use std::{collections::HashSet, env::args, hash::Hash};

use crate::error::Error;

/// Program arguments.
pub struct CliArgs {
    flags: HashSet<CliFlag>,
    file_name: String, // Note: crabcc only handles a single source file (for now).
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum CliFlag {
    Help,
    Lex,
    Parse,
    Codegen,
    Debug,
}

impl CliArgs {
    fn init(flags: HashSet<CliFlag>, file_name: String) -> Self {
        Self { flags, file_name }
    }

    pub fn collect_args() -> Result<Self, Error> {
        let mut file = None;
        let mut flags = HashSet::new();
        let raw_args: Vec<String> = args().skip(1).collect();
        for arg in raw_args {
            match arg.as_str() {
                "-h" | "--help" => {
                    flags.insert(CliFlag::Help);
                }
                "-l" | "--lex" => {
                    flags.insert(CliFlag::Lex);
                }
                "-p" | "--parse" => {
                    flags.insert(CliFlag::Parse);
                }
                "-c" | "--codegen" => {
                    flags.insert(CliFlag::Codegen);
                }
                "-d" | "--debug" => {
                    flags.insert(CliFlag::Debug);
                }
                // File name
                file_name if !file_name.starts_with("--") && !file_name.starts_with("-") => {
                    file = Some(file_name.to_string());
                }
                // Invalid argument
                invalid_arg => return Err(Error::InvalidCliFlag(invalid_arg.to_string())),
            }
        }

        if let Some(file_name) = file {
            return Ok(CliArgs::init(flags, file_name));
        }

        Err(Error::MissingCliFileName)
    }

    pub fn find_flag(&self, flag: CliFlag) -> bool {
        self.flags.get(&flag).is_some()
    }

    pub fn get_file_name(&self) -> String {
        self.file_name.clone()
    }
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
