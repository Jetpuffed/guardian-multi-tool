use serde::{Deserialize, Serialize};

use super::{
    common::DestinyDisplayPropertiesDefinition, DestinyItemSocketEntryPlugItemRandomizedDefinition,
};

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinyInsertPlugActionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyInsertPlugActionDefinition {
    action_execute_seconds: Option<i32>,
    action_type: Option<i32>,
}

impl DestinyInsertPlugActionDefinition {
    pub fn action_execute_seconds(&self) -> Option<i32> {
        self.action_execute_seconds
    }

    pub fn action_type(&self) -> Option<i32> {
        self.action_type
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinyPlugSetDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlugSetDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    is_fake_plug_set: Option<bool>,
    redacted: Option<bool>,
    reusable_plug_items: Option<Vec<DestinyItemSocketEntryPlugItemRandomizedDefinition>>,
}

impl DestinyPlugSetDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn is_fake_plug_set(&self) -> Option<bool> {
        self.is_fake_plug_set
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn reusable_plug_items(
        &self,
    ) -> Option<&Vec<DestinyItemSocketEntryPlugItemRandomizedDefinition>> {
        self.reusable_plug_items.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinyPlugWhitelistEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlugWhitelistEntryDefinition {
    category_hash: Option<u32>,
    category_identifier: Option<String>,
    reinitialization_possible_plug_hashes: Option<Vec<u32>>,
}

impl DestinyPlugWhitelistEntryDefinition {
    pub fn category_hash(&self) -> Option<u32> {
        self.category_hash
    }

    pub fn category_identifier(&self) -> Option<&String> {
        self.category_identifier.as_ref()
    }

    pub fn reinitialization_possible_plug_hashes(&self) -> Option<&Vec<u32>> {
        self.reinitialization_possible_plug_hashes.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinySocketCategoryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySocketCategoryDefinition {
    category_style: Option<i32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
    ui_category_style: Option<u32>,
}

impl DestinySocketCategoryDefinition {
    pub fn category_style(&self) -> Option<i32> {
        self.category_style
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

    pub fn ui_category_style(&self) -> Option<u32> {
        self.ui_category_style
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinySocketTypeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySocketTypeDefinition {
    always_randomize_sockets: Option<bool>,
    avoid_duplicates_on_initialization: Option<bool>,
    currency_scalars: Option<Vec<DestinySocketTypeScalarMaterialRequirementEntry>>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    hide_duplicate_reusable_plugs: Option<bool>,
    index: Option<i32>,
    insert_action: Option<DestinyInsertPlugActionDefinition>,
    is_preview_enabled: Option<bool>,
    overrides_ui_appearance: Option<bool>,
    plug_whitelist: Option<Vec<DestinyPlugWhitelistEntryDefinition>>,
    redacted: Option<bool>,
    socket_category_hash: Option<u32>,
    visibility: Option<i32>,
}

impl DestinySocketTypeDefinition {
    pub fn always_randomize_sockets(&self) -> Option<bool> {
        self.always_randomize_sockets
    }

    pub fn avoid_duplicates_on_initialization(&self) -> Option<bool> {
        self.avoid_duplicates_on_initialization
    }

    pub fn currency_scalars(
        &self,
    ) -> Option<&Vec<DestinySocketTypeScalarMaterialRequirementEntry>> {
        self.currency_scalars.as_ref()
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn hide_duplicate_reusable_plugs(&self) -> Option<bool> {
        self.hide_duplicate_reusable_plugs
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn insert_action(&self) -> Option<&DestinyInsertPlugActionDefinition> {
        self.insert_action.as_ref()
    }

    pub fn is_preview_enabled(&self) -> Option<bool> {
        self.is_preview_enabled
    }

    pub fn overrides_ui_appearance(&self) -> Option<bool> {
        self.overrides_ui_appearance
    }

    pub fn plug_whitelist(&self) -> Option<&Vec<DestinyPlugWhitelistEntryDefinition>> {
        self.plug_whitelist.as_ref()
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn socket_category_hash(&self) -> Option<u32> {
        self.socket_category_hash
    }

    pub fn visibility(&self) -> Option<i32> {
        self.visibility
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinySocketTypeScalarMaterialRequirementEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySocketTypeScalarMaterialRequirementEntry {
    currency_item_hash: Option<u32>,
    scalar_value: Option<i32>,
}

impl DestinySocketTypeScalarMaterialRequirementEntry {
    pub fn currency_item_hash(&self) -> Option<u32> {
        self.currency_item_hash
    }

    pub fn scalar_value(&self) -> Option<i32> {
        self.scalar_value
    }
}
