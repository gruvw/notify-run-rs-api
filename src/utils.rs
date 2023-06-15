use chrono::DateTime;
use serde_json::Value;
use url::Url;

use crate::{
    error::{ServerError, UrlError},
    message::Message,
    notify::{MESSAGE_KEY, TIME_KEY},
};

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

pub fn decode_msg(msg: &Value) -> Result<Message, ServerError> {
    let content = msg
        .get(MESSAGE_KEY)
        .ok_or(ServerError::Parse(
            "JSON response does not contain message".to_string(),
        ))?
        .as_str()
        .ok_or(ServerError::Parse(
            "JSON response message content should be text".to_string(),
        ))?;
    let time = DateTime::parse_from_rfc3339(
        msg.get(TIME_KEY)
            .ok_or(ServerError::Parse(
                "JSON response message content should have timestamp".to_string(),
            ))?
            .as_str()
            .ok_or(ServerError::Parse(
                "JSON response message timestamp should be text".to_string(),
            ))?,
    )
    .map_err(|_| ServerError::Parse("Could not parse timestamp".to_string()))?;

    Ok(Message::new(content.to_string(), time))
}
