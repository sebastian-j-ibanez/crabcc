// Copyright (c) 2026 Sebastian Ibanez

use std::fmt::Display;

pub enum Error {
    InvalidCliFlag(String),
    MissingCliFlag,
    FileNotFound,
    UnableToReadFile,
    LexError,
    Unimplemented,
    UnfinishedMultilineComment,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCliFlag(flag) => write!(f, "invalid cli flag: {}", flag),
            Self::MissingCliFlag => write!(f, "expected cli flag"),
            Self::FileNotFound => write!(f, "file not found"),
            Self::UnableToReadFile => write!(f, "unable to read file"),
            Self::LexError => write!(f, "unable to lex file"),
            Self::Unimplemented => write!(f, "unimplemented"),
            Self::UnfinishedMultilineComment => write!(f, "unfinished multiline comment"),
        }
    }
}
