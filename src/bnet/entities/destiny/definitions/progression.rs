use serde::{Deserialize, Serialize};

use crate::bnet::entities::interpolation::InterpolationPointFloat;

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Progression.DestinyProgressionLevelRequirementDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProgressionLevelRequirementDefinition {
    hash: Option<u32>,
    index: Option<i32>,
    progression_hash: Option<u32>,
    redacted: Option<bool>,
    requirement_curve: Option<Vec<InterpolationPointFloat>>,
}

impl DestinyProgressionLevelRequirementDefinition {
    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn progression_hash(&self) -> Option<u32> {
        self.progression_hash
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn requirement_curve(&self) -> Option<&Vec<InterpolationPointFloat>> {
        self.requirement_curve.as_ref()
    }
}
