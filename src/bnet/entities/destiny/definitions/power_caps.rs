use serde::{Deserialize, Serialize};

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.PowerCaps.DestinyPowerCapDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPowerCapDefinition {
    hash: Option<u32>,
    index: Option<i32>,
    power_cap: Option<i32>,
    redacted: Option<bool>,
}

impl DestinyPowerCapDefinition {
    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn power_cap(&self) -> Option<i32> {
        self.power_cap
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}
