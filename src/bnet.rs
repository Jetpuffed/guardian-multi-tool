use std::collections::HashMap;
use std::error::Error;

use chrono::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/* Definitions with no known documentation:
 *     DestinyAchievementDefinition
 *     DestinyArtDyeChannelDefinition
 *     DestinyActivityInteractableDefinition
 *     DestinyBondDefinition
 *     DestinyCharacterCustomizationCategoryDefinition
 *     DestinyCharacterCustomizationOptionDefinition
 *     DestinyEntitlementOfferDefinition
 *     DestinyInventoryItemLiteDefinition
 *     DestinyNodeStepSummaryDefinition
 *     DestinyPlatformBucketMappingDefinition
 *     DestinyRewardAdjusterPointerDefinition
 *     DestinyRewardAdjusterProgressionMapDefinition
 *     DestinyRewardItemListDefinition
 *     DestinyRewardMappingDefinition
 *     DestinyRewardSheetDefinition
 *     DestinySackRewardItemListDefinition
 *     DestinyUnlockCountMappingDefinition
 *     DestinyUnlockEventDefinition
 *     DestinyUnlockExpressionMappingDefinition
 */

pub const BASE_URL: &str = "https://www.bungie.net";

/// https://bungie-net.github.io/#Destiny2.GetDestinyManifest
pub async fn get_destiny_manifest(
    c: &Client,
) -> Result<BungieResponse<DestinyManifest>, Box<dyn Error>> {
    const PATH: &str = "/platform/destiny2/manifest";

    match c.get([BASE_URL, PATH].join("")).send().await {
        Ok(resp) => return Ok(resp.json::<BungieResponse<DestinyManifest>>().await?),
        Err(e) => panic!("{}", e),
    }
}

/// https://bungie-net.github.io/#Destiny2.GetDestinyManifest
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BungieResponse<T> {
    response: T,
    error_code: i32,
    throttle_seconds: i32,
    error_status: String,
    message: String,
    message_data: HashMap<String, String>,
    detailed_error_trace: Option<String>,
}

impl<T> BungieResponse<T> {
    pub fn response(&self) -> &T {
        &self.response
    }

    pub fn error_code(&self) -> i32 {
        self.error_code
    }

    pub fn throttle_seconds(&self) -> i32 {
        self.throttle_seconds
    }

