// Copyright (c) 2026 Sebastian Ibanez

use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    InvalidCliFlag(String),
    MissingCliFileName,
    FileNotFound,
    UnableToReadFile,
    LexError,
    UnfinishedMultilineComment,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCliFlag(flag) => write!(f, "invalid cli flag: {}", flag),
            Self::MissingCliFileName => write!(f, "expected file name"),
            Self::FileNotFound => write!(f, "file not found"),
            Self::UnableToReadFile => write!(f, "unable to read file"),
            Self::LexError => write!(f, "unable to lex file"),
            Self::UnfinishedMultilineComment => write!(f, "unfinished multiline comment"),
        }
    }
}
