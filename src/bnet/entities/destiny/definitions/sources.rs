use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::DestinyInventoryItemStatDefinition;

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sources.DestinyItemSourceDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSourceDefinition {
    computed_stats: Option<HashMap<u32, DestinyInventoryItemStatDefinition>>,
    level: Option<i32>,
    max_level_required: Option<i32>,
    max_quality: Option<i32>,
    min_level_required: Option<i32>,
    min_quality: Option<i32>,
    source_hashes: Option<Vec<u32>>,
}

impl DestinyItemSourceDefinition {
    pub fn computed_stats(&self) -> Option<&HashMap<u32, DestinyInventoryItemStatDefinition>> {
        self.computed_stats.as_ref()
    }

    pub fn level(&self) -> Option<i32> {
        self.level
    }

    pub fn max_level_required(&self) -> Option<i32> {
        self.max_level_required
    }

    pub fn max_quality(&self) -> Option<i32> {
        self.max_quality
    }

    pub fn min_level_required(&self) -> Option<i32> {
        self.min_level_required
    }

    pub fn min_quality(&self) -> Option<i32> {
        self.min_quality
    }

    pub fn source_hashes(&self) -> Option<&Vec<u32>> {
        self.source_hashes.as_ref()
    }
}
