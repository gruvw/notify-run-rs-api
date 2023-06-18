//! A Rust client for sending notifications to your own phone or desktop.
//!
//! # Usage
//!
//! ```no_run
//! use notify_run::Notify;
//!
//! // Register a new notify.run endpoint
//! let notify = Notify::register()?;
//!
//! // Display information to subscribe a device
//! println!("How to subscribe:\n{}", notify);
//!
//! // Send a notification message
//! notify.send("Hello world!")?;
//! # Ok::<(), notify_run::error::NotifyError>(())
//! ```

mod message;
mod notify;
mod utils;

pub mod error;
pub use message::Message;
pub use notify::Notify;
