use serde::{Deserialize, Serialize};

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyDisplayPropertiesDefinition {
    description: Option<String>,
    has_icon: Option<bool>,
    high_res_icon: Option<String>,
    icon: Option<String>,
    icon_sequences: Option<Vec<DestinyIconSequenceDefinition>>,
    name: Option<String>,
}

impl DestinyDisplayPropertiesDefinition {
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn has_icon(&self) -> Option<bool> {
        self.has_icon
    }

    pub fn high_res_icon(&self) -> Option<&String> {
        self.high_res_icon.as_ref()
    }

    pub fn icon(&self) -> Option<&String> {
        self.icon.as_ref()
    }

    pub fn icon_sequences(&self) -> Option<&Vec<DestinyIconSequenceDefinition>> {
        self.icon_sequences.as_ref()
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Common.DestinyIconSequenceDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyIconSequenceDefinition {
    frames: Option<Vec<String>>,
}

impl DestinyIconSequenceDefinition {
    pub fn frames(&self) -> Option<&Vec<String>> {
        self.frames.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Common.DestinyPositionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPositionDefinition {
    x: Option<i32>,
    y: Option<i32>,
    z: Option<i32>,
}

impl DestinyPositionDefinition {
    pub fn x(&self) -> Option<i32> {
        self.x
    }

    pub fn y(&self) -> Option<i32> {
        self.y
    }

    pub fn z(&self) -> Option<i32> {
        self.z
    }
}
