use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use self::definitions::{
    activity_modifiers::DestinyActivityModifierDefinition,
    artifacts::DestinyArtifactDefinition,
    breaker_types::DestinyBreakerTypeDefinition,
    checklists::DestinyChecklistDefinition,
    collectibles::DestinyCollectibleDefinition,
    director::DestinyActivityGraphDefinition,
    energy_types::DestinyEnergyTypeDefinition,
    items::DestinyItemTierTypeDefinition,
    lore::DestinyLoreDefinition,
    metrics::DestinyMetricDefinition,
    milestones::DestinyMilestoneDefinition,
    power_caps::DestinyPowerCapDefinition,
    presentation::DestinyPresentationNodeDefinition,
    progression::DestinyProgressionLevelRequirementDefinition,
    records::DestinyRecordDefinition,
    reporting::DestinyReportReasonCategoryDefinition,
    seasons::{DestinySeasonDefinition, DestinySeasonPassDefinition},
    sockets::{
        DestinyPlugSetDefinition, DestinySocketCategoryDefinition, DestinySocketTypeDefinition,
    },
    traits::{DestinyTraitCategoryDefinition, DestinyTraitDefinition},
    DestinyActivityDefinition, DestinyActivityModeDefinition, DestinyActivityTypeDefinition,
    DestinyArtDyeReference, DestinyClassDefinition, DestinyDamageTypeDefinition,
    DestinyDestinationDefinition, DestinyEquipmentSlotDefinition, DestinyFactionDefinition,
    DestinyGenderDefinition, DestinyInventoryBucketDefinition, DestinyInventoryItemDefinition,
    DestinyItemCategoryDefinition, DestinyLocationDefinition,
    DestinyMaterialRequirementSetDefinition, DestinyMedalTierDefinition,
    DestinyObjectiveDefinition, DestinyPlaceDefinition, DestinyProgressionDefinition,
    DestinyProgressionMappingDefinition, DestinyRaceDefinition, DestinyRewardSourceDefinition,
    DestinySandboxPatternDefinition, DestinySandboxPerkDefinition, DestinyStatDefinition,
    DestinyStatGroupDefinition, DestinyTalentGridDefinition, DestinyUnlockDefinition,
    DestinyUnlockValueDefinition, DestinyVendorDefinition, DestinyVendorGroupDefinition,
};

pub mod components;
pub mod config;
pub mod constants;
pub mod definitions;
pub mod entities;
pub mod misc;
pub mod quests;
pub mod responses;

