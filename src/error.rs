#[derive(Debug)]
pub enum UrlError {
    ParseError(String),
    InvalidScheme(String),
}

#[derive(Debug)]
pub enum ServerError {
    Url(UrlError),
    Response(reqwest::Error),
    Connection(reqwest::Error),
}

#[derive(Debug)]
pub enum ConfigError<'a> {
    ConfigNotFound(&'a str),
    ServerError(ServerError),
}
