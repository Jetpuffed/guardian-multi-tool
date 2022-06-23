use serde::{Deserialize, Serialize};

use super::common::DestinyDisplayPropertiesDefinition;

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Lore.DestinyLoreDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyLoreDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
    subtitle: Option<String>,
}

impl DestinyLoreDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn subtitle(&self) -> Option<&String> {
        self.subtitle.as_ref()
    }
}
