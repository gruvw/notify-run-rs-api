mod error;
mod utils;

use std::fmt::Display;

use error::{ConfigError, ServerError};
use qrcode::{render::unicode, QrCode};
use reqwest::{blocking::Client, header};
use serde::Deserialize;
use url::Url;
use utils::parse_url;

const DEFAULT_API_SERVER: &str = "https://notify.run/api/";
const REGISTER_PATH: &str = "/register_channel/";
const CHANNEL_PATH: &str = "/c/";

pub struct Notify {
    api_server: Url,
    channel_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterResponse {
    channel_id: String,
}

impl Notify {
    pub fn new(api_server: &str, channel_id: &str) -> Result<Notify, ServerError> {
        Ok(Notify {
            api_server: parse_url(api_server).map_err(ServerError::Url)?,
            channel_id: channel_id.to_string(),
        })
    }

    pub fn load_config() -> Result<Notify, ConfigError<'static>> {
        // TODO endpoint from config
        Notify::new("", "").map_err(ConfigError::ServerError)
    }

    pub fn register() -> Result<Notify, ServerError> {
        Notify::register_from(DEFAULT_API_SERVER)
    }

    pub fn register_from(api_server: &str) -> Result<Notify, ServerError> {
        let api_server = parse_url(api_server).map_err(ServerError::Url)?;
        let client = Client::new();
        let response = client
            .post(api_server.join(REGISTER_PATH).unwrap())
            .header(header::USER_AGENT, "")
            .header(header::CONTENT_LENGTH, 0)
            .send()
            .map_err(ServerError::Connection)?;
        let data = response
            .json::<RegisterResponse>()
            .map_err(ServerError::Response)?;
        Notify::new(api_server.as_str(), &data.channel_id)
    }

    pub fn server(&self) -> Url {
        Url::parse(
            format!(
                "{}://{}",
                self.api_server.scheme(),
                self.api_server.host_str().unwrap()
            )
            .as_str(),
        )
        .unwrap()
    }

    pub fn endpoint(&self) -> Url {
        self.server().join(&self.channel_id).unwrap()
    }

    pub fn channel(&self) -> Url {
        self.server()
            .join(CHANNEL_PATH)
            .unwrap()
            .join(&self.channel_id)
            .unwrap()
    }
}

impl Display for Notify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = QrCode::new(self.channel().as_str()).unwrap();
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
