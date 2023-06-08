use chrono::{DateTime, FixedOffset};

use crate::notify::Notify;

pub struct Message {
    content: String,
    time: DateTime<FixedOffset>,
}
