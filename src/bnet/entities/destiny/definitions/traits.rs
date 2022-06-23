use serde::{Deserialize, Serialize};

use super::common::DestinyDisplayPropertiesDefinition;

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Traits.DestinyTraitDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyTraitDefinition {
    display_hint: Option<String>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
    trait_category_hash: Option<u32>,
    trait_category_id: Option<String>,
}

impl DestinyTraitDefinition {
    pub fn display_hint(&self) -> Option<&String> {
        self.display_hint.as_ref()
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

    pub fn trait_category_hash(&self) -> Option<u32> {
        self.trait_category_hash
    }

    pub fn trait_category_id(&self) -> Option<&String> {
        self.trait_category_id.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Traits.DestinyTraitCategoryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyTraitCategoryDefinition {
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
    trait_category_id: Option<String>,
    trait_hashes: Option<Vec<u32>>,
    trait_ids: Option<Vec<String>>,
}

impl DestinyTraitCategoryDefinition {
    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn trait_category_id(&self) -> Option<&String> {
        self.trait_category_id.as_ref()
    }

    pub fn trait_hashes(&self) -> Option<&Vec<u32>> {
        self.trait_hashes.as_ref()
    }

    pub fn trait_ids(&self) -> Option<&Vec<String>> {
        self.trait_ids.as_ref()
    }
}
