use serde::{Deserialize, Serialize};

use super::{
    common::DestinyDisplayPropertiesDefinition,
    presentation::{DestinyPresentationChildBlock, DestinyPresentationNodeRequirementsBlock},
};

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Collectibles.DestinyCollectibleAcquisitionBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyCollectibleAcquisitionBlock {
    acquire_material_requirement_hash: Option<u32>,
    acquire_timestamp_unlock_value_hash: Option<u32>,
}

impl DestinyCollectibleAcquisitionBlock {
    pub fn acquire_material_requirement_hash(&self) -> Option<u32> {
        self.acquire_material_requirement_hash
    }

    pub fn acquire_timestamp_unlock_value_hash(&self) -> Option<u32> {
        self.acquire_timestamp_unlock_value_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Collectibles.DestinyCollectibleDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyCollectibleDefinition {
    acquisition_info: Option<DestinyCollectibleAcquisitionBlock>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    item_hash: Option<u32>,
    parent_node_hashes: Option<Vec<u32>>,
    presentation_info: Option<DestinyPresentationChildBlock>,
    presentation_node_type: Option<i32>,
    redacted: Option<bool>,
    scope: Option<i32>,
    source_hash: Option<u32>,
    source_string: Option<String>,
    state_info: Option<DestinyCollectibleStateBlock>,
    trait_hashes: Option<Vec<u32>>,
    trait_ids: Option<Vec<String>>,
}

impl DestinyCollectibleDefinition {
    pub fn acquisition_info(&self) -> Option<&DestinyCollectibleAcquisitionBlock> {
        self.acquisition_info.as_ref()
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

    pub fn item_hash(&self) -> Option<u32> {
        self.item_hash
    }

    pub fn parent_node_hashes(&self) -> Option<&Vec<u32>> {
        self.parent_node_hashes.as_ref()
    }

    pub fn presentation_info(&self) -> Option<&DestinyPresentationChildBlock> {
        self.presentation_info.as_ref()
    }

    pub fn presentation_node_type(&self) -> Option<i32> {
        self.presentation_node_type
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn scope(&self) -> Option<i32> {
        self.scope
    }

    pub fn source_hash(&self) -> Option<u32> {
        self.source_hash
    }

    pub fn source_string(&self) -> Option<&String> {
        self.source_string.as_ref()
    }

    pub fn state_info(&self) -> Option<&DestinyCollectibleStateBlock> {
        self.state_info.as_ref()
    }

    pub fn trait_hashes(&self) -> Option<&Vec<u32>> {
        self.trait_hashes.as_ref()
    }

    pub fn trait_ids(&self) -> Option<&Vec<String>> {
        self.trait_ids.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Collectibles.DestinyCollectibleStateBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyCollectibleStateBlock {
    obscured_override_item_hash: Option<u32>,
    requirements: Option<DestinyPresentationNodeRequirementsBlock>,
}

impl DestinyCollectibleStateBlock {
    pub fn obscured_override_item_hash(&self) -> Option<u32> {
        self.obscured_override_item_hash
    }

    pub fn requirements(&self) -> Option<&DestinyPresentationNodeRequirementsBlock> {
        self.requirements.as_ref()
    }
}