/// Where all the deserialized game content lives.
///
/// Returned by any function that deserializes the world content paths obtained
/// from the manifest in Bungie's API.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DestinyWorldContent {
    // destiny_achievement_definition: HashMap<String, DestinyAchievementDefinition>,
    destiny_activity_definition: HashMap<String, DestinyActivityDefinition>,
    destiny_activity_graph_definition: HashMap<String, DestinyActivityGraphDefinition>,
    // destiny_activity_interactable_definition: HashMap<String, DestinyActivityInteractableDefinition>,
    destiny_activity_mode_definition: HashMap<String, DestinyActivityModeDefinition>,
    destiny_activity_modifier_definition: HashMap<String, DestinyActivityModifierDefinition>,
    destiny_activity_type_definition: HashMap<String, DestinyActivityTypeDefinition>,
    // destiny_art_dye_channel_definition: HashMap<String, DestinyArtDyeChannelDefinition>,
    destiny_art_dye_reference_definition: HashMap<String, DestinyArtDyeReference>,
    destiny_artifact_definition: HashMap<String, DestinyArtifactDefinition>,
    // destiny_bond_definition: HashMap<String, DestinyBondDefinition>,
    destiny_breaker_type_definition: HashMap<String, DestinyBreakerTypeDefinition>,
    // destiny_character_customization_category_definition: HashMap<String, DestinyCharacterCustomizationCategoryDefinition>,
    // destiny_character_customization_option_definition: HashMap<String, DestinyCharacterCustomizationOptionDefinition>,
    destiny_checklist_definition: HashMap<String, DestinyChecklistDefinition>,
    destiny_class_definition: HashMap<String, DestinyClassDefinition>,
    destiny_collectible_definition: HashMap<String, DestinyCollectibleDefinition>,
    destiny_damage_type_definition: HashMap<String, DestinyDamageTypeDefinition>,
    destiny_destination_definition: HashMap<String, DestinyDestinationDefinition>,
    destiny_energy_type_definition: HashMap<String, DestinyEnergyTypeDefinition>,
    // destiny_entitlement_offer_definition: HashMap<String, DestinyEntitlementOfferDefinition>,
    destiny_equipment_slot_definition: HashMap<String, DestinyEquipmentSlotDefinition>,
    destiny_faction_definition: HashMap<String, DestinyFactionDefinition>,
    destiny_gender_definition: HashMap<String, DestinyGenderDefinition>,
    destiny_item_category_definition: HashMap<String, DestinyItemCategoryDefinition>,
    destiny_item_tier_type_definition: HashMap<String, DestinyItemTierTypeDefinition>,
    destiny_inventory_bucket_definition: HashMap<String, DestinyInventoryBucketDefinition>,
    destiny_inventory_item_definition: HashMap<String, DestinyInventoryItemDefinition>,
    // destiny_inventory_item_lite_definition: HashMap<String, DestinyInventoryItemLiteDefinition>,
    destiny_location_definition: HashMap<String, DestinyLocationDefinition>,
    destiny_lore_definition: HashMap<String, DestinyLoreDefinition>,
    destiny_material_requirement_set_definition:
        HashMap<String, DestinyMaterialRequirementSetDefinition>,
    destiny_medal_tier_definition: HashMap<String, DestinyMedalTierDefinition>,
    destiny_metric_definition: HashMap<String, DestinyMetricDefinition>,
    destiny_milestone_definition: HashMap<String, DestinyMilestoneDefinition>,
    // destiny_node_step_summary_definition: HashMap<String, DestinyNodeStepSummaryDefinition>,
    destiny_objective_definition: HashMap<String, DestinyObjectiveDefinition>,
    destiny_place_definition: HashMap<String, DestinyPlaceDefinition>,
    // destiny_platform_bucket_mapping_definition: HashMap<String, DestinyPlatformBucketMappingDefinition>,
    destiny_plug_set_definition: HashMap<String, DestinyPlugSetDefinition>,
    destiny_power_cap_definition: HashMap<String, DestinyPowerCapDefinition>,
    destiny_presentation_node_definition: HashMap<String, DestinyPresentationNodeDefinition>,
    destiny_progression_definition: HashMap<String, DestinyProgressionDefinition>,
    destiny_progression_level_requirement_definition:
        HashMap<String, DestinyProgressionLevelRequirementDefinition>,
    destiny_progression_mapping_definition: HashMap<String, DestinyProgressionMappingDefinition>,
    destiny_race_definition: HashMap<String, DestinyRaceDefinition>,
    destiny_record_definition: HashMap<String, DestinyRecordDefinition>,
    destiny_report_reason_category_definition:
        HashMap<String, DestinyReportReasonCategoryDefinition>,
    // destiny_reward_adjuster_pointer_definition: HashMap<String, DestinyRewardAdjusterPointerDefinition>,
    // destiny_reward_adjuster_progression_map_definition: HashMap<String, DestinyRewardAdjusterProgressionMapDefinition>,
    // destiny_reward_item_list_definition: HashMap<String, DestinyRewardItemListDefinition>,
    // destiny_reward_mapping_definition: HashMap<String, DestinyRewardMappingDefinition>,
    // destiny_reward_sheet_definition: HashMap<String, DestinyRewardSheetDefinition>,
    destiny_reward_source_definition: HashMap<String, DestinyRewardSourceDefinition>,
    // destiny_sack_reward_item_list_definition: HashMap<String, DestinySackRewardItemListDefinition>,
    destiny_sandbox_pattern_definition: HashMap<String, DestinySandboxPatternDefinition>,
    destiny_sandbox_perk_definition: HashMap<String, DestinySandboxPerkDefinition>,
    destiny_season_definition: HashMap<String, DestinySeasonDefinition>,
    destiny_season_pass_definition: HashMap<String, DestinySeasonPassDefinition>,
    destiny_socket_category_definition: HashMap<String, DestinySocketCategoryDefinition>,
    destiny_socket_type_definition: HashMap<String, DestinySocketTypeDefinition>,
    destiny_stat_definition: HashMap<String, DestinyStatDefinition>,
    destiny_stat_group_definition: HashMap<String, DestinyStatGroupDefinition>,
    destiny_talent_grid_definition: HashMap<String, DestinyTalentGridDefinition>,
    destiny_trait_definition: HashMap<String, DestinyTraitDefinition>,
    destiny_trait_category_definition: HashMap<String, DestinyTraitCategoryDefinition>,
    // destiny_unlock_count_mapping_definition: HashMap<String, DestinyUnlockCountMappingDefinition>,
    destiny_unlock_definition: HashMap<String, DestinyUnlockDefinition>,
    // destiny_unlock_event_definition: HashMap<String, DestinyUnlockEventDefinition>,
    // destiny_unlock_expression_mapping_definition: HashMap<String, DestinyUnlockExpressionMappingDefinition>,
    destiny_unlock_value_definition: HashMap<String, DestinyUnlockValueDefinition>,
    destiny_vendor_definition: HashMap<String, DestinyVendorDefinition>,
    destiny_vendor_group_definition: HashMap<String, DestinyVendorGroupDefinition>,
}

