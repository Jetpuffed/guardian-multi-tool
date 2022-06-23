use serde::{Deserialize, Serialize};

use super::common::DestinyDisplayPropertiesDefinition;

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.BreakerTypes.DestinyBreakerTypeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyBreakerTypeDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    enum_value: Option<i32>,
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
}

impl DestinyBreakerTypeDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn enum_value(&self) -> Option<i32> {
        self.enum_value
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
}
