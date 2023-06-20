//! This module contains every error type used in the `notify-run` crate.

use std::io;
use thiserror::Error;

/// Utility wrapper error used to take in every error that `notify-run` could emit.
#[derive(Error, Debug)]
pub enum NotifyError {
    #[error("url error > {0}")]
    UrlError(#[from] UrlError),
    #[error("server error > {0}")]
    ServerError(#[from] ServerError),
    #[error("config error > {0}")]
    ConfigError(#[from] ConfigError),
}

/// The error used when the url passed is not valid.
#[derive(Error, Debug)]
pub enum UrlError {
    #[error("the provided text could not be parsed as a valid URL, provided: {text:?}")]
    ParseError { text: String },
    #[error("the provided URL scheme was neither 'http' nor 'https', provided: {scheme:?}")]
    InvalidScheme { scheme: String },
}

/// The error used when the communication to the notify server was not successful.
#[derive(Error, Debug)]
pub enum ServerError {
    #[error("wrong server url > {0}")]
    Url(#[from] UrlError),
    #[error("request unsuccessful > {0}")]
    Response(#[from] reqwest::Error),
    #[error("response parsing failed: {0}")]
    Parse(String),
}

/// The error used when the configuration of the notify server could not be retrieved successfully.
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
