//! This modules serves to retrieve messages previously sent to a notify.run endpoint.

use chrono::{DateTime, FixedOffset, Utc};
use std::fmt::Display;

/// A message that was sent to a notify.run endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    content: String,
    time: DateTime<FixedOffset>,
}

impl Message {
    /// Create a message from a content and a timestamp.
    pub(crate) fn new(content: String, time: DateTime<FixedOffset>) -> Message {
        Message { content, time }
    }

    /// Returns the textual content of the message.
    pub fn content(&self) -> &String {
        &self.content
    }

    /// Returns the UTC timestamp when the message was received by its notify.run endpoint.
    pub fn time(&self) -> &DateTime<FixedOffset> {
        &self.time
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]: {}",
            self.time
                .with_timezone(&Utc)
                .format("%Y-%m-%d %H:%M:%S UTC"),
            self.content
        )
    }
}
