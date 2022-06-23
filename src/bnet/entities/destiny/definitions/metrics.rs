use serde::{Deserialize, Serialize};

use super::common::DestinyDisplayPropertiesDefinition;

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Metrics.DestinyMetricDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMetricDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    lower_value_is_better: Option<bool>,
    parent_node_hashes: Option<Vec<u32>>,
    presentation_node_type: Option<i32>,
    redacted: Option<bool>,
    tracking_objective_hash: Option<u32>,
    trait_hashes: Option<Vec<u32>>,
    trait_ids: Option<Vec<String>>,
}

impl DestinyMetricDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn lower_value_is_better(&self) -> Option<bool> {
        self.lower_value_is_better
    }

    pub fn parent_node_hashes(&self) -> Option<&Vec<u32>> {
        self.parent_node_hashes.as_ref()
    }

    pub fn presentation_node_type(&self) -> Option<i32> {
        self.presentation_node_type
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn tracking_objective_hash(&self) -> Option<u32> {
        self.tracking_objective_hash
    }

    pub fn trait_hashes(&self) -> Option<&Vec<u32>> {
        self.trait_hashes.as_ref()
    }

    pub fn trait_ids(&self) -> Option<&Vec<String>> {
        self.trait_ids.as_ref()
    }
}