    pub fn error_status(&self) -> &str {
        self.error_status.as_ref()
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    pub fn message_data(&self) -> &HashMap<String, String> {
        &self.message_data
    }

    pub fn detailed_error_trace(&self) -> Option<&String> {
        self.detailed_error_trace.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Config.DestinyManifest
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyManifest {
    version: String,
    mobile_asset_content_path: String,
    mobile_gear_asset_data_bases: Vec<GearAssetDataBaseDefinition>,
    mobile_world_content_paths: HashMap<String, String>,
    json_world_content_paths: HashMap<String, String>,
    json_world_component_content_paths: HashMap<String, HashMap<String, String>>,
    mobile_clan_banner_database_path: String,
    mobile_gear_c_d_n: HashMap<String, String>,
    icon_image_pyramid_info: Vec<ImagePyramidEntry>,
}

impl DestinyManifest {
    pub fn version(&self) -> &str {
        self.version.as_ref()
    }

    pub fn mobile_asset_content_path(&self) -> &str {
        self.mobile_asset_content_path.as_ref()
    }

    pub fn mobile_gear_asset_data_bases(&self) -> &[GearAssetDataBaseDefinition] {
        self.mobile_gear_asset_data_bases.as_ref()
    }

    pub fn mobile_world_content_paths(&self) -> &HashMap<String, String> {
        &self.mobile_world_content_paths
    }

    pub fn json_world_content_paths(&self) -> &HashMap<String, String> {
        &self.json_world_content_paths
    }

    pub fn json_world_component_content_paths(&self) -> &HashMap<String, HashMap<String, String>> {
        &self.json_world_component_content_paths
    }

    pub fn mobile_clan_banner_database_path(&self) -> &str {
        self.mobile_clan_banner_database_path.as_ref()
    }

    pub fn mobile_gear_cdn(&self) -> &HashMap<String, String> {
        &self.mobile_gear_c_d_n
    }

    pub fn icon_image_pyramid_info(&self) -> &[ImagePyramidEntry] {
        self.icon_image_pyramid_info.as_ref()
    }
}

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
    // destiny_activity_interactable_definition: HashMap<String, DestinyActivityGraphDefinition>,
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
    destiny_material_requirement_set_definition: HashMap<String, DestinyMaterialRequirementSetDefinition>,
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
    destiny_progression_level_requirement_definition: HashMap<String, DestinyProgressionLevelRequirementDefinition>,
    destiny_progression_mapping_definition: HashMap<String, DestinyProgressionMappingDefinition>,
    destiny_race_definition: HashMap<String, DestinyRaceDefinition>,
    destiny_record_definition: HashMap<String, DestinyRecordDefinition>,
    destiny_report_reason_category_definition: HashMap<String, DestinyReportReasonCategoryDefinition>,
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

/// https://bungie-net.github.io/#/components/schemas/Dates.DateRange
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateRange {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    original_display_properties: DestinyDisplayPropertiesDefinition,
    selection_screen_display_properties: Option<DestinyDisplayPropertiesDefinition>,
    release_icon: Option<String>,
    release_time: i32,
    activity_light_level: i32,
    destination_hash: u32,
    place_hash: u32,
    activity_type_hash: u32,
    tier: i32,
    pgcr_image: Option<String>,
    rewards: Option<Vec<DestinyActivityRewardDefinition>>,
    modifiers: Option<Vec<DestinyActivityModifierReferenceDefinition>>,
    is_playlist: bool,
    challenges: Option<Vec<DestinyActivityChallengeDefinition>>,
    optional_unlock_strings: Option<Vec<DestinyActivityUnlockStringDefinition>>,
    playlist_items: Option<Vec<DestinyActivityPlaylistItemDefinition>>,
    activity_graph_list: Option<Vec<DestinyActivityGraphListEntryDefinition>>,
    matchmaking: Option<DestinyActivityMatchmakingBlockDefinition>,
    guided_game: Option<DestinyActivityGuidedBlockDefinition>,
    direct_activity_mode_hash: Option<u32>,
    direct_activity_mode_type: Option<i32>,
    loadouts: Option<Vec<DestinyActivityLoadoutRequirementSet>>,
    activity_mode_hashes: Option<Vec<u32>>,
    activity_mode_types: Option<Vec<i32>>,
    is_pvp: Option<bool>,
    insertion_points: Option<Vec<DestinyActivityInsertionPointDefinition>>,
    activity_location_mappings: Option<Vec<DestinyEnvironmentLocationMapping>>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityChallengeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityChallengeDefinition {
    objective_hash: u32,
    dummy_rewards: Vec<DestinyItemQuantity>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphArtElementDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphArtElementDefinition {
    position: DestinyPositionDefinition,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphConnectionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphConnectionDefinition {
    source_node_hash: u32,
    dest_node_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphDefinition {
    nodes: Vec<DestinyActivityGraphNodeDefinition>,
    art_elements: Vec<DestinyActivityGraphArtElementDefinition>,
    connections: Vec<DestinyActivityGraphConnectionDefinition>,
    display_objectives: Vec<DestinyActivityGraphDisplayObjectiveDefinition>,
    display_progressions: Vec<DestinyActivityGraphDisplayProgressionDefinition>,
    linked_graphs: Vec<DestinyLinkedGraphDefinition>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphDisplayObjectiveDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphDisplayObjectiveDefinition {
    id: u32,
    objective_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphDisplayProgressionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphDisplayProgressionDefinition {
    id: u32,
    progression_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityGraphListEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphListEntryDefinition {
    activity_graph_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphNodeActivityDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphNodeActivityDefinition {
    node_activity_id: u32,
    activity_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphNodeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphNodeDefinition {
    node_id: u32,
    override_display: DestinyDisplayPropertiesDefinition,
    position: DestinyPositionDefinition,
    featuring_states: Vec<DestinyActivityGraphNodeFeaturingStateDefinition>,
    activities: Vec<DestinyActivityGraphNodeActivityDefinition>,
    states: Vec<DestinyActivityGraphNodeStateEntry>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphNodeFeaturingStateDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphNodeFeaturingStateDefinition {
    highlight_type: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphNodeStateEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphNodeStateEntry {
    state: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityGuidedBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGuidedBlockDefinition {
    guided_max_lobby_size: i32,
    guided_min_lobby_size: i32,
    guided_disband_count: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityInsertionPointDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityInsertionPointDefinition {
    phase_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityLoadoutRequirement
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityLoadoutRequirement {
    equipment_slot_hash: u32,
    allowed_equipped_item_hashes: Vec<u32>,
    allowed_weapon_sub_types: Vec<i32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityLoadoutRequirementSet
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityLoadoutRequirementSet {
    requirements: Vec<DestinyActivityLoadoutRequirement>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityMatchmakingBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityMatchmakingBlockDefinition {
    is_matchmade: bool,
    min_party: i32,
    max_party: i32,
    max_players: i32,
    requires_guardian_oath: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityModeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityModeDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    pgcr_image: String,
    mode_type: i32,
    activity_mode_category: i32,
    is_team_based: bool,
    is_aggregate_mode: bool,
    parent_hashes: Option<Vec<u32>>,
    friendly_name: String,
    activity_mode_mappings: Option<HashMap<u32, i32>>,
    display: bool,
    order: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.ActivityModifiers.DestinyActivityModifierDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityModifierDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    display_in_nav_mode: bool,
    display_in_activity_selection: bool,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityModifierReferenceDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityModifierReferenceDefinition {
    activity_modifier_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityPlaylistItemDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityPlaylistItemDefinition {
    activity_hash: u32,
    direct_activity_mode_hash: Option<u32>,
    direct_activity_mode_type: Option<i32>,
    activity_mode_hashes: Vec<u32>,
    activity_mode_types: Vec<i32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityRewardDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityRewardDefinition {
    reward_text: Option<String>,
    reward_items: Vec<DestinyItemQuantity>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityTypeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityTypeDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityUnlockStringDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityUnlockStringDefinition {
    display_string: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Animations.DestinyAnimationReference
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyAnimationReference {
    anim_name: String,
    anim_identifier: String,
    path: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyArrangementRegionFilterDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyArrangementRegionFilterDefinition {
    art_arrangement_region_hash: u32,
    art_arrangement_region_index: i32,
    stat_hash: u32,
    arrangement_index_by_stat_value: HashMap<i32, i32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyArtDyeReference
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyArtDyeReference {
    art_dye_channel_hash: Option<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Artifacts.DestinyArtifactDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyArtifactDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    translation_block: DestinyItemTranslationBlockDefinition,
    tiers: Vec<DestinyArtifactTierDefinition>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Artifacts.DestinyArtifactTierDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyArtifactTierDefinition {
    tier_hash: u32,
    display_title: String,
    progress_requirement_message: Option<String>,
    items: Vec<DestinyArtifactTierItemDefinition>,
    minimum_unlock_points_used_requirement: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Artifacts.DestinyArtifactTierItemDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyArtifactTierItemDefinition {
    item_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.BreakerTypes.DestinyBreakerTypeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyBreakerTypeDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    enum_value: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyBubbleDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyBubbleDefinition {
    hash: u32,
    display_properties: DestinyDisplayPropertiesDefinition,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Checklists.DestinyChecklistDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyChecklistDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    view_action_string: String,
    scope: i32,
    entries: Vec<DestinyChecklistEntryDefinition>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Checklists.DestinyChecklistEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyChecklistEntryDefinition {
    hash: u32,
    display_properties: DestinyDisplayPropertiesDefinition,
    destination_hash: Option<u32>,
    location_hash: Option<u32>,
    bubble_hash: Option<u32>,
    activity_hash: Option<u32>,
    item_hash: Option<u32>,
    vendor_hash: Option<u32>,
    vendor_interaction_index: Option<i32>,
    scope: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.DestinyClass
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum DestinyClass {
    Titan = 0,
    Hunter = 1,
    Warlock = 2,
    Unknown = 3,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyClassDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyClassDefinition {
    class_type: i32,
    display_properties: DestinyDisplayPropertiesDefinition,
    gendered_class_names: HashMap<DestinyGender, String>,
    gendered_class_names_by_gender_hash: HashMap<u32, String>,
    mentor_vendor_hash: Option<u32>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Collectibles.DestinyCollectibleAcquisitionBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyCollectibleAcquisitionBlock {
    acquire_material_requirement_hash: Option<u32>,
    acquire_timestamp_unlock_value_hash: Option<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Collectibles.DestinyCollectibleDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyCollectibleDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    scope: i32,
    source_string: Option<String>,
    source_hash: Option<u32>,
    item_hash: u32,
    acquisition_info: Option<DestinyCollectibleAcquisitionBlock>,
    state_info: Option<DestinyCollectibleStateBlock>,
    presentation_info: Option<DestinyPresentationChildBlock>,
    presentation_node_type: i32,
    trait_ids: Option<Vec<String>>,
    trait_hashes: Option<Vec<u32>>,
    parent_node_hashes: Option<Vec<u32>>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Collectibles.DestinyCollectibleStateBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyCollectibleStateBlock {
    obscured_override_item_hash: Option<u32>,
    requirements: DestinyPresentationNodeRequirementsBlock,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Misc.DestinyColor
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyColor {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyDamageTypeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyDamageTypeDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    transparent_icon_path: Option<String>,
    show_icon: bool,
    enum_value: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyDerivedItemCategoryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyDerivedItemCategoryDefinition {
    category_description: String,
    items: Vec<DestinyDerivedItemDefinition>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyDerivedItemDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyDerivedItemDefinition {
    item_hash: u32,
    item_name: Option<String>,
    item_detail: Option<String>,
    item_description: Option<String>,
    icon_path: Option<String>,
    vendor_item_index: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyDestinationBubbleSettingDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyDestinationBubbleSettingDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyDestinationDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyDestinationDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    place_hash: u32,
    default_freeroam_activity_hash: u32,
    activity_graph_entries: Vec<DestinyActivityGraphListEntryDefinition>,
    bubble_settings: Vec<DestinyDestinationBubbleSettingDefinition>,
    bubbles: Vec<DestinyBubbleDefinition>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyDisplayCategoryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyDisplayCategoryDefinition {
    index: i32,
    identifier: String,
    display_category_hash: u32,
    display_properties: DestinyDisplayPropertiesDefinition,
    display_in_banner: bool,
    progression_hash: Option<u32>,
    sort_order: u32,
    display_style_hash: Option<u32>,
    display_style_identifier: Option<String>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyDisplayPropertiesDefinition {
    description: Option<String>,
    name: Option<String>,
    icon: Option<String>,
    icon_sequences: Option<Vec<DestinyIconSequenceDefinition>>,
    high_res_icon: Option<String>,
    has_icon: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyEnergyCapacityEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyEnergyCapacityEntry {
    capacity_value: i32,
    energy_type_hash: u32,
    energy_type: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyEnergyCostEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyEnergyCostEntry {
    energy_cost: i32,
    energy_type_hash: u32,
    energy_type: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.EnergyTypes.DestinyEnergyTypeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyEnergyTypeDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    transparent_icon_path: String,
    show_icon: bool,
    enum_value: i32,
    capacity_stat_hash: Option<u32>,
    cost_stat_hash: u32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Constants.DestinyEnvironmentLocationMapping
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyEnvironmentLocationMapping {
    location_hash: u32,
    activation_source: String,
    item_hash: Option<u32>,
    objective_hash: Option<u32>,
    activity_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyEquipmentSlotDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyEquipmentSlotDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    equipment_category_hash: u32,
    bucket_type_hash: u32,
    apply_custom_art_dyes: bool,
    art_dye_channels: Vec<DestinyArtDyeReference>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyEquippingBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyEquippingBlockDefinition {
    gearset_item_hash: Option<u32>,
    unique_label: Option<String>,
    unique_label_hash: u32,
    equipment_slot_type_hash: u32,
    attributes: i32,
    ammo_type: i32,
    display_strings: Vec<String>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyFactionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyFactionDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    progression_hash: u32,
    token_values: Option<HashMap<u32, u32>>,
    reward_item_hash: u32,
    reward_vendor_hash: u32,
    vendors: Option<Vec<DestinyFactionVendorDefinition>>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyFactionVendorDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyFactionVendorDefinition {
    vendor_hash: u32,
    destination_hash: u32,
    background_image_path: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyGearArtArrangementReference
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyGearArtArrangementReference {
    class_hash: u32,
    art_arrangement_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.DestinyGender
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum DestinyGender {
    Male = 0,
    Female = 1,
    Unknown = 2,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyGenderDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyGenderDefinition {
    gender_type: i32,
    display_properties: DestinyDisplayPropertiesDefinition,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Common.DestinyIconSequenceDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyIconSequenceDefinition {
    frames: Vec<String>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinyInsertPlugActionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyInsertPlugActionDefinition {
    action_execute_seconds: i32,
    action_type: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyInventoryBucketDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyInventoryBucketDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    scope: i32,
    category: i32,
    bucket_order: i32,
    item_count: i32,
    location: i32,
    has_transfer_destination: bool,
    enabled: bool,
    fifo: bool,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyInventoryItemDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyInventoryItemDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    tooltip_notifications: Option<Vec<DestinyItemTooltipNotification>>,
    collectible_hash: Option<u32>,
    icon_watermark: Option<String>,
    icon_watermark_shelved: Option<String>,
    secondary_icon: Option<String>,
    secondary_overlay: Option<String>,
    secondary_special: Option<String>,
    background_color: Option<DestinyColor>,
    screenshot: Option<String>,
    item_type_display_name: Option<String>,
    flavor_text: Option<String>,
    ui_item_display_style: Option<String>,
    item_type_and_tier_display_name: Option<String>,
    display_source: Option<String>,
    tooltip_style: Option<String>,
    action: Option<DestinyItemActionBlockDefinition>,
    crafting: Option<DestinyItemCraftingBlockDefinition>,
    inventory: DestinyItemInventoryBlockDefinition,
    set_data: Option<DestinyItemSetBlockDefinition>,
    stats: Option<DestinyItemStatBlockDefinition>,
    emblem_objective_hash: Option<u32>,
    equipping_block: Option<DestinyEquippingBlockDefinition>,
    translation_block: Option<DestinyItemTranslationBlockDefinition>,
    preview: Option<DestinyItemPreviewBlockDefinition>,
    quality: Option<DestinyItemQualityBlockDefinition>,
    value: Option<DestinyItemValueBlockDefinition>,
    source_data: Option<DestinyItemSourceBlockDefinition>,
    objectives: Option<DestinyItemObjectiveBlockDefinition>,
    metrics: Option<DestinyItemMetricBlockDefinition>,
    plug: Option<DestinyItemPlugDefinition>,
    gearset: Option<DestinyItemGearsetBlockDefinition>,
    sack: Option<DestinyItemSackBlockDefinition>,
    sockets: Option<DestinyItemSocketBlockDefinition>,
    summary: Option<DestinyItemSummaryBlockDefinition>,
    talent_grid: Option<DestinyItemTalentGridBlockDefinition>,
    investment_stats: Option<Vec<DestinyItemInvestmentStatDefinition>>,
    perks: Option<Vec<DestinyItemPerkEntryDefinition>>,
    lore_hash: Option<u32>,
    summary_item_hash: Option<u32>,
    animations: Option<Vec<DestinyAnimationReference>>,
    allow_actions: bool,
    links: Option<Vec<HyperlinkReference>>,
    does_postmaster_pull_have_side_effects: bool,
    non_transferrable: bool,
    item_category_hashes: Option<Vec<u32>>,
    special_item_type: i32,
    item_type: i32,
    item_sub_type: i32,
    class_type: i32,
    breaker_type: i32,
    breaker_type_hash: Option<u32>,
    equippable: bool,
    damage_type_hashes: Option<Vec<u32>>,
    damage_types: Option<Vec<i32>>,
    default_damage_type: i32,
    default_damage_type_hash: Option<u32>,
    season_hash: Option<u32>,
    is_wrapper: bool,
    trait_ids: Option<Vec<String>>,
    trait_hashes: Option<Vec<u32>>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyInventoryItemStatDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyInventoryItemStatDefinition {
    stat_hash: u32,
    value: i32,
    minimum: i32,
    maximum: i32,
    display_maximum: Option<i32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemActionBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemActionBlockDefinition {
    verb_name: String,
    verb_description: String,
    is_positive: bool,
    overlay_screen_name: Option<String>,
    overlay_icon: Option<String>,
    required_cooldown_seconds: i32,
    required_items: Vec<DestinyItemActionRequiredItemDefinition>,
    progression_rewards: Vec<DestinyProgressionRewardDefinition>,
    action_type_label: Option<String>,
    required_location: Option<String>,
    required_cooldown_hash: u32,
    delete_on_action: bool,
    consume_entire_stack: bool,
    use_on_acquire: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemActionRequiredItemDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemActionRequiredItemDefinition {
    count: i32,
    item_hash: u32,
    delete_on_action: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemCategoryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemCategoryDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    visible: bool,
    deprecated: bool,
    short_title: String,
    item_type_regex: Option<String>,
    grant_destiny_breaker_type: i32,
    plug_category_identifier: Option<String>,
    item_type_regex_not: Option<String>,
    origin_bucket_identifier: Option<String>,
    grant_destiny_item_type: i32,
    grant_destiny_sub_type: i32,
    grant_destiny_class: i32,
    trait_id: Option<String>,
    grouped_category_hashes: Vec<u32>,
    parent_category_hashes: Vec<u32>,
    group_category_only: bool,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemCraftingBlockBonusPlugDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemCraftingBlockBonusPlugDefinition {
    socket_type_hash: u32,
    plug_item_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemCraftingBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemCraftingBlockDefinition {
    output_item_hash: u32,
    required_socket_type_hashes: Vec<u32>,
    failed_requirement_strings: Vec<String>,
    base_material_requirements: Option<u32>,
    bonus_plugs: Vec<DestinyItemCraftingBlockBonusPlugDefinition>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemCreationEntryLevelDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemCreationEntryLevelDefinition {
    level: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemGearsetBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemGearsetBlockDefinition {
    tracking_value_max: i32,
    item_list: Vec<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemIntrinsicSocketEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemIntrinsicSocketEntryDefinition {
    plug_item_hash: u32,
    socket_type_hash: u32,
    default_visible: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemInventoryBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemInventoryBlockDefinition {
    stack_unique_label: Option<String>,
    max_stack_size: i32,
    bucket_type_hash: u32,
    recovery_bucket_type_hash: u32,
    tier_type_hash: u32,
    is_instance_item: bool,
    tier_type_name: Option<String>,
    tier_type: i32,
    expiration_tooltip: Option<String>,
    expired_in_activity_message: Option<String>,
    expired_in_orbit_message: Option<String>,
    suppress_expiration_when_objectives_complete: bool,
    recipe_item_hash: Option<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemInvestmentStatDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemInvestmentStatDefinition {
    stat_type_hash: u32,
    value: i32,
    is_conditionally_active: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemMetricBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemMetricBlockDefinition {
    available_metric_category_node_hashes: Vec<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemObjectiveBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemObjectiveBlockDefinition {
    objective_hashes: Vec<u32>,
    display_activity_hashes: Vec<u32>,
    require_full_objective_completion: bool,
    questline_item_hash: u32,
    narrative: String,
    objective_verb_name: String,
    quest_type_identifier: String,
    quest_type_hash: u32,
    per_objective_display_properties: Vec<DestinyObjectiveDisplayProperties>,
    display_as_stat_tracker: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemPerkEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemPerkEntryDefinition {
    requirement_display_string: String,
    perk_hash: u32,
    perk_visibility: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyItemPlugDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemPlugDefinition {
    insertion_rules: Vec<DestinyPlugRuleDefinition>,
    plug_category_identifier: String,
    plug_category_hash: u32,
    on_action_recreate_self: bool,
    insertion_material_requirement_hash: u32,
    preview_item_override_hash: u32,
    enabled_material_requirement_hash: u32,
    enabled_rules: Vec<DestinyPlugRuleDefinition>,
    ui_plug_label: String,
    plug_style: i32,
    plug_availability: i32,
    alternate_ui_plug_label: String,
    alternate_plug_style: i32,
    is_dummy_plug: bool,
    parent_item_override: Option<DestinyParentItemOverride>,
    energy_capacity: Option<DestinyEnergyCapacityEntry>,
    energy_cost: Option<DestinyEnergyCostEntry>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemPreviewBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemPreviewBlockDefinition {
    screen_style: String,
    preview_vendor_hash: u32,
    artifact_hash: Option<u32>,
    preview_action_string: String,
    derived_item_categories: Option<Vec<DestinyDerivedItemCategoryDefinition>>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemQualityBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemQualityBlockDefinition {
    item_levels: Vec<i32>,
    quality_level: i32,
    infusion_category_name: String,
    infusion_category_hash: u32,
    infusion_category_hashes: Vec<u32>,
    progression_level_requirement: Option<u32>,
    current_version: u32,
    versions: Vec<DestinyItemVersionDefinition>,
    display_version_watermark_icons: Vec<String>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.DestinyItemQuantity
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemQuantity {
    item_hash: u32,
    item_instance_id: Option<i64>,
    quantity: i32,
    has_conditional_visibility: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSackBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSackBlockDefinition {
    detail_action: String,
    open_action: String,
    select_item_count: i32,
    vendor_sack_type: Option<String>,
    open_on_acquire: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSetBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSetBlockDefinition {
    item_list: Vec<DestinyItemSetBlockEntryDefinition>,
    require_ordered_set_item_add: bool,
    set_is_featured: bool,
    set_type: String,
    quest_line_name: String,
    quest_line_description: String,
    quest_step_summary: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSetBlockEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSetBlockEntryDefinition {
    tracking_value: i32,
    item_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSocketBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSocketBlockDefinition {
    detail: String,
    socket_entries: Vec<DestinyItemSocketEntryDefinition>,
    intrinsic_sockets: Vec<DestinyItemIntrinsicSocketEntryDefinition>,
    socket_categories: Vec<DestinyItemSocketCategoryDefinition>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSocketCategoryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSocketCategoryDefinition {
    socket_category_hash: u32,
    socket_indexes: Vec<i32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSocketEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSocketEntryDefinition {
    socket_type_hash: u32,
    single_initial_item_hash: u32,
    reusable_plug_items: Vec<DestinyItemSocketEntryPlugItemDefinition>,
    prevent_initialization_on_vendor_purchase: bool,
    hide_perks_in_item_tooltip: bool,
    plug_sources: i32,
    reusable_plug_set_hash: Option<u32>,
    randomized_plug_set_hash: Option<u32>,
    default_visible: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSocketEntryPlugItemDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSocketEntryPlugItemDefinition {
    plug_item_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSocketEntryPlugItemRandomizedDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSocketEntryPlugItemRandomizedDefinition {
    crafting_requirements: Option<DestinyPlugItemCraftingRequirements>,
    currently_can_roll: bool,
    plug_item_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSourceBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSourceBlockDefinition {
    source_hashes: Vec<u32>,
    sources: Vec<DestinyItemSourceDefinition>,
    exclusive: i32,
    vendor_sources: Vec<DestinyItemVendorSourceReference>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sources.DestinyItemSourceDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSourceDefinition {
    level: i32,
    min_quality: i32,
    max_quality: i32,
    min_level_required: i32,
    max_level_required: i32,
    computed_stats: HashMap<u32, DestinyInventoryItemStatDefinition>,
    source_hashes: Vec<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemStatBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemStatBlockDefinition {
    disable_primary_stat_display: bool,
    stat_group_hash: Option<u32>,
    stats: HashMap<u32, DestinyInventoryItemStatDefinition>,
    has_displayable_stats: bool,
    primary_base_stat_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSummaryBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSummaryBlockDefinition {
    sort_priority: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemTalentGridBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemTalentGridBlockDefinition {
    talent_grid_hash: u32,
    item_detail_string: String,
    build_name: Option<String>,
    hud_damage_type: i32,
    hud_icon: Option<String>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyItemTierTypeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemTierTypeDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    infusion_process: DestinyItemTierTypeInfusionBlock,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyItemTierTypeInfusionBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemTierTypeInfusionBlock {
    base_quality_transfer_ratio: f32,
    minimum_quality_increment: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemTooltipNotification
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemTooltipNotification {
    display_string: String,
    display_style: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemTranslationBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemTranslationBlockDefinition {
    weapon_pattern_identifier: Option<String>,
    weapon_pattern_hash: u32,
    default_dyes: Vec<DyeReference>,
    locked_dyes: Vec<DyeReference>,
    custom_dyes: Vec<DyeReference>,
    arrangements: Vec<DestinyGearArtArrangementReference>,
    has_geometry: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemValueBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemValueBlockDefinition {
    item_value: Vec<DestinyItemQuantity>,
    value_description: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemVendorSourceReference
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemVendorSourceReference {
    vendor_hash: u32,
    vendor_item_indexes: Vec<i32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemVersionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemVersionDefinition {
    power_cap_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyLinkedGraphDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyLinkedGraphDefinition {
    description: String,
    name: String,
    unlock_expression: DestinyUnlockExpressionDefinition,
    linked_graph_id: u32,
    linked_graphs: Vec<DestinyLinkedGraphEntryDefinition>,
    overview: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyLinkedGraphEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyLinkedGraphEntryDefinition {
    activity_graph_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyLocationDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyLocationDefinition {
    vendor_hash: u32,
    location_releases: Option<Vec<DestinyLocationReleaseDefinition>>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyLocationReleaseDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyLocationReleaseDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    small_transparent_icon: Option<String>,
    map_icon: Option<String>,
    large_transparent_icon: Option<String>,
    spawn_point: u32,
    destination_hash: u32,
    activity_hash: u32,
    activity_graph_hash: u32,
    activity_graph_node_hash: u32,
    activity_bubble_name: u32,
    activity_path_bundle: u32,
    activity_path_destination: u32,
    nav_point_type: i32,
    world_position: Vec<i32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Lore.DestinyLoreDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyLoreDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    subtitle: Option<String>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyMaterialRequirement
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMaterialRequirement {
    item_hash: u32,
    delete_on_action: bool,
    count: i32,
    count_is_constant: bool,
    omit_from_requirements: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyMaterialRequirementSetDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMaterialRequirementSetDefinition {
    materials: Option<Vec<DestinyMaterialRequirement>>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyMedalTierDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMedalTierDefinition {
    tier_name: String,
    order: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Metrics.DestinyMetricDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMetricDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    tracking_objective_hash: u32,
    lower_value_is_better: bool,
    presentation_node_type: i32,
    trait_ids: Vec<String>,
    trait_hashes: Vec<u32>,
    parent_node_hashes: Vec<u32>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneActivityDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneActivityDefinition {
    conceptual_activity_hash: u32,
    variants: HashMap<u32, DestinyMilestoneActivityVariantDefinition>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneActivityVariantDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneActivityVariantDefinition {
    activity_hash: u32,
    order: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneChallengeActivityDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneChallengeActivityDefinition {
    activity_hash: u32,
    challenges: Vec<DestinyMilestoneChallengeDefinition>,
    activity_graph_nodes: Vec<DestinyMilestoneChallengeActivityGraphNodeEntry>,
    phases: Option<Vec<DestinyMilestoneChallengeActivityPhase>>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneChallengeActivityGraphNodeEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneChallengeActivityGraphNodeEntry {
    activity_graph_hash: u32,
    activity_graph_node_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneChallengeActivityPhase
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneChallengeActivityPhase {
    phase_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneChallengeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneChallengeDefinition {
    challenge_objective_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    display_preference: Option<i32>,
    image: Option<String>,
    milestone_type: i32,
    recruitable: bool,
    friendly_name: Option<String>,
    show_in_explorer: bool,
    show_in_milestones: bool,
    explore_prioritizes_activity_image: bool,
    has_predictable_dates: bool,
    quests: Option<HashMap<u32, DestinyMilestoneQuestDefinition>>,
    rewards: Option<HashMap<u32, DestinyMilestoneRewardCategoryDefinition>>,
    vendors_display_title: Option<String>,
    vendors: Option<Vec<DestinyMilestoneVendorDefinition>>,
    values: Option<HashMap<String, DestinyMilestoneValueDefinition>>,
    is_in_game_milestone: bool,
    activities: Option<Vec<DestinyMilestoneChallengeActivityDefinition>>,
    default_order: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneQuestDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneQuestDefinition {
    quest_item_hash: u32,
    display_properties: DestinyDisplayPropertiesDefinition,
    override_image: Option<String>,
    quest_rewards: Option<DestinyMilestoneQuestRewardsDefinition>,
    activities: Option<HashMap<u32, DestinyMilestoneActivityDefinition>>,
    destination_hash: Option<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneQuestRewardsDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneQuestRewardsDefinition {
    items: Vec<DestinyMilestoneQuestRewardItem>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneQuestRewardItem
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneQuestRewardItem {
    vendor_hash: Option<u32>,
    vendor_item_index: Option<i32>,
    item_hash: u32,
    item_instance_id: Option<i64>,
    quantity: i32,
    has_conditional_visibility: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneRewardCategoryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneRewardCategoryDefinition {
    category_hash: u32,
    category_identifier: String,
    display_properties: DestinyDisplayPropertiesDefinition,
    reward_entries: HashMap<u32, DestinyMilestoneRewardEntryDefinition>,
    order: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneRewardEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneRewardEntryDefinition {
    reward_entry_hash: u32,
    reward_entry_identifier: String,
    items: Vec<DestinyItemQuantity>,
    vendor_hash: Option<u32>,
    display_properties: DestinyDisplayPropertiesDefinition,
    order: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneValueDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneValueDefinition {
    key: String,
    display_properties: DestinyDisplayPropertiesDefinition,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneVendorDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMilestoneVendorDefinition {
    vendor_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyNodeActivationRequirement
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyNodeActivationRequirement {
    grid_level: i32,
    material_requirement_hashes: Vec<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyNodeSocketReplaceResponse
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyNodeSocketReplaceResponse {
    socket_type_hash: u32,
    plug_item_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyNodeStepDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyNodeStepDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    step_index: i32,
    node_step_hash: u32,
    interaction_description: String,
    damage_type: i32,
    damage_type_hash: Option<u32>,
    activation_requirement: DestinyNodeActivationRequirement,
    can_activate_next_step: bool,
    next_step_index: i32,
    is_next_step_random: bool,
    perk_hashes: Vec<u32>,
    start_progression_bar_at_progress: i32,
    stat_hashes: Vec<u32>,
    affects_quality: bool,
    step_groups: Option<DestinyTalentNodeStepGroups>,
    affects_level: bool,
    socket_replacements: Option<Vec<DestinyNodeSocketReplaceResponse>>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyObjectiveDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyObjectiveDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    completion_value: i32,
    scope: i32,
    location_hash: u32,
    allow_negative_value: bool,
    allow_value_change_when_completed: bool,
    is_counting_downward: bool,
    value_style: i32,
    progress_description: String,
    perks: Option<DestinyObjectivePerkEntryDefinition>,
    stats: Option<DestinyObjectiveStatEntryDefinition>,
    minimum_visibility_threshold: i32,
    allow_overcompletion: bool,
    show_value_on_complete: bool,
    completed_value_style: i32,
    in_progress_value_style: i32,
    ui_label: Option<String>,
    ui_style: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyObjectiveDisplayProperties
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyObjectiveDisplayProperties {
    activity_hash: Option<u32>,
    display_on_item_preview_screen: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyObjectivePerkEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyObjectivePerkEntryDefinition {
    perk_hash: u32,
    style: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyObjectiveStatEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyObjectiveStatEntryDefinition {
    stat: Option<DestinyItemInvestmentStatDefinition>,
    style: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyParentItemOverride
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyParentItemOverride {
    additional_equip_requirements_display_strings: Vec<String>,
    pip_icon: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyPlaceDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlaceDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyPlugItemCraftingRequirements
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlugItemCraftingRequirements {
    unlock_requirements: Vec<DestinyPlugItemCraftingUnlockRequirement>,
    required_level: Option<i32>,
    material_requirement_hashes: Vec<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyPlugItemCraftingUnlockRequirement
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlugItemCraftingUnlockRequirement {
    failure_description: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyPlugRuleDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlugRuleDefinition {
    failure_message: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinyPlugSetDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlugSetDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    reusable_plug_items: Option<Vec<DestinyItemSocketEntryPlugItemRandomizedDefinition>>,
    is_fake_plug_set: bool,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinyPlugWhitelistEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlugWhitelistEntryDefinition {
    category_hash: u32,
    category_identifier: String,
    reinitialization_possible_plug_hashes: Option<Vec<u32>>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Common.DestinyPositionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPositionDefinition {
    x: i32,
    y: i32,
    z: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.PowerCaps.DestinyPowerCapDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPowerCapDefinition {
    power_cap: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationChildBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationChildBlock {
    presentation_node_type: i32,
    parent_presentation_node_hashes: Vec<u32>,
    display_style: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeChildrenBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationNodeChildrenBlock {
    presentation_nodes: Vec<DestinyPresentationNodeChildEntry>,
    collectibles: Vec<DestinyPresentationNodeCollectibleChildEntry>,
    records: Vec<DestinyPresentationNodeRecordChildEntry>,
    metrics: Vec<DestinyPresentationNodeMetricChildEntry>,
    craftables: Vec<DestinyPresentationNodeCraftableChildEntry>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeChildEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationNodeChildEntry {
    presentation_node_hash: u32,
    node_display_priority: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeCollectibleChildEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationNodeCollectibleChildEntry {
    collectible_hash: u32,
    node_display_priority: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeCraftableChildEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationNodeCraftableChildEntry {
    craftable_item_hash: u32,
    node_display_priority: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeMetricChildEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationNodeMetricChildEntry {
    metric_hash: u32,
    node_display_priority: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeRecordChildEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationNodeRecordChildEntry {
    record_hash: u32,
    node_display_priority: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationNodeDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    original_icon: Option<String>,
    root_view_icon: Option<String>,
    node_type: i32,
    scope: i32,
    objective_hash: Option<u32>,
    completion_record_hash: Option<u32>,
    children: DestinyPresentationNodeChildrenBlock,
    display_style: i32,
    screen_style: i32,
    requirements: DestinyPresentationNodeRequirementsBlock,
    disable_child_subscreen_navigation: bool,
    max_category_record_score: i32,
    presentation_node_type: i32,
    trait_ids: Vec<String>,
    trait_hashes: Vec<u32>,
    parent_node_hashes: Vec<u32>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeRequirementsBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPresentationNodeRequirementsBlock {
    entitlement_unavailable_message: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyProgressionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProgressionDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    scope: i32,
    repeat_last_step: bool,
    source: Option<String>,
    steps: Option<Vec<DestinyProgressionStepDefinition>>,
    visible: bool,
    faction_hash: Option<u32>,
    color: Option<DestinyColor>,
    rank_icon: Option<String>,
    reward_items: Option<Vec<DestinyProgressionRewardItemQuantity>>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Progression.DestinyProgressionLevelRequirementDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProgressionLevelRequirementDefinition {
    requirement_curve: Vec<InterpolationPointFloat>,
    progression_hash: u32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyProgressionMappingDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProgressionMappingDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    display_units: Option<String>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyProgressionRewardDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProgressionRewardDefinition {
    progression_mapping_hash: u32,
    amount: i32,
    apply_throttles: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyProgressionRewardItemQuantity
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProgressionRewardItemQuantity {
    rewarded_at_progression_level: i32,
    acquisition_behavior: i32,
    ui_display_style: String,
    claim_unlock_display_strings: Vec<String>,
    item_hash: u32,
    item_instance_id: Option<i64>,
    quantity: i32,
    has_conditional_visibility: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyProgressionStepDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProgressionStepDefinition {
    step_name: String,
    display_effect_type: i32,
    progress_total: i32,
    reward_items: Option<Vec<DestinyItemQuantity>>,
    icon: Option<String>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.DestinyRace
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum DestinyRace {
    Human = 0,
    Awoken = 1,
    Exo = 2,
    Unknown = 3,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyRaceDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRaceDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    race_type: i32,
    gendered_race_names: HashMap<DestinyGender, String>,
    gendered_race_names_by_gender_hash: HashMap<u32, String>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.DestinyRecordDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRecordDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    scope: i32,
    presentation_info: Option<DestinyPresentationChildBlock>,
    lore_hash: Option<u32>,
    objective_hashes: Vec<u32>,
    record_value_style: i32,
    for_title_gilding: bool,
    should_show_large_icons: bool,
    title_info: DestinyRecordTitleBlock,
    completion_info: DestinyRecordCompletionBlock,
    state_info: SchemaRecordStateBlock,
    requirements: DestinyPresentationNodeRequirementsBlock,
    expiration_info: DestinyRecordExpirationBlock,
    interval_info: DestinyRecordIntervalBlock,
    reward_items: Vec<DestinyItemQuantity>,
    presentation_node_type: i32,
    trait_ids: Vec<String>,
    trait_hashes: Vec<u32>,
    parent_node_hashes: Vec<u32>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.DestinyRecordCompletionBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRecordCompletionBlock {
    partial_completion_objective_count_threshold: i32,
    score_value: Option<i32>,
    should_fire_toast: bool,
    toast_style: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.DestinyRecordExpirationBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRecordExpirationBlock {
    has_expiration: bool,
    description: String,
    icon: Option<String>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.DestinyRecordIntervalBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRecordIntervalBlock {
    interval_objectives: Vec<DestinyRecordIntervalObjective>,
    interval_rewards: Vec<DestinyRecordIntervalRewards>,
    original_objective_array_insertion_index: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.DestinyRecordIntervalObjective
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRecordIntervalObjective {
    interval_objective_hash: u32,
    interval_score_value: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.DestinyRecordIntervalRewards
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRecordIntervalRewards {
    interval_reward_items: Vec<DestinyItemQuantity>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.DestinyRecordTitleBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRecordTitleBlock {
    has_title: bool,
    titles_by_gender: Option<HashMap<DestinyGender, String>>,
    titles_by_gender_hash: Option<HashMap<u32, String>>,
    gilding_tracking_record_hash: Option<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Reporting.DestinyReportReasonCategoryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyReportReasonCategoryDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    reasons: HashMap<u32, DestinyReportReasonDefinition>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Reporting.DestinyReportReasonDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyReportReasonDefinition {
    reason_hash: u32,
    display_properties: DestinyDisplayPropertiesDefinition,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyRewardSourceDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRewardSourceDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    category: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinySandboxPatternDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySandboxPatternDefinition {
    pattern_hash: u32,
    pattern_global_tag_id_hash: u32,
    weapon_content_group_hash: u32,
    weapon_translation_group_hash: u32,
    weapon_type_hash: Option<u32>,
    weapon_type: i32,
    filters: Option<Vec<DestinyArrangementRegionFilterDefinition>>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinySandboxPerkDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySandboxPerkDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    perk_identifier: Option<String>,
    is_displayable: bool,
    damage_type: i32,
    damage_type_hash: Option<u32>,
    perk_groups: Option<DestinyTalentNodeStepGroups>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Seasons.DestinySeasonDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySeasonDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    background_image_path: Option<String>,
    season_number: i32,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    season_pass_hash: Option<u32>,
    season_pass_progression_hash: u32,
    artifact_item_hash: Option<u32>,
    seal_presentation_node_hash: Option<u32>,
    seasonal_challenges_presentation_node_hash: Option<u32>,
    preview: Option<DestinySeasonPreviewDefinition>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Seasons.DestinySeasonPassDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySeasonPassDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    reward_progression_hash: u32,
    prestige_progression_hash: u32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Seasons.DestinySeasonPreviewDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySeasonPreviewDefinition {
    description: String,
    link_path: String,
    video_link: Option<String>,
    images: Vec<DestinySeasonPreviewImageDefinition>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Seasons.DestinySeasonPreviewImageDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySeasonPreviewImageDefinition {
    thumbnail_image: String,
    high_res_image: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinySocketCategoryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySocketCategoryDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    ui_category_style: u32,
    category_style: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinySocketTypeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySocketTypeDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    insert_action: Option<DestinyInsertPlugActionDefinition>,
    plug_whitelist: Option<Vec<DestinyPlugWhitelistEntryDefinition>>,
    socket_category_hash: u32,
    visibility: i32,
    always_randomize_sockets: bool,
    is_preview_enabled: bool,
    hide_duplicate_reusable_plugs: bool,
    overrides_ui_appearance: bool,
    avoid_duplicates_on_initialization: bool,
    currency_scalars: Option<Vec<DestinySocketTypeScalarMaterialRequirementEntry>>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinySocketTypeScalarMaterialRequirementEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySocketTypeScalarMaterialRequirementEntry {
    currency_item_hash: u32,
    scalar_value: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyStatDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyStatDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    aggregation_type: i32,
    has_computed_block: bool,
    stat_category: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyStatDisplayDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyStatDisplayDefinition {
    stat_hash: u32,
    maximum_value: i32,
    display_as_numeric: bool,
    display_interpolation: Vec<InterpolationPoint>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyStatGroupDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyStatGroupDefinition {
    maximum_value: i32,
    ui_position: i32,
    scaled_stats: Vec<DestinyStatDisplayDefinition>,
    overrides: HashMap<u32, DestinyStatOverrideDefinition>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyStatOverrideDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyStatOverrideDefinition {
    stat_hash: u32,
    display_properties: DestinyDisplayPropertiesDefinition,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyTalentExclusiveGroup
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyTalentExclusiveGroup {
    group_hash: u32,
    lore_hash: Option<u32>,
    node_hashes: Vec<u32>,
    opposing_group_hashes: Vec<u32>,
    opposing_node_hashes: Vec<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyTalentGridDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyTalentGridDefinition {
    max_grid_level: i32,
    grid_level_per_column: i32,
    progression_hash: u32,
    nodes: Vec<DestinyTalentNodeDefinition>,
    exclusive_sets: Vec<DestinyTalentNodeExclusiveSetDefinition>,
    independent_node_indexes: Vec<i32>,
    groups: HashMap<u32, DestinyTalentExclusiveGroup>,
    node_categories: Vec<DestinyTalentNodeCategory>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyTalentNodeCategory
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyTalentNodeCategory {
    identifier: String,
    is_lore_driven: bool,
    display_properties: DestinyDisplayPropertiesDefinition,
    node_hashes: Vec<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyTalentNodeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyTalentNodeDefinition {
    node_index: i32,
    node_hash: u32,
    row: i32,
    column: i32,
    prerequisite_node_indexes: Vec<i32>,
    binary_pair_node_index: i32,
    auto_unlocks: bool,
    last_step_repeats: bool,
    is_random: bool,
    random_activation_requirement: Option<DestinyNodeActivationRequirement>,
    is_random_repurchasable: bool,
    steps: Vec<DestinyNodeStepDefinition>,
    exclusive_with_node_hashes: Vec<u32>,
    random_start_progression_bar_at_progression: i32,
    layout_identifier: String,
    group_hash: Option<u32>,
    lore_hash: Option<u32>,
    node_style_identifier: String,
    ignore_for_completion: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyTalentNodeExclusiveSetDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyTalentNodeExclusiveSetDefinition {
    node_indexes: Vec<i32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyTalentNodeStepGroups
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyTalentNodeStepGroups {
    weapon_performance: i32,
    impact_effects: i32,
    guardian_attributes: i32,
    light_abilities: i32,
    damage_types: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Traits.DestinyTraitDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyTraitDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    trait_category_id: Option<String>,
    trait_category_hash: u32,
    display_hint: String,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Traits.DestinyTraitCategoryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyTraitCategoryDefinition {
    trait_category_id: String,
    trait_hashes: Vec<u32>,
    trait_ids: Vec<String>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyUnlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyUnlockDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyUnlockExpressionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyUnlockExpressionDefinition {
    scope: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyUnlockValueDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyUnlockValueDefinition {
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorAcceptedItemDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorAcceptedItemDefinition {
    accepted_inventory_bucket_hash: u32,
    destination_inventory_bucket_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorActionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorActionDefinition {
    description: String,
    execute_seconds: i32,
    icon: Option<String>,
    name: String,
    verb: String,
    is_positive: bool,
    action_id: String,
    action_hash: u32,
    auto_perform_action: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorCategoryEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorCategoryEntryDefinition {
    category_index: i32,
    sort_value: i32,
    category_hash: u32,
    quantity_available: i32,
    show_unavailable_items: bool,
    hide_if_no_currency: bool,
    hide_from_regular_purchase: bool,
    buy_string_override: String,
    disabled_description: String,
    display_title: Option<String>,
    overlay: Option<DestinyVendorCategoryOverlayDefinition>,
    vendor_item_indexes: Vec<i32>,
    is_preview: bool,
    is_display_only: bool,
    reset_interval_minutes_override: i32,
    reset_offset_minutes_override: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorCategoryOverlayDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorCategoryOverlayDefinition {
    choice_description: String,
    description: String,
    icon: String,
    title: String,
    currency_item_hash: Option<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorDefinition {
    display_properties: DestinyDisplayPropertiesDefinition,
    vendor_progression_type: i32,
    buy_string: Option<String>,
    sell_string: Option<String>,
    display_item_hash: u32,
    inhibit_buying: bool,
    inhibit_selling: bool,
    faction_hash: u32,
    reset_interval_minutes: i32,
    reset_offset_minutes: i32,
    failure_strings: Option<Vec<String>>,
    unlock_ranges: Option<Vec<DateRange>>,
    vendor_identifier: Option<String>,
    vendor_portrait: Option<String>,
    vendor_banner: Option<String>,
    enabled: bool,
    visible: bool,
    vendor_subcategory_identifier: Option<String>,
    consolidate_categories: bool,
    actions: Option<Vec<DestinyVendorActionDefinition>>,
    categories: Option<Vec<DestinyVendorCategoryEntryDefinition>>,
    original_categories: Option<Vec<DestinyVendorCategoryEntryDefinition>>,
    display_categories: Option<Vec<DestinyDisplayCategoryDefinition>>,
    interactions: Option<Vec<DestinyVendorInteractionDefinition>>,
    inventory_flyouts: Option<Vec<DestinyVendorInventoryFlyoutDefinition>>,
    item_list: Option<Vec<DestinyVendorItemDefinition>>,
    services: Option<Vec<DestinyVendorServiceDefinition>>,
    accepted_items: Option<Vec<DestinyVendorAcceptedItemDefinition>>,
    return_with_vendor_request: bool,
    locations: Option<Vec<DestinyVendorLocationDefinition>>,
    groups: Option<Vec<DestinyVendorGroupReference>>,
    ignore_sale_item_hashes: Option<Vec<u32>>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorGroupDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorGroupDefinition {
    order: i32,
    category_name: String,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorGroupReference
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorGroupReference {
    vendor_group_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorInteractionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorInteractionDefinition {
    interaction_index: i32,
    replies: Vec<DestinyVendorInteractionReplyDefinition>,
    vendor_category_index: i32,
    questline_item_hash: u32,
    sack_interaction_list: Vec<DestinyVendorInteractionSackEntryDefinition>,
    ui_interaction_type: u32,
    interaction_type: i32,
    reward_block_label: String,
    reward_vendor_category_index: i32,
    flavor_line_one: String,
    flavor_line_two: String,
    header_display_properties: DestinyDisplayPropertiesDefinition,
    instructions: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorInteractionReplyDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorInteractionReplyDefinition {
    item_rewards_selection: i32,
    reply: String,
    reply_type: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorInteractionSackEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorInteractionSackEntryDefinition {
    sack_type: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorInventoryFlyoutBucketDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorInventoryFlyoutBucketDefinition {
    collapsible: bool,
    inventory_bucket_hash: u32,
    sort_items_by: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorInventoryFlyoutDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorInventoryFlyoutDefinition {
    locked_description: String,
    display_properties: DestinyDisplayPropertiesDefinition,
    buckets: Vec<DestinyVendorInventoryFlyoutBucketDefinition>,
    flyout_id: u32,
    suppress_newness: bool,
    equipment_slot_hash: Option<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorItemDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorItemDefinition {
    vendor_item_index: i32,
    item_hash: u32,
    quantity: i32,
    failure_indexes: Vec<i32>,
    currencies: Vec<DestinyVendorItemQuantity>,
    refund_policy: i32,
    refund_time_limit: i32,
    creation_levels: Vec<DestinyItemCreationEntryLevelDefinition>,
    display_category_index: i32,
    category_index: i32,
    original_category_index: i32,
    minimum_level: i32,
    maximum_level: i32,
    action: DestinyVendorSaleItemActionBlockDefinition,
    display_category: String,
    inventory_bucket_hash: u32,
    visibility_scope: i32,
    purchasable_scope: i32,
    exclusivity: i32,
    is_offer: Option<bool>,
    is_crm: Option<bool>,
    sort_value: i32,
    expiration_tooltip: String,
    redirect_to_sale_indexes: Vec<i32>,
    socket_overrides: Vec<DestinyVendorItemSocketOverride>,
    unpurchasable: Option<bool>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorItemQuantity
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorItemQuantity {
    item_hash: u32,
    item_instance_id: Option<i64>,
    quantity: i32,
    has_conditional_visibility: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorItemSocketOverride
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorItemSocketOverride {
    single_item_hash: Option<u32>,
    randomized_options_count: i32,
    socket_type_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Vendors.DestinyVendorLocationDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorLocationDefinition {
    destination_hash: u32,
    background_image_path: Option<String>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorSaleItemActionBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorSaleItemActionBlockDefinition {
    execute_seconds: f32,
    is_positive: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorServiceDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorServiceDefinition {
    name: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.DyeReference
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DyeReference {
    channel_hash: u32,
    dye_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Config.GearAssetDataBaseDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GearAssetDataBaseDefinition {
    version: i32,
    path: String,
}

/// https://bungie-net.github.io/#/components/schemas/Links.HyperlinkReference
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperlinkReference {
    title: String,
    url: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Config.ImagePyramidEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagePyramidEntry {
    name: String,
    factor: f32,
}

/// https://bungie-net.github.io/#/components/schemas/Interpolation.InterpolationPoint
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InterpolationPoint {
    value: i32,
    weight: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Interpolation.InterpolationPointFloat
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InterpolationPointFloat {
    value: f32,
    weight: f32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.SchemaRecordStateBlock
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaRecordStateBlock {
    featured_priority: i32,
    obscured_string: Option<String>,
}
