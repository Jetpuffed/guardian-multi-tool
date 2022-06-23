use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::bnet::entities::destiny::DestinyItemQuantity;

use super::common::DestinyDisplayPropertiesDefinition;

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneActivityDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneActivityDefinition {
    conceptual_activity_hash: Option<u32>,
    variants: Option<HashMap<u32, DestinyMilestoneActivityVariantDefinition>>,
}

impl DestinyMilestoneActivityDefinition {
    pub fn conceptual_activity_hash(&self) -> Option<u32> {
        self.conceptual_activity_hash
    }

    pub fn variants(&self) -> Option<&HashMap<u32, DestinyMilestoneActivityVariantDefinition>> {
        self.variants.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneActivityVariantDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneActivityVariantDefinition {
    activity_hash: Option<u32>,
    order: Option<i32>,
}

impl DestinyMilestoneActivityVariantDefinition {
    pub fn activity_hash(&self) -> Option<u32> {
        self.activity_hash
    }

    pub fn order(&self) -> Option<i32> {
        self.order
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneChallengeActivityDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneChallengeActivityDefinition {
    activity_graph_nodes: Option<Vec<DestinyMilestoneChallengeActivityGraphNodeEntry>>,
    activity_hash: Option<u32>,
    challenges: Option<Vec<DestinyMilestoneChallengeDefinition>>,
    phases: Option<Vec<DestinyMilestoneChallengeActivityPhase>>,
}

impl DestinyMilestoneChallengeActivityDefinition {
    pub fn activity_graph_nodes(&self) -> Option<&Vec<DestinyMilestoneChallengeActivityGraphNodeEntry>> {
        self.activity_graph_nodes.as_ref()
    }

    pub fn activity_hash(&self) -> Option<u32> {
        self.activity_hash
    }

    pub fn challenges(&self) -> Option<&Vec<DestinyMilestoneChallengeDefinition>> {
        self.challenges.as_ref()
    }

    pub fn phases(&self) -> Option<&Vec<DestinyMilestoneChallengeActivityPhase>> {
        self.phases.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneChallengeActivityGraphNodeEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneChallengeActivityGraphNodeEntry {
    activity_graph_hash: Option<u32>,
    activity_graph_node_hash: Option<u32>,
}

impl DestinyMilestoneChallengeActivityGraphNodeEntry {
    pub fn activity_graph_hash(&self) -> Option<u32> {
        self.activity_graph_hash
    }

    pub fn activity_graph_node_hash(&self) -> Option<u32> {
        self.activity_graph_node_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneChallengeActivityPhase
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneChallengeActivityPhase {
    phase_hash: Option<u32>,
}

impl DestinyMilestoneChallengeActivityPhase {
    pub fn phase_hash(&self) -> Option<u32> {
        self.phase_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneChallengeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneChallengeDefinition {
    challenge_objective_hash: Option<u32>,
}

impl DestinyMilestoneChallengeDefinition {
    pub fn challenge_objective_hash(&self) -> Option<u32> {
        self.challenge_objective_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneDefinition {
    activities: Option<Vec<DestinyMilestoneChallengeActivityDefinition>>,
    default_order: Option<i32>,
    display_preference: Option<i32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    explore_prioritizes_activity_image: Option<bool>,
    friendly_name: Option<String>,
    has_predictable_dates: Option<bool>,
    hash: Option<u32>,
    image: Option<String>,
    index: Option<i32>,
    is_in_game_milestone: Option<bool>,
    milestone_type: Option<i32>,
    quests: Option<HashMap<u32, DestinyMilestoneQuestDefinition>>,
    recruitable: Option<bool>,
    redacted: Option<bool>,
    rewards: Option<HashMap<u32, DestinyMilestoneRewardCategoryDefinition>>,
    show_in_explorer: Option<bool>,
    show_in_milestones: Option<bool>,
    values: Option<HashMap<String, DestinyMilestoneValueDefinition>>,
    vendors: Option<Vec<DestinyMilestoneVendorDefinition>>,
    vendors_display_title: Option<String>,
}

impl DestinyMilestoneDefinition {
    pub fn activities(&self) -> Option<&Vec<DestinyMilestoneChallengeActivityDefinition>> {
        self.activities.as_ref()
    }

    pub fn default_order(&self) -> Option<i32> {
        self.default_order
    }

    pub fn display_preference(&self) -> Option<i32> {
        self.display_preference
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn explore_prioritizes_activity_image(&self) -> Option<bool> {
        self.explore_prioritizes_activity_image
    }

    pub fn friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    pub fn has_predictable_dates(&self) -> Option<bool> {
        self.has_predictable_dates
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn image(&self) -> Option<&String> {
        self.image.as_ref()
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn is_in_game_milestone(&self) -> Option<bool> {
        self.is_in_game_milestone
    }

    pub fn milestone_type(&self) -> Option<i32> {
        self.milestone_type
    }

    pub fn quests(&self) -> Option<&HashMap<u32, DestinyMilestoneQuestDefinition>> {
        self.quests.as_ref()
    }

    pub fn recruitable(&self) -> Option<bool> {
        self.recruitable
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn rewards(&self) -> Option<&HashMap<u32, DestinyMilestoneRewardCategoryDefinition>> {
        self.rewards.as_ref()
    }

    pub fn show_in_explorer(&self) -> Option<bool> {
        self.show_in_explorer
    }

    pub fn show_in_milestones(&self) -> Option<bool> {
        self.show_in_milestones
    }

    pub fn values(&self) -> Option<&HashMap<String, DestinyMilestoneValueDefinition>> {
        self.values.as_ref()
    }

    pub fn vendors(&self) -> Option<&Vec<DestinyMilestoneVendorDefinition>> {
        self.vendors.as_ref()
    }

    pub fn vendors_display_title(&self) -> Option<&String> {
        self.vendors_display_title.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneQuestDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneQuestDefinition {
    activities: Option<HashMap<u32, DestinyMilestoneActivityDefinition>>,
    destination_hash: Option<u32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    override_image: Option<String>,
    quest_item_hash: Option<u32>,
    quest_rewards: Option<DestinyMilestoneQuestRewardsDefinition>,
}

impl DestinyMilestoneQuestDefinition {
    pub fn activities(&self) -> Option<&HashMap<u32, DestinyMilestoneActivityDefinition>> {
        self.activities.as_ref()
    }

    pub fn destination_hash(&self) -> Option<u32> {
        self.destination_hash
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn override_image(&self) -> Option<&String> {
        self.override_image.as_ref()
    }

    pub fn quest_item_hash(&self) -> Option<u32> {
        self.quest_item_hash
    }

    pub fn quest_rewards(&self) -> Option<&DestinyMilestoneQuestRewardsDefinition> {
        self.quest_rewards.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneQuestRewardsDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneQuestRewardsDefinition {
    items: Option<Vec<DestinyMilestoneQuestRewardItem>>,
}

impl DestinyMilestoneQuestRewardsDefinition {
    pub fn items(&self) -> Option<&Vec<DestinyMilestoneQuestRewardItem>> {
        self.items.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneQuestRewardItem
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneQuestRewardItem {
    has_conditional_visibility: Option<bool>,
    item_hash: Option<u32>,
    item_instance_id: Option<i64>,
    quantity: Option<i32>,
    vendor_hash: Option<u32>,
    vendor_item_index: Option<i32>,
}

impl DestinyMilestoneQuestRewardItem {
    pub fn has_conditional_visibility(&self) -> Option<bool> {
        self.has_conditional_visibility
    }

    pub fn item_hash(&self) -> Option<u32> {
        self.item_hash
    }

    pub fn item_instance_id(&self) -> Option<i64> {
        self.item_instance_id
    }

    pub fn quantity(&self) -> Option<i32> {
        self.quantity
    }

    pub fn vendor_hash(&self) -> Option<u32> {
        self.vendor_hash
    }

    pub fn vendor_item_index(&self) -> Option<i32> {
        self.vendor_item_index
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneRewardCategoryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneRewardCategoryDefinition {
    category_hash: Option<u32>,
    category_identifier: Option<String>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    order: Option<i32>,
    reward_entries: Option<HashMap<u32, DestinyMilestoneRewardEntryDefinition>>,
}

impl DestinyMilestoneRewardCategoryDefinition {
    pub fn category_hash(&self) -> Option<u32> {
        self.category_hash
    }

    pub fn category_identifier(&self) -> Option<&String> {
        self.category_identifier.as_ref()
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn order(&self) -> Option<i32> {
        self.order
    }

    pub fn reward_entries(&self) -> Option<&HashMap<u32, DestinyMilestoneRewardEntryDefinition>> {
        self.reward_entries.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneRewardEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneRewardEntryDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    items: Option<Vec<DestinyItemQuantity>>,
    order: Option<i32>,
    reward_entry_hash: Option<u32>,
    reward_entry_identifier: Option<String>,
    vendor_hash: Option<u32>,
}

impl DestinyMilestoneRewardEntryDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn items(&self) -> Option<&Vec<DestinyItemQuantity>> {
        self.items.as_ref()
    }

    pub fn order(&self) -> Option<i32> {
        self.order
    }

    pub fn reward_entry_hash(&self) -> Option<u32> {
        self.reward_entry_hash
    }

    pub fn reward_entry_identifier(&self) -> Option<&String> {
        self.reward_entry_identifier.as_ref()
    }

    pub fn vendor_hash(&self) -> Option<u32> {
        self.vendor_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneValueDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneValueDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    key: Option<String>,
}

impl DestinyMilestoneValueDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn key(&self) -> Option<&String> {
        self.key.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneVendorDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneVendorDefinition {
    vendor_hash: Option<u32>,
}

impl DestinyMilestoneVendorDefinition {
    pub fn vendor_hash(&self) -> Option<u32> {
        self.vendor_hash
    }
}
