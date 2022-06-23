use serde::{Deserialize, Serialize};

use super::common::DestinyDisplayPropertiesDefinition;

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationChildBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationChildBlock {
    display_style: Option<i32>,
    parent_presentation_node_hashes: Option<Vec<u32>>,
    presentation_node_type: Option<i32>,
}

impl DestinyPresentationChildBlock {
    pub fn display_style(&self) -> Option<i32> {
        self.display_style
    }

    pub fn parent_presentation_node_hashes(&self) -> Option<&Vec<u32>> {
        self.parent_presentation_node_hashes.as_ref()
    }

    pub fn presentation_node_type(&self) -> Option<i32> {
        self.presentation_node_type
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeChildrenBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationNodeChildrenBlock {
    collectibles: Option<Vec<DestinyPresentationNodeCollectibleChildEntry>>,
    craftables: Option<Vec<DestinyPresentationNodeCraftableChildEntry>>,
    metrics: Option<Vec<DestinyPresentationNodeMetricChildEntry>>,
    presentation_nodes: Option<Vec<DestinyPresentationNodeChildEntry>>,
    records: Option<Vec<DestinyPresentationNodeRecordChildEntry>>,
}

impl DestinyPresentationNodeChildrenBlock {
    pub fn collectibles(&self) -> Option<&Vec<DestinyPresentationNodeCollectibleChildEntry>> {
        self.collectibles.as_ref()
    }

    pub fn craftables(&self) -> Option<&Vec<DestinyPresentationNodeCraftableChildEntry>> {
        self.craftables.as_ref()
    }

    pub fn metrics(&self) -> Option<&Vec<DestinyPresentationNodeMetricChildEntry>> {
        self.metrics.as_ref()
    }

    pub fn presentation_nodes(&self) -> Option<&Vec<DestinyPresentationNodeChildEntry>> {
        self.presentation_nodes.as_ref()
    }

    pub fn records(&self) -> Option<&Vec<DestinyPresentationNodeRecordChildEntry>> {
        self.records.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeChildEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationNodeChildEntry {
    node_display_priority: Option<u32>,
    presentation_node_hash: Option<u32>,
}

impl DestinyPresentationNodeChildEntry {
    pub fn node_display_priority(&self) -> Option<u32> {
        self.node_display_priority
    }

    pub fn presentation_node_hash(&self) -> Option<u32> {
        self.presentation_node_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeCollectibleChildEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationNodeCollectibleChildEntry {
    collectible_hash: Option<u32>,
    node_display_priority: Option<u32>,
}

impl DestinyPresentationNodeCollectibleChildEntry {
    pub fn collectible_hash(&self) -> Option<u32> {
        self.collectible_hash
    }

    pub fn node_display_priority(&self) -> Option<u32> {
        self.node_display_priority
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeCraftableChildEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationNodeCraftableChildEntry {
    craftable_item_hash: Option<u32>,
    node_display_priority: Option<u32>,
}

impl DestinyPresentationNodeCraftableChildEntry {
    pub fn craftable_item_hash(&self) -> Option<u32> {
        self.craftable_item_hash
    }

    pub fn node_display_priority(&self) -> Option<u32> {
        self.node_display_priority
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeMetricChildEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationNodeMetricChildEntry {
    metric_hash: Option<u32>,
    node_display_priority: Option<u32>,
}

impl DestinyPresentationNodeMetricChildEntry {
    pub fn metric_hash(&self) -> Option<u32> {
        self.metric_hash
    }

    pub fn node_display_priority(&self) -> Option<u32> {
        self.node_display_priority
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeRecordChildEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationNodeRecordChildEntry {
    node_display_priority: Option<u32>,
    record_hash: Option<u32>,
}

impl DestinyPresentationNodeRecordChildEntry {
    pub fn node_display_priority(&self) -> Option<u32> {
        self.node_display_priority
    }

    pub fn record_hash(&self) -> Option<u32> {
        self.record_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationNodeDefinition {
    children: Option<DestinyPresentationNodeChildrenBlock>,
    completion_record_hash: Option<u32>,
    disable_child_subscreen_navigation: Option<bool>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    display_style: Option<i32>,
    hash: Option<u32>,
    index: Option<i32>,
    max_category_record_score: Option<i32>,
    node_type: Option<i32>,
    objective_hash: Option<u32>,
    original_icon: Option<String>,
    parent_node_hashes: Option<Vec<u32>>,
    presentation_node_type: Option<i32>,
    redacted: Option<bool>,
    requirements: Option<DestinyPresentationNodeRequirementsBlock>,
    root_view_icon: Option<String>,
    scope: Option<i32>,
    screen_style: Option<i32>,
    trait_hashes: Option<Vec<u32>>,
    trait_ids: Option<Vec<String>>,
}

impl DestinyPresentationNodeDefinition {
    pub fn children(&self) -> Option<&DestinyPresentationNodeChildrenBlock> {
        self.children.as_ref()
    }

    pub fn completion_record_hash(&self) -> Option<u32> {
        self.completion_record_hash
    }

    pub fn disable_child_subscreen_navigation(&self) -> Option<bool> {
        self.disable_child_subscreen_navigation
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn display_style(&self) -> Option<i32> {
        self.display_style
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn max_category_record_score(&self) -> Option<i32> {
        self.max_category_record_score
    }

    pub fn node_type(&self) -> Option<i32> {
        self.node_type
    }

    pub fn objective_hash(&self) -> Option<u32> {
        self.objective_hash
    }

    pub fn original_icon(&self) -> Option<&String> {
        self.original_icon.as_ref()
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

    pub fn requirements(&self) -> Option<&DestinyPresentationNodeRequirementsBlock> {
        self.requirements.as_ref()
    }

    pub fn root_view_icon(&self) -> Option<&String> {
        self.root_view_icon.as_ref()
    }

    pub fn scope(&self) -> Option<i32> {
        self.scope
    }

    pub fn screen_style(&self) -> Option<i32> {
        self.screen_style
    }

    pub fn trait_hashes(&self) -> Option<&Vec<u32>> {
        self.trait_hashes.as_ref()
    }

    pub fn trait_ids(&self) -> Option<&Vec<String>> {
        self.trait_ids.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeRequirementsBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationNodeRequirementsBlock {
    entitlement_unavailable_message: Option<String>,
}

impl DestinyPresentationNodeRequirementsBlock {
    pub fn entitlement_unavailable_message(&self) -> Option<&String> {
        self.entitlement_unavailable_message.as_ref()
    }
}