impl DestinyWorldContent {
    // pub fn destiny_achievement_definition(&self) -> &HashMap<String, DestinyAchievementDefinition> {
    //     &self.destiny_achievement_definition
    // }

    pub fn destiny_activity_definition(&self) -> &HashMap<String, DestinyActivityDefinition> {
        &self.destiny_activity_definition
    }

    pub fn destiny_activity_graph_definition(
        &self,
    ) -> &HashMap<String, DestinyActivityGraphDefinition> {
        &self.destiny_activity_graph_definition
    }

    // pub fn destiny_activity_interactable_definition(&self) -> &HashMap<String, DestinyActivityInteractableDefinition> {
    //     &self.destiny_activity_interactable_definition
    // }

    pub fn destiny_activity_mode_definition(
        &self,
    ) -> &HashMap<String, DestinyActivityModeDefinition> {
        &self.destiny_activity_mode_definition
    }

    pub fn destiny_activity_modifier_definition(
        &self,
    ) -> &HashMap<String, DestinyActivityModifierDefinition> {
        &self.destiny_activity_modifier_definition
    }

    pub fn destiny_activity_type_definition(
        &self,
    ) -> &HashMap<String, DestinyActivityTypeDefinition> {
        &self.destiny_activity_type_definition
    }

    // pub fn destiny_art_dye_channel_definition(&self) -> &HashMap<String, DestinyArtDyeChannelDefinition> {
    //     &self.destiny_art_dye_channel_definition
    // }

    pub fn destiny_art_dye_reference_definition(&self) -> &HashMap<String, DestinyArtDyeReference> {
        &self.destiny_art_dye_reference_definition
    }

    pub fn destiny_artifact_definition(&self) -> &HashMap<String, DestinyArtifactDefinition> {
        &self.destiny_artifact_definition
    }

    // pub fn destiny_bond_definition(&self) -> &HashMap<String, DestinyBondDefinition> {
    //     &self.destiny_bond_definition
    // }

    pub fn destiny_breaker_type_definition(
        &self,
    ) -> &HashMap<String, DestinyBreakerTypeDefinition> {
        &self.destiny_breaker_type_definition
    }

