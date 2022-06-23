use serde::{Deserialize, Serialize};

use super::{common::DestinyDisplayPropertiesDefinition, DestinyItemTranslationBlockDefinition};

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Artifacts.DestinyArtifactDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyArtifactDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
    tiers: Option<Vec<DestinyArtifactTierDefinition>>,
    translation_block: Option<DestinyItemTranslationBlockDefinition>,
}

impl DestinyArtifactDefinition {
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

    pub fn tiers(&self) -> Option<&Vec<DestinyArtifactTierDefinition>> {
        self.tiers.as_ref()
    }

    pub fn translation_block(&self) -> Option<&DestinyItemTranslationBlockDefinition> {
        self.translation_block.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Artifacts.DestinyArtifactTierDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyArtifactTierDefinition {
    display_title: Option<String>,
    items: Option<Vec<DestinyArtifactTierItemDefinition>>,
    minimum_unlock_points_used_requirement: Option<i32>,
    progress_requirement_message: Option<String>,
    tier_hash: Option<u32>,
}

impl DestinyArtifactTierDefinition {
    pub fn display_title(&self) -> Option<&String> {
        self.display_title.as_ref()
    }

    pub fn items(&self) -> Option<&Vec<DestinyArtifactTierItemDefinition>> {
        self.items.as_ref()
    }

    pub fn minimum_unlock_points_used_requirement(&self) -> Option<i32> {
        self.minimum_unlock_points_used_requirement
    }

    pub fn progress_requirement_message(&self) -> Option<&String> {
        self.progress_requirement_message.as_ref()
    }

    pub fn tier_hash(&self) -> Option<u32> {
        self.tier_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Artifacts.DestinyArtifactTierItemDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyArtifactTierItemDefinition {
    item_hash: Option<u32>,
}

impl DestinyArtifactTierItemDefinition {
    pub fn item_hash(&self) -> Option<u32> {
        self.item_hash
    }
}
