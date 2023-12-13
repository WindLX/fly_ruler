use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CoreError {
    TestError,
}

impl Error for CoreError {}

impl Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TestError => write!(f, "CoreError"),
        }
    }
}
