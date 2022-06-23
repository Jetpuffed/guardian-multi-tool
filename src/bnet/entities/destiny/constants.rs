use serde::{Deserialize, Serialize};

/// https://bungie-net.github.io/#/components/schemas/Destiny.Constants.DestinyEnvironmentLocationMapping
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyEnvironmentLocationMapping {
    activation_source: Option<String>,
    activity_hash: Option<u32>,
    item_hash: Option<u32>,
    location_hash: Option<u32>,
    objective_hash: Option<u32>,
}

impl DestinyEnvironmentLocationMapping {
    pub fn activation_source(&self) -> Option<&String> {
        self.activation_source.as_ref()
    }

    pub fn activity_hash(&self) -> Option<u32> {
        self.activity_hash
    }

    pub fn item_hash(&self) -> Option<u32> {
        self.item_hash
    }

    pub fn location_hash(&self) -> Option<u32> {
        self.location_hash
    }

    pub fn objective_hash(&self) -> Option<u32> {
        self.objective_hash
    }
}
