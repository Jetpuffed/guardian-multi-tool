use std::collections::HashMap;

use super::{bnet_entities::*, Result};

use reqwest::Client;
use serde::{Deserialize, Serialize};

pub const BASE_URL: &str = "https://www.bungie.net";

/// https://bungie-net.github.io/#Destiny2.GetDestinyManifest
pub async fn get_destiny_manifest(c: &Client,) -> Result<BungieResponse<DestinyManifest>> {
    const PATH: &str = "/platform/destiny2/manifest";

    match c.get([BASE_URL, PATH].join("")).send().await {
        Ok(resp) => return Ok(resp.json::<BungieResponse<DestinyManifest>>().await?),
        Err(e) => panic!("{}", e),
    }
}

/// Generic endpoint response
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BungieResponse<T> {
    detailed_error_trace: Option<String>,
    error_code: i32,
    error_status: String,
    message: String,
    message_data: HashMap<String, String>,
    response: T,
    throttle_seconds: i32,
}

impl<T> BungieResponse<T> {
    pub fn detailed_error_trace(&self) -> Option<&String> {
        self.detailed_error_trace.as_ref()
    }

    pub fn error_code(&self) -> i32 {
        self.error_code
    }

    pub fn error_status(&self) -> &str {
        self.error_status.as_ref()
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    pub fn message_data(&self) -> &HashMap<String, String> {
        &self.message_data
    }

    pub fn response(&self) -> &T {
        &self.response
    }

    pub fn throttle_seconds(&self) -> i32 {
        self.throttle_seconds
    }
}
