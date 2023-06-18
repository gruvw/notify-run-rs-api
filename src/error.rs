// TODO //!

use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotifyError {
    #[error("url error > {0}")]
    UrlError(#[from] UrlError),
    #[error("server error > {0}")]
    ServerError(#[from] ServerError),
    #[error("config error > {0}")]
    ConfigError(#[from] ConfigError),
}

#[derive(Error, Debug)]
pub enum UrlError {
    #[error("the provided text could not be parsed as a valid URL, provided: {text:?}")]
    ParseError { text: String },
    #[error("the provided URL scheme was neither 'http' nor 'https', provided: {scheme:?}")]
    InvalidScheme { scheme: String },
}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("wrong server url > {0}")]
    Url(#[from] UrlError),
    #[error("request unsuccessful > {0}")]
    Response(#[from] reqwest::Error),
    #[error("response parsing failed: {0}")]
    Parse(String),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("could not read/write config > {0}")]
    File(#[from] io::Error),
    #[error("invalid JSON in config > {0}")]
    Parse(#[from] serde_json::Error),
    #[error("missing the key in JSON: {0}")]
    KeyNotFound(String),
    #[error("bad URL in config > {0}")]
    UrlError(#[from] UrlError),
}
