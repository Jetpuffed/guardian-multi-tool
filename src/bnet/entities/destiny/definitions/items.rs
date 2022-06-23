use serde::{Deserialize, Serialize};

use super::common::DestinyDisplayPropertiesDefinition;

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyDerivedItemCategoryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyDerivedItemCategoryDefinition {
    category_description: Option<String>,
    items: Option<Vec<DestinyDerivedItemDefinition>>,
}

impl DestinyDerivedItemCategoryDefinition {
    pub fn category_description(&self) -> Option<&String> {
        self.category_description.as_ref()
    }

    pub fn items(&self) -> Option<&Vec<DestinyDerivedItemDefinition>> {
        self.items.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyDerivedItemDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyDerivedItemDefinition {
    icon_path: Option<String>,
    item_description: Option<String>,
    item_detail: Option<String>,
    item_hash: Option<u32>,
    item_name: Option<String>,
    vendor_item_index: Option<i32>,
}

impl DestinyDerivedItemDefinition {
    pub fn icon_path(&self) -> Option<&String> {
        self.icon_path.as_ref()
    }

    pub fn item_description(&self) -> Option<&String> {
        self.item_description.as_ref()
    }

    pub fn item_detail(&self) -> Option<&String> {
        self.item_detail.as_ref()
    }

    pub fn item_hash(&self) -> Option<u32> {
        self.item_hash
    }

    pub fn item_name(&self) -> Option<&String> {
        self.item_name.as_ref()
    }

    pub fn vendor_item_index(&self) -> Option<i32> {
        self.vendor_item_index
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyEnergyCapacityEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyEnergyCapacityEntry {
    capacity_value: Option<i32>,
    energy_type: Option<i32>,
    energy_type_hash: Option<u32>,
}

impl DestinyEnergyCapacityEntry {
    pub fn capacity_value(&self) -> Option<i32> {
        self.capacity_value
    }

    pub fn energy_type(&self) -> Option<i32> {
        self.energy_type
    }

    pub fn energy_type_hash(&self) -> Option<u32> {
        self.energy_type_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyEnergyCostEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyEnergyCostEntry {
    energy_cost: Option<i32>,
    energy_type: Option<i32>,
    energy_type_hash: Option<u32>,
}

impl DestinyEnergyCostEntry {
    pub fn energy_cost(&self) -> Option<i32> {
        self.energy_cost
    }

    pub fn energy_type(&self) -> Option<i32> {
        self.energy_type
    }

    pub fn energy_type_hash(&self) -> Option<u32> {
        self.energy_type_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyItemPlugDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemPlugDefinition {
    alternate_plug_style: Option<i32>,
    alternate_ui_plug_label: Option<String>,
    enabled_material_requirement_hash: Option<u32>,
    enabled_rules: Option<Vec<DestinyPlugRuleDefinition>>,
    energy_capacity: Option<DestinyEnergyCapacityEntry>,
    energy_cost: Option<DestinyEnergyCostEntry>,
    insertion_material_requirement_hash: Option<u32>,
    insertion_rules: Option<Vec<DestinyPlugRuleDefinition>>,
    is_dummy_plug: Option<bool>,
    on_action_recreate_self: Option<bool>,
    parent_item_override: Option<DestinyParentItemOverride>,
    plug_availability: Option<i32>,
    plug_category_hash: Option<u32>,
    plug_category_identifier: Option<String>,
    plug_style: Option<i32>,
    preview_item_override_hash: Option<u32>,
    ui_plug_label: Option<String>,
}

impl DestinyItemPlugDefinition {
    pub fn alternate_plug_style(&self) -> Option<i32> {
        self.alternate_plug_style
    }

    pub fn alternate_ui_plug_label(&self) -> Option<&String> {
        self.alternate_ui_plug_label.as_ref()
    }

    pub fn enabled_material_requirement_hash(&self) -> Option<u32> {
        self.enabled_material_requirement_hash
    }

    pub fn enabled_rules(&self) -> Option<&Vec<DestinyPlugRuleDefinition>> {
        self.enabled_rules.as_ref()
    }

    pub fn energy_capacity(&self) -> Option<&DestinyEnergyCapacityEntry> {
        self.energy_capacity.as_ref()
    }

    pub fn energy_cost(&self) -> Option<&DestinyEnergyCostEntry> {
        self.energy_cost.as_ref()
    }

    pub fn insertion_material_requirement_hash(&self) -> Option<u32> {
        self.insertion_material_requirement_hash
    }

    pub fn insertion_rules(&self) -> Option<&Vec<DestinyPlugRuleDefinition>> {
        self.insertion_rules.as_ref()
    }

    pub fn is_dummy_plug(&self) -> Option<bool> {
        self.is_dummy_plug
    }

    pub fn on_action_recreate_self(&self) -> Option<bool> {
        self.on_action_recreate_self
    }

    pub fn parent_item_override(&self) -> Option<&DestinyParentItemOverride> {
        self.parent_item_override.as_ref()
    }

    pub fn plug_availability(&self) -> Option<i32> {
        self.plug_availability
    }

    pub fn plug_category_hash(&self) -> Option<u32> {
        self.plug_category_hash
    }

    pub fn plug_category_identifier(&self) -> Option<&String> {
        self.plug_category_identifier.as_ref()
    }

    pub fn plug_style(&self) -> Option<i32> {
        self.plug_style
    }

    pub fn preview_item_override_hash(&self) -> Option<u32> {
        self.preview_item_override_hash
    }

    pub fn ui_plug_label(&self) -> Option<&String> {
        self.ui_plug_label.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyItemTierTypeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemTierTypeDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    infusion_process: Option<DestinyItemTierTypeInfusionBlock>,
    redacted: Option<bool>,
}

impl DestinyItemTierTypeDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn infusion_process(&self) -> Option<&DestinyItemTierTypeInfusionBlock> {
        self.infusion_process.as_ref()
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyItemTierTypeInfusionBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemTierTypeInfusionBlock {
    base_quality_transfer_ratio: Option<f32>,
    minimum_quality_increment: Option<i32>,
}

impl DestinyItemTierTypeInfusionBlock {
    pub fn base_quality_transfer_ratio(&self) -> Option<f32> {
        self.base_quality_transfer_ratio
    }

    pub fn minimum_quality_increment(&self) -> Option<i32> {
        self.minimum_quality_increment
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyParentItemOverride
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyParentItemOverride {
    additional_equip_requirements_display_strings: Option<Vec<String>>,
    pip_icon: Option<String>,
}

impl DestinyParentItemOverride {
    pub fn additional_equip_requirements_display_strings(&self) -> Option<&Vec<String>> {
        self.additional_equip_requirements_display_strings.as_ref()
    }

    pub fn pip_icon(&self) -> Option<&String> {
        self.pip_icon.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyPlugRuleDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlugRuleDefinition {
    failure_message: Option<String>,
}

impl DestinyPlugRuleDefinition {
    pub fn failure_message(&self) -> Option<&String> {
        self.failure_message.as_ref()
    }
}