    // pub fn destiny_character_customization_category_definition(&self) -> &HashMap<String, DestinyCharacterCustomizationCategoryDefinition> {
    //     &self.destiny_character_customization_category_definition
    // }

    // pub fn destiny_character_customization_option_definition(&self) -> &HashMap<String, DestinyCharacterCustomizationOptionDefinition> {
    //     &self.destiny_character_customization_option_definition
    // }

    pub fn destiny_checklist_definition(&self) -> &HashMap<String, DestinyChecklistDefinition> {
        &self.destiny_checklist_definition
    }

    pub fn destiny_class_definition(&self) -> &HashMap<String, DestinyClassDefinition> {
        &self.destiny_class_definition
    }

    pub fn destiny_collectible_definition(&self) -> &HashMap<String, DestinyCollectibleDefinition> {
        &self.destiny_collectible_definition
    }

    pub fn destiny_damage_type_definition(&self) -> &HashMap<String, DestinyDamageTypeDefinition> {
        &self.destiny_damage_type_definition
    }

    pub fn destiny_destination_definition(&self) -> &HashMap<String, DestinyDestinationDefinition> {
        &self.destiny_destination_definition
    }

    pub fn destiny_energy_type_definition(&self) -> &HashMap<String, DestinyEnergyTypeDefinition> {
        &self.destiny_energy_type_definition
    }

    // pub fn destiny_entitlement_offer_definition(&self) -> &HashMap<String, DestinyEntitlementOfferDefinition> {
    //     &self.destiny_entitlement_offer_definition
    // }

    pub fn destiny_equipment_slot_definition(
        &self,
    ) -> &HashMap<String, DestinyEquipmentSlotDefinition> {
        &self.destiny_equipment_slot_definition
    }

    pub fn destiny_faction_definition(&self) -> &HashMap<String, DestinyFactionDefinition> {
        &self.destiny_faction_definition
    }

    pub fn destiny_gender_definition(&self) -> &HashMap<String, DestinyGenderDefinition> {
        &self.destiny_gender_definition
    }

    pub fn destiny_item_category_definition(
        &self,
    ) -> &HashMap<String, DestinyItemCategoryDefinition> {
        &self.destiny_item_category_definition
    }

    pub fn destiny_item_tier_type_definition(
        &self,
    ) -> &HashMap<String, DestinyItemTierTypeDefinition> {
        &self.destiny_item_tier_type_definition
    }

    pub fn destiny_inventory_bucket_definition(
        &self,
    ) -> &HashMap<String, DestinyInventoryBucketDefinition> {
        &self.destiny_inventory_bucket_definition
    }

    pub fn destiny_inventory_item_definition(
        &self,
    ) -> &HashMap<String, DestinyInventoryItemDefinition> {
        &self.destiny_inventory_item_definition
    }

    // pub fn destiny_inventory_item_lite_definition(&self) -> &HashMap<String, DestinyInventoryItemLiteDefinition> {
    //     &self.destiny_inventory_item_lite_definition
    // }

    pub fn destiny_location_definition(&self) -> &HashMap<String, DestinyLocationDefinition> {
        &self.destiny_location_definition
    }

    pub fn destiny_lore_definition(&self) -> &HashMap<String, DestinyLoreDefinition> {
        &self.destiny_lore_definition
    }

    pub fn destiny_material_requirement_set_definition(
        &self,
    ) -> &HashMap<String, DestinyMaterialRequirementSetDefinition> {
        &self.destiny_material_requirement_set_definition
    }

    pub fn destiny_medal_tier_definition(&self) -> &HashMap<String, DestinyMedalTierDefinition> {
        &self.destiny_medal_tier_definition
    }

    pub fn destiny_metric_definition(&self) -> &HashMap<String, DestinyMetricDefinition> {
        &self.destiny_metric_definition
    }

    pub fn destiny_milestone_definition(&self) -> &HashMap<String, DestinyMilestoneDefinition> {
        &self.destiny_milestone_definition
    }

