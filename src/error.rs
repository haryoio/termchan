use std::{error::Error, fmt, io};

#[derive(Debug)]
pub enum TermchanError {
    CookieError(String),
    ReqwestError(String),
    IoError(io::Error),
    ConfigError(String),
    LoginError(String),
    AnyhowError(anyhow::Error),
}

impl From<io::Error> for TermchanError {
    fn from(error: io::Error) -> Self {
        TermchanError::IoError(error)
    }
}

impl From<reqwest::Error> for TermchanError {
    fn from(error: reqwest::Error) -> Self {
        TermchanError::ReqwestError(error.to_string())
    }
}

impl From<anyhow::Error> for TermchanError {
    fn from(error: anyhow::Error) -> Self {
        TermchanError::AnyhowError(error)
    }
}

impl fmt::Display for TermchanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TermchanError::CookieError(error) => write!(f, "cookie error: {}", error),
            TermchanError::ReqwestError(error) => write!(f, "reqwest error: {}", error),
            TermchanError::IoError(error) => write!(f, "io error: {}", error),
            TermchanError::ConfigError(error) => write!(f, "config error: {}", error),
            TermchanError::LoginError(error) => write!(f, "login error: {}", error),
            TermchanError::AnyhowError(error) => write!(f, "anyhow error: {}", error),
        }
    }
}

impl Error for TermchanError {
    fn description(&self) -> &str {
        match *self {
            TermchanError::CookieError(_) => "cookie error",
            TermchanError::ReqwestError(_) => "reqwest error",
            TermchanError::IoError(_) => "io error",
            TermchanError::ConfigError(_) => "config error",
            TermchanError::LoginError(_) => "login error",
            TermchanError::AnyhowError(_) => "anyhow error",
        }
    }
}
