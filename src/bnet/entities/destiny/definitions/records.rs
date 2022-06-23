use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::bnet::entities::destiny::{DestinyGender, DestinyItemQuantity};

use super::{
    common::DestinyDisplayPropertiesDefinition,
    presentation::{DestinyPresentationChildBlock, DestinyPresentationNodeRequirementsBlock},
};

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.DestinyRecordDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRecordDefinition {
    completion_info: Option<DestinyRecordCompletionBlock>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    expiration_info: Option<DestinyRecordExpirationBlock>,
    for_title_gilding: Option<bool>,
    hash: Option<u32>,
    index: Option<i32>,
    interval_info: Option<DestinyRecordIntervalBlock>,
    lore_hash: Option<u32>,
    objective_hashes: Option<Vec<u32>>,
    parent_node_hashes: Option<Vec<u32>>,
    presentation_info: Option<DestinyPresentationChildBlock>,
    presentation_node_type: Option<i32>,
    record_value_style: Option<i32>,
    redacted: Option<bool>,
    requirements: Option<DestinyPresentationNodeRequirementsBlock>,
    reward_items: Option<Vec<DestinyItemQuantity>>,
    scope: Option<i32>,
    should_show_large_icons: Option<bool>,
    state_info: Option<SchemaRecordStateBlock>,
    title_info: Option<DestinyRecordTitleBlock>,
    trait_hashes: Option<Vec<u32>>,
    trait_ids: Option<Vec<String>>,
}

impl DestinyRecordDefinition {
    pub fn completion_info(&self) -> Option<&DestinyRecordCompletionBlock> {
        self.completion_info.as_ref()
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn expiration_info(&self) -> Option<&DestinyRecordExpirationBlock> {
        self.expiration_info.as_ref()
    }

    pub fn for_title_gilding(&self) -> Option<bool> {
        self.for_title_gilding
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn interval_info(&self) -> Option<&DestinyRecordIntervalBlock> {
        self.interval_info.as_ref()
    }

    pub fn lore_hash(&self) -> Option<u32> {
        self.lore_hash
    }

    pub fn objective_hashes(&self) -> Option<&Vec<u32>> {
        self.objective_hashes.as_ref()
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

    pub fn record_value_style(&self) -> Option<i32> {
        self.record_value_style
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn requirements(&self) -> Option<&DestinyPresentationNodeRequirementsBlock> {
        self.requirements.as_ref()
    }

    pub fn reward_items(&self) -> Option<&Vec<DestinyItemQuantity>> {
        self.reward_items.as_ref()
    }

    pub fn scope(&self) -> Option<i32> {
        self.scope
    }

    pub fn should_show_large_icons(&self) -> Option<bool> {
        self.should_show_large_icons
    }

    pub fn state_info(&self) -> Option<&SchemaRecordStateBlock> {
        self.state_info.as_ref()
    }

    pub fn title_info(&self) -> Option<&DestinyRecordTitleBlock> {
        self.title_info.as_ref()
    }

    pub fn trait_hashes(&self) -> Option<&Vec<u32>> {
        self.trait_hashes.as_ref()
    }

    pub fn trait_ids(&self) -> Option<&Vec<String>> {
        self.trait_ids.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.DestinyRecordCompletionBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRecordCompletionBlock {
    partial_completion_objective_count_threshold: Option<i32>,
    score_value: Option<i32>,
    should_fire_toast: Option<bool>,
    toast_style: Option<i32>,
}

impl DestinyRecordCompletionBlock {
    pub fn partial_completion_objective_count_threshold(&self) -> Option<i32> {
        self.partial_completion_objective_count_threshold
    }

    pub fn score_value(&self) -> Option<i32> {
        self.score_value
    }

    pub fn should_fire_toast(&self) -> Option<bool> {
        self.should_fire_toast
    }

    pub fn toast_style(&self) -> Option<i32> {
        self.toast_style
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.DestinyRecordExpirationBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRecordExpirationBlock {
    description: Option<String>,
    has_expiration: Option<bool>,
    icon: Option<String>,
}

impl DestinyRecordExpirationBlock {
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn has_expiration(&self) -> Option<bool> {
        self.has_expiration
    }

    pub fn icon(&self) -> Option<&String> {
        self.icon.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.DestinyRecordIntervalBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRecordIntervalBlock {
    interval_objectives: Option<Vec<DestinyRecordIntervalObjective>>,
    interval_rewards: Option<Vec<DestinyRecordIntervalRewards>>,
    original_objective_array_insertion_index: Option<i32>,
}

impl DestinyRecordIntervalBlock {
    pub fn interval_objectives(&self) -> Option<&Vec<DestinyRecordIntervalObjective>> {
        self.interval_objectives.as_ref()
    }

    pub fn interval_rewards(&self) -> Option<&Vec<DestinyRecordIntervalRewards>> {
        self.interval_rewards.as_ref()
    }

    pub fn original_objective_array_insertion_index(&self) -> Option<i32> {
        self.original_objective_array_insertion_index
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.DestinyRecordIntervalObjective
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRecordIntervalObjective {
    interval_objective_hash: Option<u32>,
    interval_score_value: Option<i32>,
}

impl DestinyRecordIntervalObjective {
    pub fn interval_objective_hash(&self) -> Option<u32> {
        self.interval_objective_hash
    }

    pub fn interval_score_value(&self) -> Option<i32> {
        self.interval_score_value
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.DestinyRecordIntervalRewards
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRecordIntervalRewards {
    interval_reward_items: Option<Vec<DestinyItemQuantity>>,
}

impl DestinyRecordIntervalRewards {
    pub fn interval_reward_items(&self) -> Option<&Vec<DestinyItemQuantity>> {
        self.interval_reward_items.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.DestinyRecordTitleBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRecordTitleBlock {
    gilding_tracking_record_hash: Option<u32>,
    has_title: Option<bool>,
    titles_by_gender: Option<HashMap<DestinyGender, String>>,
    titles_by_gender_hash: Option<HashMap<u32, String>>,
}

impl DestinyRecordTitleBlock {
    pub fn gilding_tracking_record_hash(&self) -> Option<u32> {
        self.gilding_tracking_record_hash
    }

    pub fn has_title(&self) -> Option<bool> {
        self.has_title
    }

    pub fn titles_by_gender(&self) -> Option<&HashMap<DestinyGender, String>> {
        self.titles_by_gender.as_ref()
    }

    pub fn titles_by_gender_hash(&self) -> Option<&HashMap<u32, String>> {
        self.titles_by_gender_hash.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.SchemaRecordStateBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaRecordStateBlock {
    featured_priority: Option<i32>,
    obscured_string: Option<String>,
}

impl SchemaRecordStateBlock {
    pub fn featured_priority(&self) -> Option<i32> {
        self.featured_priority
    }

    pub fn obscured_string(&self) -> Option<&String> {
        self.obscured_string.as_ref()
    }
}