    // pub fn destiny_node_step_summary_definition(&self) -> &HashMap<String, DestinyNodeStepSummaryDefinition> {
    //     &self.destiny_node_step_summary_definition
    // }

    pub fn destiny_objective_definition(&self) -> &HashMap<String, DestinyObjectiveDefinition> {
        &self.destiny_objective_definition
    }

    pub fn destiny_place_definition(&self) -> &HashMap<String, DestinyPlaceDefinition> {
        &self.destiny_place_definition
    }

    // pub fn destiny_platform_bucket_mapping_definition(&self) -> &HashMap<String, DestinyPlatformBucketMappingDefinition> {
    //     &self.destiny_platform_bucket_mapping_definition
    // }

    pub fn destiny_plug_set_definition(&self) -> &HashMap<String, DestinyPlugSetDefinition> {
        &self.destiny_plug_set_definition
    }

    pub fn destiny_power_cap_definition(&self) -> &HashMap<String, DestinyPowerCapDefinition> {
        &self.destiny_power_cap_definition
    }

    pub fn destiny_presentation_node_definition(
        &self,
    ) -> &HashMap<String, DestinyPresentationNodeDefinition> {
        &self.destiny_presentation_node_definition
    }

    pub fn destiny_progression_definition(&self) -> &HashMap<String, DestinyProgressionDefinition> {
        &self.destiny_progression_definition
    }

    pub fn destiny_progression_level_requirement_definition(
        &self,
    ) -> &HashMap<String, DestinyProgressionLevelRequirementDefinition> {
        &self.destiny_progression_level_requirement_definition
    }

    pub fn destiny_progression_mapping_definition(
        &self,
    ) -> &HashMap<String, DestinyProgressionMappingDefinition> {
        &self.destiny_progression_mapping_definition
    }

    pub fn destiny_race_definition(&self) -> &HashMap<String, DestinyRaceDefinition> {
        &self.destiny_race_definition
    }

    pub fn destiny_record_definition(&self) -> &HashMap<String, DestinyRecordDefinition> {
        &self.destiny_record_definition
    }

    pub fn destiny_report_reason_category_definition(
        &self,
    ) -> &HashMap<String, DestinyReportReasonCategoryDefinition> {
        &self.destiny_report_reason_category_definition
    }

    // pub fn destiny_reward_adjuster_pointer_definition(&self) -> &HashMap<String, DestinyRewardAdjusterPointerDefinition> {
    //     &self.destiny_reward_adjuster_pointer_definition
    // }

    // pub fn destiny_reward_adjuster_progression_map_definition(&self) -> &HashMap<String, DestinyRewardAdjusterProgressionMapDefinition> {
    //     &self.destiny_reward_adjuster_progression_map_definition
    // }

    // pub fn destiny_reward_item_list_definition(&self) -> &HashMap<String, DestinyRewardItemListDefinition> {
    //     &self.destiny_reward_item_list_definition
    // }

    // pub fn destiny_reward_mapping_definition(&self) -> &HashMap<String, DestinyRewardMappingDefinition> {
    //     &self.destiny_reward_mapping_definition
    // }

    // pub fn destiny_reward_sheet_definition(&self) -> &HashMap<String, DestinyRewardSheetDefinition> {
    //     &self.destiny_reward_sheet_definition
    // }

    pub fn destiny_reward_source_definition(
        &self,
    ) -> &HashMap<String, DestinyRewardSourceDefinition> {
        &self.destiny_reward_source_definition
    }

    // pub fn destiny_sack_reward_item_list_definition(&self) -> &HashMap<String, DestinySackRewardItemListDefinition> {
    //     &self.destiny_sack_reward_item_list_definition
    // }

    pub fn destiny_sandbox_pattern_definition(
        &self,
    ) -> &HashMap<String, DestinySandboxPatternDefinition> {
        &self.destiny_sandbox_pattern_definition
    }

