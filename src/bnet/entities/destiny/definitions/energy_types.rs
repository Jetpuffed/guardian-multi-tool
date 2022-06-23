use serde::{Deserialize, Serialize};

use super::common::DestinyDisplayPropertiesDefinition;

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.EnergyTypes.DestinyEnergyTypeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyEnergyTypeDefinition {
    capacity_stat_hash: Option<u32>,
    cost_stat_hash: Option<u32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    enum_value: Option<i32>,
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
    show_icon: Option<bool>,
    transparent_icon_path: Option<String>,
}

impl DestinyEnergyTypeDefinition {
    pub fn capacity_stat_hash(&self) -> Option<u32> {
        self.capacity_stat_hash
    }

    pub fn cost_stat_hash(&self) -> Option<u32> {
        self.cost_stat_hash
    }

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

    pub fn show_icon(&self) -> Option<bool> {
        self.show_icon
    }

    pub fn transparent_icon_path(&self) -> Option<&String> {
        self.transparent_icon_path.as_ref()
    }
}
