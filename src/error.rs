use std::fmt::Display;

use reqwest::StatusCode;

#[derive(Debug)]
pub enum UrlError {
    ParseError(String),
    InvalidScheme(String),
}

impl Display for UrlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Url > ")?;
        write!(
            f,
            "{}",
            match self {
                Self::ParseError(text) => format!("ParseError: {}", text),
                Self::InvalidScheme(text) => format!("InvalidScheme: {}", text),
            }
        )
    }
}

#[derive(Debug)]
pub enum ServerError {
    Url(UrlError),
    Response(StatusCode, reqwest::Error),
    Parse(String),
    Connection(reqwest::Error),
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Server > ")?;
        match self {
            Self::Url(err) => err.fmt(f),
            Self::Response(code, err) => {
                write!(f, "Response: status code {}, ", code)?;
                err.fmt(f)
            }
            Self::Parse(text) => write!(f, "Parse: {}", text),
            Self::Connection(err) => {
                write!(f, "Connection: ")?;
                err.fmt(f)
            }
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Access(String),
    Parse(String),
    Write(String),
    UrlError(UrlError),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Config > ")?;
        match self {
            Self::Access(text) => write!(f, "ConfigNotFound: {}", text),
            Self::Parse(text) => write!(f, "Parse: {}", text),
            Self::Write(text) => write!(f, "Parse: {}", text),
            Self::UrlError(err) => err.fmt(f),
        }
    }
}