    pub fn destiny_sandbox_perk_definition(
        &self,
    ) -> &HashMap<String, DestinySandboxPerkDefinition> {
        &self.destiny_sandbox_perk_definition
    }

    pub fn destiny_season_definition(&self) -> &HashMap<String, DestinySeasonDefinition> {
        &self.destiny_season_definition
    }

    pub fn destiny_season_pass_definition(&self) -> &HashMap<String, DestinySeasonPassDefinition> {
        &self.destiny_season_pass_definition
    }

    pub fn destiny_socket_category_definition(
        &self,
    ) -> &HashMap<String, DestinySocketCategoryDefinition> {
        &self.destiny_socket_category_definition
    }

    pub fn destiny_socket_type_definition(&self) -> &HashMap<String, DestinySocketTypeDefinition> {
        &self.destiny_socket_type_definition
    }

    pub fn destiny_stat_definition(&self) -> &HashMap<String, DestinyStatDefinition> {
        &self.destiny_stat_definition
    }

    pub fn destiny_stat_group_definition(&self) -> &HashMap<String, DestinyStatGroupDefinition> {
        &self.destiny_stat_group_definition
    }

    pub fn destiny_talent_grid_definition(&self) -> &HashMap<String, DestinyTalentGridDefinition> {
        &self.destiny_talent_grid_definition
    }

    pub fn destiny_trait_definition(&self) -> &HashMap<String, DestinyTraitDefinition> {
        &self.destiny_trait_definition
    }

    pub fn destiny_trait_category_definition(
        &self,
    ) -> &HashMap<String, DestinyTraitCategoryDefinition> {
        &self.destiny_trait_category_definition
    }

    // pub fn destiny_unlock_count_mapping_definition(&self) -> &HashMap<String, DestinyUnlockCountMappingDefinition> {
    //     &self.destiny_unlock_count_mapping_definition
    // }

    pub fn destiny_unlock_definition(&self) -> &HashMap<String, DestinyUnlockDefinition> {
        &self.destiny_unlock_definition
    }

    // pub fn destiny_unlock_event_definition(&self) -> &HashMap<String, DestinyUnlockEventDefinition> {
    //     &self.destiny_unlock_event_definition
    // }

    // pub fn destiny_unlock_expression_mapping_definition(&self) -> &HashMap<String, DestinyUnlockExpressionMappingDefinition> {
    //     &self.destiny_unlock_expression_mapping_definition
    // }

    pub fn destiny_unlock_value_definition(
        &self,
    ) -> &HashMap<String, DestinyUnlockValueDefinition> {
        &self.destiny_unlock_value_definition
    }

    pub fn destiny_vendor_definition(&self) -> &HashMap<String, DestinyVendorDefinition> {
        &self.destiny_vendor_definition
    }

    pub fn destiny_vendor_group_definition(
        &self,
    ) -> &HashMap<String, DestinyVendorGroupDefinition> {
        &self.destiny_vendor_group_definition
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.DestinyClass
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum DestinyClass {
    Titan = 0,
    Hunter = 1,
    Warlock = 2,
    Unknown = 3,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.DestinyGender
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum DestinyGender {
    Male = 0,
    Female = 1,
    Unknown = 2,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.DestinyItemQuantity
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemQuantity {
    has_conditional_visibility: Option<bool>,
    item_hash: Option<u32>,
    item_instance_id: Option<i64>,
    quantity: Option<i32>,
}

impl DestinyItemQuantity {
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
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.DestinyRace
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum DestinyRace {
    Human = 0,
    Awoken = 1,
    Exo = 2,
    Unknown = 3,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.DyeReference
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DyeReference {
    channel_hash: Option<u32>,
    dye_hash: Option<u32>,
}

impl DyeReference {
    pub fn channel_hash(&self) -> Option<u32> {
        self.channel_hash
    }

    pub fn dye_hash(&self) -> Option<u32> {
        self.dye_hash
    }
}
