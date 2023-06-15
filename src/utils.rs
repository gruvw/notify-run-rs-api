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
        let scheme = url.scheme();
        if scheme == "http" || scheme == "https" {
            Ok(url)
        } else {
            Err(UrlError::InvalidScheme {
                scheme: scheme.into(),
            })
        }
    } else {
        Err(UrlError::ParseError { text: text.into() })
    }
}

pub fn decode_msg(msg: &Value) -> Result<Message, ServerError> {
    let content = msg
        .get(MESSAGE_KEY)
        .ok_or(ServerError::Parse(
            "JSON response does not contain message".into(),
        ))?
        .as_str()
        .ok_or(ServerError::Parse(
            "JSON response message content should be text".into(),
        ))?;
    let time = DateTime::parse_from_rfc3339(
        msg.get(TIME_KEY)
            .ok_or(ServerError::Parse(
                "JSON response message content should have timestamp".into(),
            ))?
            .as_str()
            .ok_or(ServerError::Parse(
                "JSON response message timestamp should be text".into(),
            ))?,
    )
    .map_err(|_| ServerError::Parse("Could not parse timestamp".into()))?;

    Ok(Message::new(content.into(), time))
}
