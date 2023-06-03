mod error;
mod utils;

use std::{collections::HashMap, env, fmt::Display, fs};

use error::{ConfigError, ServerError, UrlError};
use qrcode::{render::unicode, QrCode};
use reqwest::{blocking::Client, header};
use serde_json::{self, json};
use url::Url;
use utils::parse_url;

const DEFAULT_API_SERVER: &str = "https://notify.run/api/";
const REGISTER_PATH: &str = "register_channel";
const CHANNEL_PATH: &str = "/c/";
const CHANNEL_KEY: &str = "channelId";

const API_ENV_VAR: &str = "NOTIFY_API_SERVER";
const CONFIG_PATH: &str = "~/.config/notify-run";
const USER_AGENT: &str = "NotifyRun Rust Client";
const ENDPOINT_KEY: &str = "endpoint";
const MESSAGE_KEY: &str = "message";
const ACTION_KEY: &str = "action";

pub struct Notify {
    api_server: Url,
    channel_id: String,
    client: Client,
}

impl Notify {
    pub fn new(api_server: &str, channel_id: &str) -> Result<Notify, UrlError> {
        Ok(Notify {
            api_server: parse_url(api_server)?,
            channel_id: channel_id.to_string(),
            client: Client::new(),
        })
    }

    pub fn from_endpoint(endpoint: &str) -> Result<Notify, UrlError> {
        let parts: Vec<&str> = endpoint.rsplitn(2, '/').collect();
        let channel_id = parts
            .first()
            .ok_or(UrlError::ParseError("Invalid endpoint".to_string()))?;
        let api_server = parts
            .get(1)
            .ok_or(UrlError::ParseError("Invalid endpoint".to_string()))?;

        Notify::new(api_server, channel_id)
    }

    pub fn register() -> Result<Notify, ServerError> {
        Notify::register_from(match env::var(API_ENV_VAR) {
            Ok(server) => server,
            Err(_) => DEFAULT_API_SERVER.to_string(),
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
            .send()
            .map_err(ServerError::Connection)?;

        let code = response.status();
        let text = response
            .text()
            .map_err(|err| ServerError::Response(code, err))?;
        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|_| ServerError::Parse("Invalid JSON response".to_string()))?;

        let channel = json[CHANNEL_KEY]
            .as_str()
            .ok_or(ServerError::Parse(format!(
                "Could not find {} key in JSON response",
                CHANNEL_KEY
            )))?;

        Notify::new(api_server.as_str(), channel).map_err(ServerError::Url)
    }

    pub fn is_configured() -> bool {
        matches!(Notify::from_config(), Ok(_))
    }

    pub fn from_config() -> Result<Notify, ConfigError> {
        let json: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(shellexpand::tilde(CONFIG_PATH).as_ref())
                .map_err(|e| ConfigError::Access(format!("{}", e)))?,
        )
        .map_err(|_| ConfigError::Parse("Invalid JSON in config".to_string()))?;

        let endpoint = json[ENDPOINT_KEY]
            .as_str()
            .ok_or(ConfigError::Parse(format!(
                "Could not find {} key in JSON config",
                ENDPOINT_KEY
            )))?;

        Self::from_endpoint(endpoint).map_err(ConfigError::UrlError)
    }

    pub fn update_config(&self) -> Result<Notify, ConfigError> {
        let json = json!({ ENDPOINT_KEY: self.endpoint().as_str() });
        fs::write(
            shellexpand::tilde(CONFIG_PATH).as_ref(),
            serde_json::to_string(&json).expect("JSON config should always be valid"),
        )
        .map_err(|_| ConfigError::Write("Could not write config".to_string()))?;

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

        let response = self
            .client
            .post(self.endpoint())
            .header(header::USER_AGENT, USER_AGENT)
            .header(header::ACCEPT, "*/*")
            .form(&params)
            .body(message.to_string())
            .send()
            .map_err(ServerError::Connection)?;

        let code = response.status();
        response
            .error_for_status()
            .map_err(|e| ServerError::Response(code, e))?;

        Ok(())
    }

    pub fn send(&self, message: &str) -> Result<(), ServerError> {
        self.send_action(message, "")
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
