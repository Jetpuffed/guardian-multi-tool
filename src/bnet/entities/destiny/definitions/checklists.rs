use serde::{Deserialize, Serialize};

use super::common::DestinyDisplayPropertiesDefinition;

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Checklists.DestinyChecklistDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyChecklistDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    entries: Option<Vec<DestinyChecklistEntryDefinition>>,
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
    scope: Option<i32>,
    view_action_string: Option<String>,
}

impl DestinyChecklistDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn entries(&self) -> Option<&Vec<DestinyChecklistEntryDefinition>> {
        self.entries.as_ref()
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

    pub fn scope(&self) -> Option<i32> {
        self.scope
    }

    pub fn view_action_string(&self) -> Option<&String> {
        self.view_action_string.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Checklists.DestinyChecklistEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyChecklistEntryDefinition {
    activity_hash: Option<u32>,
    bubble_hash: Option<u32>,
    destination_hash: Option<u32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    item_hash: Option<u32>,
    location_hash: Option<u32>,
    scope: Option<i32>,
    vendor_hash: Option<u32>,
    vendor_interaction_index: Option<i32>,
}

impl DestinyChecklistEntryDefinition {
    pub fn activity_hash(&self) -> Option<u32> {
        self.activity_hash
    }

    pub fn bubble_hash(&self) -> Option<u32> {
        self.bubble_hash
    }

    pub fn destination_hash(&self) -> Option<u32> {
        self.destination_hash
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn item_hash(&self) -> Option<u32> {
        self.item_hash
    }

    pub fn location_hash(&self) -> Option<u32> {
        self.location_hash
    }

    pub fn scope(&self) -> Option<i32> {
        self.scope
    }

    pub fn vendor_hash(&self) -> Option<u32> {
        self.vendor_hash
    }

    pub fn vendor_interaction_index(&self) -> Option<i32> {
        self.vendor_interaction_index
    }
}
