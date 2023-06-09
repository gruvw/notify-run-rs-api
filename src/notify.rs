// TODO //!

use crate::error::{ConfigError, ServerError, UrlError};
use crate::message::Message;
use crate::utils::{decode_msg, parse_url};
use qrcode::{render::unicode, QrCode};
use reqwest::{blocking::Client, header};
use serde_json::{self, json};
use std::{collections::HashMap, env, fmt::Display, fs};
use url::Url;

const DEFAULT_API_SERVER: &str = "https://notify.run/api/";
const REGISTER_PATH: &str = "register_channel";
const INFO_PATH: &str = "json";
const CHANNEL_PATH: &str = "/c/";

pub(crate) const MESSAGE_KEY: &str = "message";
pub(crate) const TIME_KEY: &str = "time";
const MESSAGES_KEY: &str = "messages";
const ENDPOINT_KEY: &str = "endpoint";
const ACTION_KEY: &str = "action";
const CHANNEL_KEY: &str = "channelId";
const API_ENV_VAR: &str = "NOTIFY_API_SERVER";
const CONFIG_PATH: &str = "~/.config/notify-run";
const USER_AGENT: &str = "NotifyRun Rust Client";

/// Notification object. Use to access and interact with a notify.run endpoint
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notify {
    api_server: Url,
    channel_id: String,
}

impl Notify {
    pub fn new(channel_id: &str) -> Result<Notify, UrlError> {
        Self::with_server(DEFAULT_API_SERVER, channel_id)
    }

    pub fn with_server(api_server: &str, channel_id: &str) -> Result<Notify, UrlError> {
        Ok(Notify {
            api_server: parse_url(api_server)?,
            channel_id: channel_id.into(),
        })
    }

    pub fn from_endpoint(endpoint: &str) -> Result<Notify, UrlError> {
        let parts: Vec<&str> = endpoint.rsplitn(2, '/').collect();
        let channel_id = parts.first().ok_or(UrlError::ParseError {
            text: endpoint.into(),
        })?;
        let api_server = parts.get(1).ok_or(UrlError::ParseError {
            text: endpoint.into(),
        })?;

        Notify::with_server(api_server, channel_id)
    }

    pub fn register() -> Result<Notify, ServerError> {
        Notify::register_from(match env::var(API_ENV_VAR) {
            Ok(server) => server,
            Err(_) => DEFAULT_API_SERVER.into(),
        })
    }

    pub fn register_from(api_server: String) -> Result<Notify, ServerError> {
        let api_server = parse_url(&api_server).map_err(ServerError::Url)?;
        let url = api_server
            .join(REGISTER_PATH)
            .expect("Registration join should always be valid");

        let client = Client::new();
        let response = client
            .post(url)
            .header(header::USER_AGENT, USER_AGENT)
            .header(header::CONTENT_LENGTH, 0)
            .send()?;

        let text = response.text()?;
        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|_| ServerError::Parse("Invalid JSON response".into()))?;

        let channel = json[CHANNEL_KEY]
            .as_str()
            .ok_or(ServerError::Parse(format!(
                "Could not find {} key in JSON response",
                CHANNEL_KEY
            )))?;

        Notify::with_server(api_server.as_str(), channel).map_err(ServerError::Url)
    }

    pub fn is_configured() -> bool {
        matches!(Notify::from_config(), Ok(_))
    }

    pub fn from_config() -> Result<Notify, ConfigError> {
        let config = fs::read_to_string(shellexpand::tilde(CONFIG_PATH).as_ref())?;
        let json: serde_json::Value = serde_json::from_str(config.as_str())?;

        let endpoint = json[ENDPOINT_KEY]
            .as_str()
            .ok_or(ConfigError::KeyNotFound(ENDPOINT_KEY.into()))?;

        Self::from_endpoint(endpoint).map_err(ConfigError::UrlError)
    }

    pub fn write_to_config(&self) -> Result<Notify, ConfigError> {
        let json = json!({ ENDPOINT_KEY: self.endpoint().as_str() });
        fs::write(
            shellexpand::tilde(CONFIG_PATH).as_ref(),
            serde_json::to_string(&json).expect("JSON config should always be valid"),
        )?;

        Self::from_config()
    }

    pub fn server(&self) -> Url {
        Url::parse(
            format!(
                "{}://{}",
                self.api_server.scheme(),
                self.api_server.host_str().expect("Url should have host")
            )
            .as_str(),
        )
        .expect("Server Url should always parse")
    }

    pub fn endpoint(&self) -> Url {
        self.server()
            .join(&self.channel_id)
            .expect("Channel ID join should always work")
    }

    pub fn channel(&self) -> Url {
        self.api_server
            .join(CHANNEL_PATH)
            .expect("Channel path join should always work")
            .join(&self.channel_id)
            .expect("Channel ID join should always work")
    }

    pub fn send_action(&self, message: &str, action: &str) -> Result<(), ServerError> {
        let mut params = HashMap::new();
        params.insert(MESSAGE_KEY, message);
        if !action.is_empty() {
            params.insert(ACTION_KEY, action);
        }

        let response = Client::new()
            .post(self.endpoint())
            .header(header::USER_AGENT, USER_AGENT)
            .header(header::ACCEPT, "*/*")
            .form(&params)
            .body(message.to_string())
            .send()?;

        response.error_for_status()?;

        Ok(())
    }

    pub fn send(&self, message: &str) -> Result<(), ServerError> {
        self.send_action(message, "")
    }

    pub fn messages(&self) -> Result<Vec<Message>, ServerError> {
        let mut url = self.endpoint();
        url.path_segments_mut().unwrap().push(INFO_PATH);

        let response = Client::new()
            .get(url)
            .header(header::USER_AGENT, USER_AGENT)
            .send()?;

        let text = response.text()?;
        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|_| ServerError::Parse("Invalid JSON response".into()))?;

        json.get(MESSAGES_KEY)
            .ok_or(ServerError::Parse(
                "JSON response does not contains messages".into(),
            ))?
            .as_array()
            .ok_or(ServerError::Parse(
                "JSON response messages type is not an array".into(),
            ))?
            .iter()
            .map(decode_msg)
            .collect::<Result<Vec<_>, _>>()
    }
}

impl Display for Notify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = QrCode::new(self.channel().as_str()).expect("QrCode should always be valid");
        let image = code
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();

        write!(
            f,
            "Endpoint: {}\nTo subscribe, open: {}\nOr scan this QR code:\n{}",
            self.endpoint(),
            self.channel(),
            image
        )
    }
}
