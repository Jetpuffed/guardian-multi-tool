use serde::{Deserialize, Serialize};

use super::common::DestinyDisplayPropertiesDefinition;

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.ActivityModifiers.DestinyActivityModifierDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityModifierDefinition {
    display_in_activity_selection: Option<bool>,
    display_in_nav_mode: Option<bool>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
}

impl DestinyActivityModifierDefinition {
    pub fn display_in_activity_selection(&self) -> Option<bool> {
        self.display_in_activity_selection
    }

    pub fn display_in_nav_mode(&self) -> Option<bool> {
        self.display_in_nav_mode
    }

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
}
