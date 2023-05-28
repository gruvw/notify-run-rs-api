use url::Url;

use crate::error::UrlError;

pub fn parse_url(text: &str) -> Result<Url, UrlError> {
    if let Ok(url) = Url::parse(text) {
        if url.scheme() == "http" || url.scheme() == "https" {
            Ok(url)
        } else {
            Err(UrlError::InvalidScheme(format!(
                "The provided URL scheme was neither 'http' nor 'https'. Provided: {}",
                text
            )))
        }
    } else {
        Err(UrlError::ParseError(format!(
            "The provided text could not be parsed as a valid URL. Provided: {}",
            text
        )))
    }
}
