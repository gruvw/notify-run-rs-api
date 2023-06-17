// TODO //!

use std::fmt::Display;

use chrono::{DateTime, FixedOffset, Utc};

pub struct Message {
    content: String,
    time: DateTime<FixedOffset>,
}

impl Message {
    pub(crate) fn new(content: String, time: DateTime<FixedOffset>) -> Message {
        Message { content, time }
    }

    pub fn content(&self) -> &String {
        &self.content
    }

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
