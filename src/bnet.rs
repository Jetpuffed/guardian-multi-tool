use std::collections::HashMap;

use chrono::prelude::*;
use serde::Deserialize;

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

/// Where all the deserialized game content lives.
/// 
/// Returned by any function that deserializes the world content paths obtained
/// from the manifest in Bungie's API.
#[derive(Debug, Deserialize)]
pub struct DestinyWorldContent {
    // #[serde(rename = "DestinyAchievementDefinition")]
    // achievement_definition: HashMap<String, DestinyAchievementDefinition>,
    #[serde(rename = "DestinyActivityDefinition")]
    activity_definition: HashMap<String, DestinyActivityDefinition>,
    #[serde(rename = "DestinyActivityGraphDefinition")]
    activity_graph_definition: HashMap<String, DestinyActivityGraphDefinition>,
    // #[serde(rename = "DestinyActivityInteractableDefinition")]
    // activity_interactable_definition: HashMap<String, DestinyActivityGraphDefinition>,
    #[serde(rename = "DestinyActivityModeDefinition")]
    activity_mode_definition: HashMap<String, DestinyActivityModeDefinition>,
    #[serde(rename = "DestinyActivityModifierDefinition")]
    activity_modifier_definition: HashMap<String, DestinyActivityModifierDefinition>,
    #[serde(rename = "DestinyActivityTypeDefinition")]
    activity_type_definition: HashMap<String, DestinyActivityTypeDefinition>,
    // #[serde(rename = "DestinyArtDyeChannelDefinition")]
    // art_dye_channel_definition: HashMap<String, DestinyArtDyeChannelDefinition>,
    #[serde(rename = "DestinyArtDyeReferenceDefinition")]
    art_dye_reference_definition: HashMap<String, DestinyArtDyeReference>,
    #[serde(rename = "DestinyArtifactDefinition")]
    artifact_definition: HashMap<String, DestinyArtifactDefinition>,
    // #[serde(rename = "DestinyBondDefinition")]
    // bond_definition: HashMap<String, DestinyBondDefinition>,
    #[serde(rename = "DestinyBreakerTypeDefinition")]
    breaker_type_definition: HashMap<String, DestinyBreakerTypeDefinition>,
    // #[serde(rename = "DestinyCharacterCustomizationCategoryDefinition")]
    // character_customization_category_definition: HashMap<String, DestinyCharacterCustomizationCategoryDefinition>,
    // #[serde(rename = "DestinyCharacterCustomizationOptionDefinition")]
    // character_customization_option_definition: HashMap<String, DestinyCharacterCustomizationOptionDefinition>,
    #[serde(rename = "DestinyChecklistDefinition")]
    checklist_definition: HashMap<String, DestinyChecklistDefinition>,
    #[serde(rename = "DestinyClassDefinition")]
    class_definition: HashMap<String, DestinyClassDefinition>,
    #[serde(rename = "DestinyCollectibleDefinition")]
    collectible_definition: HashMap<String, DestinyCollectibleDefinition>,
    #[serde(rename = "DestinyDamageTypeDefinition")]
    damage_type_definition: HashMap<String, DestinyDamageTypeDefinition>,
    #[serde(rename = "DestinyDestinationDefinition")]
    destination_definition: HashMap<String, DestinyDestinationDefinition>,
    #[serde(rename = "DestinyEnergyTypeDefinition")]
    energy_type_definition: HashMap<String, DestinyEnergyTypeDefinition>,
    // #[serde(rename = "DestinyEntitlementOfferDefinition")]
    // entitlement_offer_definition: HashMap<String, DestinyEntitlementOfferDefinition>,
    #[serde(rename = "DestinyEquipmentSlotDefinition")]
    equipment_slot_definition: HashMap<String, DestinyEquipmentSlotDefinition>,
    #[serde(rename = "DestinyFactionDefinition")]
    faction_definition: HashMap<String, DestinyFactionDefinition>,
    #[serde(rename = "DestinyGenderDefinition")]
    gender_definition: HashMap<String, DestinyGenderDefinition>,
    #[serde(rename = "DestinyItemCategoryDefinition")]
    item_category_definition: HashMap<String, DestinyItemCategoryDefinition>,
    #[serde(rename = "DestinyItemTierTypeDefinition")]
    item_tier_type_definition: HashMap<String, DestinyItemTierTypeDefinition>,
    #[serde(rename = "DestinyInventoryBucketDefinition")]
    inventory_bucket_definition: HashMap<String, DestinyInventoryBucketDefinition>,
    #[serde(rename = "DestinyInventoryItemDefinition")]
    inventory_item_definition: HashMap<String, DestinyInventoryItemDefinition>,
    // #[serde(rename = "DestinyInventoryItemLiteDefinition")]
    // inventory_item_lite_definition: HashMap<String, DestinyInventoryItemLiteDefinition>,
    #[serde(rename = "DestinyLocationDefinition")]
    location_definition: HashMap<String, DestinyLocationDefinition>,
    #[serde(rename = "DestinyLoreDefinition")]
    lore_definition: HashMap<String, DestinyLoreDefinition>,
    #[serde(rename = "DestinyMaterialRequirementSetDefinition")]
    material_requirement_set_definition: HashMap<String, DestinyMaterialRequirementSetDefinition>,
    #[serde(rename = "DestinyMedalTierDefinition")]
    medal_tier_definition: HashMap<String, DestinyMedalTierDefinition>,
    #[serde(rename = "DestinyMetricDefinition")]
    metric_definition: HashMap<String, DestinyMetricDefinition>,
    #[serde(rename = "DestinyMilestoneDefinition")]
    milestone_definition: HashMap<String, DestinyMilestoneDefinition>,
    // #[serde(rename = "DestinyNodeStepSummaryDefinition")]
    // node_step_summary_definition: HashMap<String, DestinyNodeStepSummaryDefinition>,
    #[serde(rename = "DestinyObjectiveDefinition")]
    objective_definition: HashMap<String, DestinyObjectiveDefinition>,
    #[serde(rename = "DestinyPlaceDefinition")]
    place_definition: HashMap<String, DestinyPlaceDefinition>,
    // #[serde(rename = "DestinyPlatformBucketMappingDefinition")]
    // platform_bucket_mapping_definition: HashMap<String, DestinyPlatformBucketMappingDefinition>,
    #[serde(rename = "DestinyPlugSetDefinition")]
    plug_set_definition: HashMap<String, DestinyPlugSetDefinition>,
    #[serde(rename = "DestinyPowerCapDefinition")]
    power_cap_definition: HashMap<String, DestinyPowerCapDefinition>,
    #[serde(rename = "DestinyPresentationNodeDefinition")]
    presentation_node_definition: HashMap<String, DestinyPresentationNodeDefinition>,
    #[serde(rename = "DestinyProgressionDefinition")]
    progression_definition: HashMap<String, DestinyProgressionDefinition>,
    #[serde(rename = "DestinyProgressionLevelRequirementDefinition")]
    progression_level_requirement_definition: HashMap<String, DestinyProgressionLevelRequirementDefinition>,
    #[serde(rename = "DestinyProgressionMappingDefinition")]
    progression_mapping_definition: HashMap<String, DestinyProgressionMappingDefinition>,
    #[serde(rename = "DestinyRaceDefinition")]
    race_definition: HashMap<String, DestinyRaceDefinition>,
    #[serde(rename = "DestinyRecordDefinition")]
    record_definition: HashMap<String, DestinyRecordDefinition>,
    #[serde(rename = "DestinyReportReasonCategoryDefinition")]
    report_reason_category_definition: HashMap<String, DestinyReportReasonCategoryDefinition>,
    // #[serde(rename = "DestinyRewardAdjusterPointerDefinition")]
    // reward_adjuster_pointer_definition: HashMap<String, DestinyRewardAdjusterPointerDefinition>,
    // #[serde(rename = "DestinyRewardAdjusterProgressionMapDefinition")]
    // reward_adjuster_progression_map_definition: HashMap<String, DestinyRewardAdjusterProgressionMapDefinition>,
    // #[serde(rename = "DestinyRewardItemListDefinition")]
    // reward_item_list_definition: HashMap<String, DestinyRewardItemListDefinition>,
    // #[serde(rename = "DestinyRewardMappingDefinition")]
    // reward_mapping_definition: HashMap<String, DestinyRewardMappingDefinition>,
    // #[serde(rename = "DestinyRewardSheetDefinition")]
    // reward_sheet_definition: HashMap<String, DestinyRewardSheetDefinition>,
    #[serde(rename = "DestinyRewardSourceDefinition")]
    reward_source_definition: HashMap<String, DestinyRewardSourceDefinition>,
    // #[serde(rename = "DestinySackRewardItemListDefinition")]
    // sack_reward_item_list_definition: HashMap<String, DestinySackRewardItemListDefinition>,
    #[serde(rename = "DestinySandboxPatternDefinition")]
    sandbox_pattern_definition: HashMap<String, DestinySandboxPatternDefinition>,
    #[serde(rename = "DestinySandboxPerkDefinition")]
    sandbox_perk_definition: HashMap<String, DestinySandboxPerkDefinition>,
    #[serde(rename = "DestinySeasonDefinition")]
    season_definition: HashMap<String, DestinySeasonDefinition>,
    #[serde(rename = "DestinySeasonPassDefinition")]
    season_pass_definition: HashMap<String, DestinySeasonPassDefinition>,
    #[serde(rename = "DestinySocketCategoryDefinition")]
    socket_category_definition: HashMap<String, DestinySocketCategoryDefinition>,
    #[serde(rename = "DestinySocketTypeDefinition")]
    socket_type_definition: HashMap<String, DestinySocketTypeDefinition>,
    #[serde(rename = "DestinyStatDefinition")]
    stat_definition: HashMap<String, DestinyStatDefinition>,
    #[serde(rename = "DestinyStatGroupDefinition")]
    stat_group_definition: HashMap<String, DestinyStatGroupDefinition>,
    #[serde(rename = "DestinyTalentGridDefinition")]
    talent_grid_definition: HashMap<String, DestinyTalentGridDefinition>,
    #[serde(rename = "DestinyTraitDefinition")]
    trait_definition: HashMap<String, DestinyTraitDefinition>,
    #[serde(rename = "DestinyTraitCategoryDefinition")]
    trait_category_definition: HashMap<String, DestinyTraitCategoryDefinition>,
    // #[serde(rename = "DestinyUnlockCountMappingDefinition")]
    // unlock_count_mapping_definition: HashMap<String, DestinyUnlockCountMappingDefinition>,
    #[serde(rename = "DestinyUnlockDefinition")]
    unlock_definition: HashMap<String, DestinyUnlockDefinition>,
    // #[serde(rename = "DestinyUnlockEventDefinition")]
    // unlock_event_definition: HashMap<String, DestinyUnlockEventDefinition>,
    // #[serde(rename = "DestinyUnlockExpressionMappingDefinition")]
    // unlock_expression_mapping_definition: HashMap<String, DestinyUnlockExpressionMappingDefinition>,
    #[serde(rename = "DestinyUnlockValueDefinition")]
    unlock_value_definition: HashMap<String, DestinyUnlockValueDefinition>,
    #[serde(rename = "DestinyVendorDefinition")]
    vendor_definition: HashMap<String, DestinyVendorDefinition>,
    #[serde(rename = "DestinyVendorGroupDefinition")]
    vendor_group_definition: HashMap<String, DestinyVendorGroupDefinition>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "originalDisplayProperties")]
    original_display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "selectionScreenDisplayProperties")]
    selection_screen_display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "releaseIcon")]
    release_icon: String,
    #[serde(rename = "releaseTime")]
    release_time: i32,
    #[serde(rename = "activityLightLevel")]
    activity_light_level: i32,
    #[serde(rename = "destinationHash")]
    destination_hash: u32,
    #[serde(rename = "placeHash")]
    place_hash: u32,
    #[serde(rename = "activityTypeHash")]
    activity_type_hash: u32,
    tier: i32,
    #[serde(rename = "pgcrImage")]
    pgcr_image: String,
    rewards: Vec<DestinyActivityRewardDefinition>,
    modifiers: Vec<DestinyActivityModifierReferenceDefinition>,
    #[serde(rename = "isPlaylist")]
    is_playlist: bool,
    challenges: Vec<DestinyActivityChallengeDefinition>,
    #[serde(rename = "optionalUnlockStrings")]
    optional_unlock_strings: Vec<DestinyActivityUnlockStringDefinition>,
    #[serde(rename = "playlistItems")]
    playlist_items: Vec<DestinyActivityPlaylistItemDefinition>,
    #[serde(rename = "activityGraphList")]
    activity_graph_list: Vec<DestinyActivityGraphListEntryDefinition>,
    matchmaking: DestinyActivityMatchmakingBlockDefinition,
    #[serde(rename = "guidedGame")]
    guided_game: DestinyActivityGuidedBlockDefinition,
    #[serde(rename = "directActivityModeHash")]
    direct_activity_mode_hash: u32,
    #[serde(rename = "directActivityModeType")]
    direct_activity_mode_type: i32,
    loadouts: Vec<DestinyActivityLoadoutRequirementSet>,
    #[serde(rename = "activityModeHashes")]
    activity_mode_hashes: Vec<u32>,
    #[serde(rename = "activityModeTypes")]
    activity_mode_types: Vec<i32>,
    #[serde(rename = "isPvP")]
    is_pvp: bool,
    #[serde(rename = "insertionPoints")]
    insertion_points: Vec<DestinyActivityInsertionPointDefinition>,
    #[serde(rename = "activityLocationMappings")]
    activity_location_mappings: Vec<DestinyEnvironmentLocationMapping>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityChallengeDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityChallengeDefinition {
    #[serde(rename = "objectiveHash")]
    objective_hash: u32,
    #[serde(rename = "dummyRewards")]
    dummy_rewards: Vec<DestinyItemQuantity>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphArtElementDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityGraphArtElementDefinition {
    position: DestinyPositionDefinition,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphConnectionDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityGraphConnectionDefinition {
    #[serde(rename = "sourceNodeHash")]
    source_node_hash: u32,
    #[serde(rename = "destNodeHash")]
    dest_node_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityGraphDefinition {
    nodes: Vec<DestinyActivityGraphNodeDefinition>,
    #[serde(rename = "artElements")]
    art_elements: Vec<DestinyActivityGraphArtElementDefinition>,
    connections: Vec<DestinyActivityGraphConnectionDefinition>,
    #[serde(rename = "displayObjectives")]
    display_objectives: Vec<DestinyActivityGraphDisplayObjectiveDefinition>,
    #[serde(rename = "displayProgressions")]
    display_progressions: Vec<DestinyActivityGraphDisplayProgressionDefinition>,
    #[serde(rename = "linkedGraphs")]
    linked_graphs: Vec<DestinyLinkedGraphDefinition>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphDisplayObjectiveDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityGraphDisplayObjectiveDefinition {
    id: u32,
    #[serde(rename = "objectiveHash")]
    objective_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphDisplayProgressionDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityGraphDisplayProgressionDefinition {
    id: u32,
    #[serde(rename = "progressionHash")]
    progression_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityGraphListEntryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityGraphListEntryDefinition {
    #[serde(rename = "activityGraphHash")]
    activity_graph_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphNodeActivityDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityGraphNodeActivityDefinition {
    #[serde(rename = "nodeActivityId")]
    node_activity_id: u32,
    #[serde(rename = "activityHash")]
    activity_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphNodeDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityGraphNodeDefinition {
    #[serde(rename = "nodeId")]
    node_id: u32,
    #[serde(rename = "overrideDisplay")]
    override_display: DestinyDisplayPropertiesDefinition,
    position: DestinyPositionDefinition,
    #[serde(rename = "featuringStates")]
    featuring_states: Vec<DestinyActivityGraphNodeFeaturingStateDefinition>,
    activities: Vec<DestinyActivityGraphNodeActivityDefinition>,
    states: Vec<DestinyActivityGraphNodeStateEntry>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphNodeFeaturingStateDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityGraphNodeFeaturingStateDefinition {
    #[serde(rename = "highlightType")]
    highlight_type: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphNodeStateEntry
#[derive(Debug, Deserialize)]
pub struct DestinyActivityGraphNodeStateEntry {
    state: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityGuidedBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityGuidedBlockDefinition {
    #[serde(rename = "guidedMaxLobbySize")]
    guided_max_lobby_size: i32,
    #[serde(rename = "guidedMinLobbySize")]
    guided_min_lobby_size: i32,
    #[serde(rename = "guidedDisbandCount")]
    guided_disband_count: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityInsertionPointDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityInsertionPointDefinition {
    #[serde(rename = "phaseHash")]
    phase_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityLoadoutRequirement
#[derive(Debug, Deserialize)]
pub struct DestinyActivityLoadoutRequirement {
    #[serde(rename = "equipmentSlotHash")]
    equipment_slot_hash: u32,
    #[serde(rename = "allowedEquippedItemHashes")]
    allowed_equipped_item_hashes: Vec<u32>,
    #[serde(rename = "allowedWeaponSubTypes")]
    allowed_weapon_sub_types: Vec<i32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityLoadoutRequirementSet
#[derive(Debug, Deserialize)]
pub struct DestinyActivityLoadoutRequirementSet {
    requirements: Vec<DestinyActivityLoadoutRequirement>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityMatchmakingBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityMatchmakingBlockDefinition {
    #[serde(rename = "isMatchmade")]
    is_matchmade: bool,
    #[serde(rename = "minParty")]
    min_party: i32,
    #[serde(rename = "maxParty")]
    max_party: i32,
    #[serde(rename = "maxPlayers")]
    max_players: i32,
    #[serde(rename = "requiresGuardianOath")]
    requires_guardian_oath: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityModeDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityModeDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "pgcrImage")]
    pgcr_image: String,
    #[serde(rename = "modeType")]
    mode_type: i32,
    #[serde(rename = "activityModeCategory")]
    activity_mode_category: i32,
    #[serde(rename = "isTeamBased")]
    is_team_based: bool,
    #[serde(rename = "isAggregateMode")]
    is_aggregate_mode: bool,
    #[serde(rename = "parentHashes")]
    parent_hashes: Vec<u32>,
    #[serde(rename = "friendlyName")]
    friendly_name: String,
    #[serde(rename = "activityModeMappings")]
    activity_mode_mappings: HashMap<u32, i32>,
    display: bool,
    order: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.ActivityModifiers.DestinyActivityModifierDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityModifierDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "displayInNavMode")]
    display_in_nav_mode: bool,
    #[serde(rename = "displayInActivitySelection")]
    display_in_activity_selection: bool,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityModifierReferenceDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityModifierReferenceDefinition {
    #[serde(rename = "activityModifierHash")]
    activity_modifier_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityPlaylistItemDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityPlaylistItemDefinition {
    #[serde(rename = "activityHash")]
    activity_hash: u32,
    #[serde(rename = "directActivityModeHash")]
    direct_activity_mode_hash: u32,
    #[serde(rename = "directActivityModeType")]
    direct_activity_mode_type: i32,
    #[serde(rename = "activityModeHashes")]
    activity_mode_hashes: Vec<u32>,
    #[serde(rename = "activityModeTypes")]
    activity_mode_types: Vec<i32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityRewardDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityRewardDefinition {
    #[serde(rename = "rewardText")]
    reward_text: String,
    #[serde(rename = "rewardItems")]
    reward_items: Vec<DestinyItemQuantity>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityTypeDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityTypeDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityUnlockStringDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityUnlockStringDefinition {
    #[serde(rename = "displayString")]
    display_string: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Animations.DestinyAnimationReference
#[derive(Debug, Deserialize)]
pub struct DestinyAnimationReference {
    #[serde(rename = "animName")]
    anim_name: String,
    #[serde(rename = "animIdentifier")]
    anim_identifier: String,
    path: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyArtDyeReference
#[derive(Debug, Deserialize)]
pub struct DestinyArtDyeReference {
    #[serde(rename = "artDyeChannelHash")]
    art_dye_channel_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Artifacts.DestinyArtifactDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyArtifactDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "translationBlock")]
    translation_block: DestinyItemTranslationBlockDefinition,
    tiers: Vec<DestinyArtifactTierDefinition>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Artifacts.DestinyArtifactTierDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyArtifactTierDefinition {
    #[serde(rename = "tierHash")]
    tier_hash: u32,
    #[serde(rename = "displayTitle")]
    display_title: String,
    #[serde(rename = "progressRequirementMessage")]
    progress_requirement_message: String,
    items: Vec<DestinyArtifactTierItemDefinition>,
    #[serde(rename = "minimumUnlockPointsUsedRequirement")]
    minimum_unlock_points_used_requirement: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Artifacts.DestinyArtifactTierItemDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyArtifactTierItemDefinition {
    #[serde(rename = "itemHash")]
    item_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.BreakerTypes.DestinyBreakerTypeDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyBreakerTypeDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "enumValue")]
    enum_value: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyBubbleDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyBubbleDefinition {
    hash: u32,
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Checklists.DestinyChecklistDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyChecklistDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "viewActionString")]
    view_action_string: String,
    scope: i32,
    entries: Vec<DestinyChecklistEntryDefinition>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Checklists.DestinyChecklistEntryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyChecklistEntryDefinition {
    hash: u32,
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "destinationHash")]
    destination_hash: u32,
    #[serde(rename = "locationHash")]
    location_hash: u32,
    #[serde(rename = "bubbleHash")]
    bubble_hash: u32,
    #[serde(rename = "activityHash")]
    activity_hash: u32,
    #[serde(rename = "itemHash")]
    item_hash: u32,
    #[serde(rename = "vendorHash")]
    vendor_hash: u32,
    #[serde(rename = "vendorInteractionIndex")]
    vendor_interaction_index: i32,
    scope: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyClassDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyClassDefinition {
    #[serde(rename = "classType")]
    class_type: i32,
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "genderedClassNames")]
    gendered_class_names: HashMap<i32, String>,
    #[serde(rename = "genderedClassNamesByGenderHash")]
    gendered_class_names_by_gender_hash: HashMap<u32, String>,
    #[serde(rename = "mentorVendorHash")]
    mentor_vendor_hash: u32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Collectibles.DestinyCollectibleAcquisitionBlock
#[derive(Debug, Deserialize)]
pub struct DestinyCollectibleAcquisitionBlock {
    #[serde(rename = "acquireMaterialRequirementHash")]
    acquire_material_requirement_hash: u32,
    #[serde(rename = "acquireTimestampUnlockValueHash")]
    acquire_timestamp_unlock_value_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Collectibles.DestinyCollectibleDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyCollectibleDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    scope: i32,
    #[serde(rename = "sourceString")]
    source_string: String,
    #[serde(rename = "sourceHash")]
    source_hash: u32,
    #[serde(rename = "itemHash")]
    item_hash: u32,
    #[serde(rename = "acquisitionInfo")]
    acquisition_info: DestinyCollectibleAcquisitionBlock,
    #[serde(rename = "stateInfo")]
    state_info: DestinyCollectibleStateBlock,
    #[serde(rename = "presentationInfo")]
    presentation_info: DestinyPresentationChildBlock,
    #[serde(rename = "presentationNodeType")]
    presentation_node_type: i32,
    #[serde(rename = "traitIds")]
    trait_ids: Vec<String>,
    #[serde(rename = "traitHashes")]
    trait_hashes: Vec<u32>,
    #[serde(rename = "parentNodeHashes")]
    parent_node_hashes: Vec<u32>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Collectibles.DestinyCollectibleStateBlock
#[derive(Debug, Deserialize)]
pub struct DestinyCollectibleStateBlock {
    #[serde(rename = "obscuredOverrideItemHash")]
    obscured_override_item_hash: u32,
    requirements: DestinyPresentationNodeRequirementsBlock,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Misc.DestinyColor
#[derive(Debug, Deserialize)]
pub struct DestinyColor {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyDamageTypeDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyDamageTypeDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "transparentIconPath")]
    transparent_icon_path: String,
    #[serde(rename = "showIcon")]
    show_icon: bool,
    #[serde(rename = "enumValue")]
    enum_value: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyDerivedItemCategoryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyDerivedItemCategoryDefinition {
    #[serde(rename = "categoryDescription")]
    category_description: String,
    items: Vec<DestinyDerivedItemDefinition>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyDerivedItemDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyDerivedItemDefinition {
    #[serde(rename = "itemHash")]
    item_hash: u32,
    #[serde(rename = "itemName")]
    item_name: String,
    #[serde(rename = "itemDetail")]
    item_detail: String,
    #[serde(rename = "itemDescription")]
    item_description: String,
    #[serde(rename = "iconPath")]
    icon_path: String,
    #[serde(rename = "vendorItemIndex")]
    vendor_item_index: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyDestinationBubbleSettingDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyDestinationBubbleSettingDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyDestinationDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyDestinationDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "placeHash")]
    place_hash: u32,
    #[serde(rename = "defaultFreeroamActivityHash")]
    default_freeroam_activity_hash: u32,
    #[serde(rename = "activityGraphEntries")]
    activity_graph_entries: Vec<DestinyActivityGraphListEntryDefinition>,
    #[serde(rename = "bubbleSettings")]
    bubble_settings: Vec<DestinyDestinationBubbleSettingDefinition>,
    bubbles: Vec<DestinyBubbleDefinition>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyDisplayPropertiesDefinition {
    description: String,
    name: String,
    icon: String,
    #[serde(rename = "iconSequences")]
    icon_sequences: Vec<DestinyIconSequenceDefinition>,
    #[serde(rename = "highResIcon")]
    high_res_icon: String,
    #[serde(rename = "hasIcon")]
    has_icon: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyEnergyCapacityEntry
#[derive(Debug, Deserialize)]
pub struct DestinyEnergyCapacityEntry {
    #[serde(rename = "capacityValue")]
    capacity_value: i32,
    #[serde(rename = "energyTypeHash")]
    energy_type_hash: u32,
    #[serde(rename = "energyType")]
    energy_type: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyEnergyCostEntry
#[derive(Debug, Deserialize)]
pub struct DestinyEnergyCostEntry {
    #[serde(rename = "energyCost")]
    energy_cost: i32,
    #[serde(rename = "energyTypeHash")]
    energy_type_hash: u32,
    #[serde(rename = "energyType")]
    energy_type: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.EnergyTypes.DestinyEnergyTypeDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyEnergyTypeDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "transparentIconPath")]
    transparent_icon_path: String,
    #[serde(rename = "showIcon")]
    show_icon: bool,
    #[serde(rename = "enumValue")]
    enum_value: i32,
    #[serde(rename = "capacityStatHash")]
    capacity_stat_hash: u32,
    #[serde(rename = "costStatHash")]
    cost_stat_hash: u32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Constants.DestinyEnvironmentLocationMapping
#[derive(Debug, Deserialize)]
pub struct DestinyEnvironmentLocationMapping {
    #[serde(rename = "locationHash")]
    location_hash: u32,
    #[serde(rename = "activationSource")]
    activation_source: String,
    #[serde(rename = "itemHash")]
    item_hash: u32,
    #[serde(rename = "objectiveHash")]
    objective_hash: u32,
    #[serde(rename = "activityHash")]
    activity_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyEquipmentSlotDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyEquipmentSlotDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "equipmentCategoryHash")]
    equipment_category_hash: u32,
    #[serde(rename = "bucketTypeHash")]
    bucket_type_hash: u32,
    #[serde(rename = "applyCustomArtDyes")]
    apply_custom_art_dyes: bool,
    #[serde(rename = "artDyeChannels")]
    art_dye_channels: Vec<DestinyArtDyeReference>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyEquippingBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyEquippingBlockDefinition {
    #[serde(rename = "gearsetItemHash")]
    gearset_item_hash: u32,
    #[serde(rename = "uniqueLabel")]
    unique_label: String,
    #[serde(rename = "uniqueLabelHash")]
    unique_label_hash: u32,
    #[serde(rename = "equipmentSlotTypeHash")]
    equipment_slot_type_hash: u32,
    attributes: i32,
    #[serde(rename = "ammoType")]
    ammo_type: i32,
    #[serde(rename = "displayStrings")]
    display_strings: Vec<String>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyFactionDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyFactionDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "progressionHash")]
    progression_hash: u32,
    #[serde(rename = "tokenValues")]
    token_values: HashMap<u32, u32>,
    #[serde(rename = "rewardItemHash")]
    reward_item_hash: u32,
    #[serde(rename = "rewardVendorHash")]
    reward_vendor_hash: u32,
    vendors: Vec<DestinyFactionVendorDefinition>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyFactionVendorDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyFactionVendorDefinition {
    #[serde(rename = "vendorHash")]
    vendor_hash: u32,
    #[serde(rename = "destinationHash")]
    destination_hash: u32,
    #[serde(rename = "backgroundImagePath")]
    background_image_path: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyGearArtArrangementReference
#[derive(Debug, Deserialize)]
pub struct DestinyGearArtArrangementReference {
    #[serde(rename = "classHash")]
    class_hash: u32,
    #[serde(rename = "artArrangementHash")]
    art_arrangement_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyGenderDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyGenderDefinition {
    #[serde(rename = "genderType")]
    gender_type: i32,
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Common.DestinyIconSequenceDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyIconSequenceDefinition {
    frames: Vec<String>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemActionBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemActionBlockDefinition {
    #[serde(rename = "verbName")]
    verb_name: String,
    #[serde(rename = "verbDescription")]
    verb_description: String,
    #[serde(rename = "isPositive")]
    is_positive: bool,
    #[serde(rename = "overlayScreenName")]
    overlay_screen_name: String,
    #[serde(rename = "overlayIcon")]
    overlay_icon: String,
    #[serde(rename = "requiredCooldownSeconds")]
    required_cooldown_seconds: i32,
    #[serde(rename = "requiredItems")]
    required_items: Vec<DestinyItemActionRequiredItemDefinition>,
    #[serde(rename = "progressionRewards")]
    progression_rewards: Vec<DestinyProgressionRewardDefinition>,
    #[serde(rename = "actionTypeLabel")]
    action_type_label: String,
    #[serde(rename = "requiredLocation")]
    required_location: String,
    #[serde(rename = "requiredCooldownHash")]
    required_cooldown_hash: u32,
    #[serde(rename = "deleteOnAction")]
    delete_on_action: bool,
    #[serde(rename = "consumeEntireStack")]
    consume_entire_stack: bool,
    #[serde(rename = "useOnAcquire")]
    use_on_acquire: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemActionRequiredItemDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemActionRequiredItemDefinition {
    count: i32,
    #[serde(rename = "itemHash")]
    item_hash: u32,
    #[serde(rename = "deleteOnAction")]
    delete_on_action: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemCategoryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemCategoryDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    visible: bool,
    deprecated: bool,
    #[serde(rename = "shortTitle")]
    short_title: String,
    #[serde(rename = "itemTypeRegex")]
    item_type_regex: String,
    #[serde(rename = "grantDestinyBreakerType")]
    grant_destiny_breaker_type: i32,
    #[serde(rename = "plugCategoryIdentifier")]
    plug_category_identifier: String,
    #[serde(rename = "itemTypeRegexNot")]
    item_type_regex_not: String,
    #[serde(rename = "originBucketIdentifier")]
    origin_bucket_identifier: String,
    #[serde(rename = "grantDestinyItemType")]
    grant_destiny_item_type: i32,
    #[serde(rename = "grantDestinySubType")]
    grant_destiny_sub_type: i32,
    #[serde(rename = "grantDestinyClass")]
    grant_destiny_class: i32,
    #[serde(rename = "traitId")]
    trait_id: String,
    #[serde(rename = "groupedCategoryHashes")]
    grouped_category_hashes: Vec<u32>,
    #[serde(rename = "parentCategoryHashes")]
    parent_category_hashes: Vec<u32>,
    #[serde(rename = "groupCategoryOnly")]
    group_category_only: bool,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemCraftingBlockBonusPlugDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemCraftingBlockBonusPlugDefinition {
    #[serde(rename = "socketTypeHash")]
    socket_type_hash: u32,
    #[serde(rename = "plugItemHash")]
    plug_item_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemCraftingBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemCraftingBlockDefinition {
    #[serde(rename = "outputItemHash")]
    output_item_hash: u32,
    #[serde(rename = "requiredSocketTypeHashes")]
    required_socket_type_hashes: Vec<u32>,
    #[serde(rename = "failedRequirementStrings")]
    failed_requirement_strings: Vec<String>,
    #[serde(rename = "baseMaterialRequirements")]
    base_material_requirements: u32,
    #[serde(rename = "bonusPlugs")]
    bonus_plugs: Vec<DestinyItemCraftingBlockBonusPlugDefinition>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemGearsetBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemGearsetBlockDefinition {
    #[serde(rename = "trackingValueMax")]
    tracking_value_max: i32,
    #[serde(rename = "itemList")]
    item_list: Vec<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemIntrinsicSocketEntryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemIntrinsicSocketEntryDefinition {
    #[serde(rename = "plugItemHash")]
    plug_item_hash: u32,
    #[serde(rename = "socketTypeHash")]
    socket_type_hash: u32,
    #[serde(rename = "defaultVisible")]
    default_visible: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemInventoryBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemInventoryBlockDefinition {
    #[serde(rename = "stackUniqueLabel")]
    stack_unique_label: String,
    #[serde(rename = "maxStackSize")]
    max_stack_size: i32,
    #[serde(rename = "bucketTypeHash")]
    bucket_type_hash: u32,
    #[serde(rename = "recoveryBucketTypeHash")]
    recovery_bucket_type_hash: u32,
    #[serde(rename = "tierTypeHash")]
    tier_type_hash: u32,
    #[serde(rename = "isInstanceItem")]
    is_instance_item: bool,
    #[serde(rename = "tierTypeName")]
    tier_type_name: String,
    #[serde(rename = "tierType")]
    tier_type: i32,
    #[serde(rename = "expirationTooltip")]
    expiration_tooltip: String,
    #[serde(rename = "expiredInActivityMessage")]
    expired_in_activity_message: String,
    #[serde(rename = "expiredInOrbitMessage")]
    expired_in_orbit_message: String,
    #[serde(rename = "suppressExpirationWhenObjectivesComplete")]
    suppress_expiration_when_objectives_complete: bool,
    #[serde(rename = "recipeItemHash")]
    recipe_item_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemInvestmentStatDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemInvestmentStatDefinition {
    #[serde(rename = "statTypeHash")]
    stat_type_hash: u32,
    value: i32,
    #[serde(rename = "isConditionallyActive")]
    is_conditionally_active: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemMetricBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemMetricBlockDefinition {
    #[serde(rename = "availableMetricCategoryNodeHashes")]
    available_metric_category_node_hashes: Vec<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemObjectiveBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemObjectiveBlockDefinition {
    #[serde(rename = "objectiveHashes")]
    objective_hashes: Vec<u32>,
    #[serde(rename = "displayActivityHashes")]
    display_activity_hashes: Vec<u32>,
    #[serde(rename = "requireFullObjectiveCompletion")]
    require_full_objective_completion: bool,
    #[serde(rename = "questlineItemHash")]
    questline_item_hash: u32,
    narrative: String,
    #[serde(rename = "objectiveVerbName")]
    objective_verb_name: String,
    #[serde(rename = "questTypeIdentifier")]
    quest_type_identifier: String,
    #[serde(rename = "questTypeHash")]
    quest_type_hash: u32,
    #[serde(rename = "perObjectiveDisplayProperties")]
    per_objective_display_properties: Vec<DestinyObjectiveDisplayProperties>,
    #[serde(rename = "displayAsStatTracker")]
    display_as_stat_tracker: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemPerkEntryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemPerkEntryDefinition {
    #[serde(rename = "requirementDisplayString")]
    requirement_display_string: String,
    #[serde(rename = "perkHash")]
    perk_hash: u32,
    #[serde(rename = "perkVisibility")]
    perk_visibility: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyItemPlugDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemPlugDefinition {
    #[serde(rename = "insertionRules")]
    insertion_rules: Vec<DestinyPlugRuleDefinition>,
    #[serde(rename = "plugCategoryIdentifier")]
    plug_category_identifier: String,
    #[serde(rename = "plugCategoryHash")]
    plug_category_hash: u32,
    #[serde(rename = "onActionRecreateSelf")]
    on_action_recreate_self: bool,
    #[serde(rename = "insertionMaterialRequirementHash")]
    insertion_material_requirement_hash: u32,
    #[serde(rename = "previewItemOverrideHash")]
    preview_item_override_hash: u32,
    #[serde(rename = "enabledMaterialRequirementHash")]
    enabled_material_requirement_hash: u32,
    #[serde(rename = "enabledRules")]
    enabled_rules: Vec<DestinyPlugRuleDefinition>,
    #[serde(rename = "uiPlugLabel")]
    ui_plug_label: String,
    #[serde(rename = "plugStyle")]
    plug_style: i32,
    #[serde(rename = "plugAvailability")]
    plug_availability: i32,
    #[serde(rename = "alternateUiPlugLabel")]
    alternate_ui_plug_label: String,
    #[serde(rename = "alternatePlugStyle")]
    alternate_plug_style: i32,
    #[serde(rename = "isDummyPlug")]
    is_dummy_plug: bool,
    #[serde(rename = "parentItemOverride")]
    parent_item_override: DestinyParentItemOverride,
    #[serde(rename = "energyCapacity")]
    energy_capacity: DestinyEnergyCapacityEntry,
    #[serde(rename = "energyCost")]
    energy_cost: DestinyEnergyCostEntry,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemPreviewBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemPreviewBlockDefinition {
    #[serde(rename = "screenStyle")]
    screen_style: String,
    #[serde(rename = "previewVendorHash")]
    preview_vendor_hash: u32,
    #[serde(rename = "artifactHash")]
    artifact_hash: u32,
    #[serde(rename = "previewActionString")]
    preview_action_string: String,
    #[serde(rename = "derivedItemCategories")]
    derived_item_categories: Vec<DestinyDerivedItemCategoryDefinition>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemQualityBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemQualityBlockDefinition {
    #[serde(rename = "itemLevels")]
    item_levels: Vec<i32>,
    #[serde(rename = "qualityLevel")]
    quality_level: i32,
    #[serde(rename = "infusionCategoryName")]
    infusion_category_name: String,
    #[serde(rename = "infusionCategoryHash")]
    infusion_category_hash: u32,
    #[serde(rename = "infusionCategoryHashes")]
    infusion_category_hashes: Vec<u32>,
    #[serde(rename = "progressionLevelRequirementHash")]
    progression_level_requirement: u32,
    #[serde(rename = "currentVersion")]
    current_version: u32,
    versions: Vec<DestinyItemVersionDefinition>,
    #[serde(rename = "displayVersionWatermarkIcons")]
    display_version_watermark_icons: Vec<String>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.DestinyItemQuantity
#[derive(Debug, Deserialize)]
pub struct DestinyItemQuantity {
    #[serde(rename = "itemHash")]
    item_hash: u32,
    #[serde(rename = "itemInstanceId")]
    item_instance_id: i64,
    quantity: i32,
    #[serde(rename = "hasConditionalVisibility")]
    has_conditional_visibility: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSackBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemSackBlockDefinition {
    #[serde(rename = "detailAction")]
    detail_action: String,
    #[serde(rename = "openAction")]
    open_action: String,
    #[serde(rename = "selectItemCount")]
    select_item_count: i32,
    #[serde(rename = "vendorSackType")]
    vendor_sack_type: String,
    #[serde(rename = "openOnAcquire")]
    open_on_acquire: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSetBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemSetBlockDefinition {
    #[serde(rename = "itemList")]
    item_list: Vec<DestinyItemSetBlockEntryDefinition>,
    #[serde(rename = "requireOrderedSetItemAdd")]
    require_ordered_set_item_add: bool,
    #[serde(rename = "setIsFeatured")]
    set_is_featured: bool,
    #[serde(rename = "setType")]
    set_type: String,
    #[serde(rename = "questLineName")]
    quest_line_name: String,
    #[serde(rename = "questLineDescription")]
    quest_line_description: String,
    #[serde(rename = "questStepSummary")]
    quest_step_summary: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSetBlockEntryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemSetBlockEntryDefinition {
    #[serde(rename = "trackingValue")]
    tracking_value: i32,
    #[serde(rename = "itemHash")]
    item_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSocketBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemSocketBlockDefinition {
    detail: String,
    #[serde(rename = "socketEntries")]
    socket_entries: Vec<DestinyItemSocketEntryDefinition>,
    #[serde(rename = "intrinsicSockets")]
    intrinsic_sockets: Vec<DestinyItemIntrinsicSocketEntryDefinition>,
    #[serde(rename = "socketCategories")]
    socket_categories: Vec<DestinyItemSocketCategoryDefinition>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSocketCategoryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemSocketCategoryDefinition {
    #[serde(rename = "socketCategoryHash")]
    socket_category_hash: u32,
    #[serde(rename = "socketIndexes")]
    socket_indexes: Vec<i32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSocketEntryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemSocketEntryDefinition {
    #[serde(rename = "socketTypeHash")]
    socket_type_hash: u32,
    #[serde(rename = "singleInitialItemHash")]
    single_initial_item_hash: u32,
    #[serde(rename = "reusablePlugItems")]
    reusable_plug_items: Vec<DestinyItemSocketEntryPlugItemDefinition>,
    #[serde(rename = "preventInitializationOnVendorPurchase")]
    prevent_initialization_on_vendor_purchase: bool,
    #[serde(rename = "hidePerksInItemTooltip")]
    hide_perks_in_item_tooltip: bool,
    #[serde(rename = "plugSources")]
    plug_sources: i32,
    #[serde(rename = "reusablePlugSetHash")]
    reusable_plug_set_hash: u32,
    #[serde(rename = "randomizedPlugSetHash")]
    randomized_plug_set_hash: u32,
    #[serde(rename = "defaultVisible")]
    default_visible: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSocketEntryPlugItemDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemSocketEntryPlugItemDefinition {
    #[serde(rename = "plugItemHash")]
    plug_item_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSocketEntryPlugItemRandomizedDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemSocketEntryPlugItemRandomizedDefinition {
    #[serde(rename = "craftingRequirements")]
    crafting_requirements: DestinyPlugItemCraftingRequirements,
    #[serde(rename = "currentlyCanRoll")]
    currently_can_roll: bool,
    #[serde(rename = "plugItemHash")]
    plug_item_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSourceBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemSourceBlockDefinition {
    #[serde(rename = "sourceHashes")]
    source_hashes: Vec<u32>,
    sources: Vec<DestinyItemSourceDefinition>,
    exclusive: i32,
    #[serde(rename = "vendorSources")]
    vendor_sources: Vec<DestinyItemVendorSourceReference>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sources.DestinyItemSourceDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemSourceDefinition {
    level: i32,
    #[serde(rename = "minQuality")]
    min_quality: i32,
    #[serde(rename = "maxQuality")]
    max_quality: i32,
    #[serde(rename = "minLevelRequired")]
    min_level_required: i32,
    #[serde(rename = "maxLevelRequired")]
    max_level_required: i32,
    #[serde(rename = "computedStats")]
    computed_stats: HashMap<u32, DestinyInventoryItemStatDefinition>,
    #[serde(rename = "sourceHashes")]
    source_hashes: Vec<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemStatBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemStatBlockDefinition {
    #[serde(rename = "disablePrimaryStatDisplay")]
    disable_primary_stat_display: bool,
    #[serde(rename = "statGroupHash")]
    stat_group_hash: u32,
    stats: HashMap<u32, DestinyInventoryItemStatDefinition>,
    #[serde(rename = "hasDisplayableStats")]
    has_displayable_stats: bool,
    #[serde(rename = "primaryBaseStatHash")]
    primary_base_stat_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSummaryBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemSummaryBlockDefinition {
    #[serde(rename = "sortPriority")]
    sort_priority: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemTalentGridBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemTalentGridBlockDefinition {
    #[serde(rename = "talentGridHash")]
    talent_grid_hash: u32,
    #[serde(rename = "itemDetailString")]
    item_detail_string: String,
    #[serde(rename = "buildName")]
    build_name: String,
    #[serde(rename = "hudDamageType")]
    hud_damage_type: i32,
    #[serde(rename = "hudIcon")]
    hud_icon: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyItemTierTypeDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemTierTypeDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "infusionProcess")]
    infusion_process: DestinyItemTierTypeInfusionBlock,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyItemTierTypeInfusionBlock
#[derive(Debug, Deserialize)]
pub struct DestinyItemTierTypeInfusionBlock {
    #[serde(rename = "baseQualityTransferRatio")]
    base_quality_transfer_ratio: f32,
    #[serde(rename = "minimumQualityIncrement")]
    minimum_quality_increment: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemTooltipNotification
#[derive(Debug, Deserialize)]
pub struct DestinyItemTooltipNotification {
    #[serde(rename = "displayString")]
    display_string: String,
    #[serde(rename = "displayStyle")]
    display_style: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemTranslationBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemTranslationBlockDefinition {
    #[serde(rename = "weaponPatternIdentifier")]
    weapon_pattern_identifier: String,
    #[serde(rename = "weaponPatternHash")]
    weapon_pattern_hash: u32,
    #[serde(rename = "defaultDyes")]
    default_dyes: Vec<DyeReference>,
    #[serde(rename = "lockedDyes")]
    locked_dyes: Vec<DyeReference>,
    #[serde(rename = "customDyes")]
    custom_dyes: Vec<DyeReference>,
    arrangements: Vec<DestinyGearArtArrangementReference>,
    #[serde(rename = "hasGeometry")]
    has_geometry: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemValueBlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemValueBlockDefinition {
    #[serde(rename = "itemValue")]
    item_value: Vec<DestinyItemQuantity>,
    #[serde(rename = "valueDescription")]
    value_description: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemVendorSourceReference
#[derive(Debug, Deserialize)]
pub struct DestinyItemVendorSourceReference {
    #[serde(rename = "vendorHash")]
    vendor_hash: u32,
    #[serde(rename = "vendorItemIndexes")]
    vendor_item_indexes: Vec<i32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemVersionDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyItemVersionDefinition {
    #[serde(rename = "powerCapHash")]
    power_cap_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyInventoryBucketDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyInventoryBucketDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    scope: i32,
    category: i32,
    #[serde(rename = "bucketOrder")]
    bucket_order: i32,
    #[serde(rename = "itemCount")]
    item_count: i32,
    location: i32,
    #[serde(rename = "hasTransferDestination")]
    has_transfer_destination: bool,
    enabled: bool,
    fifo: bool,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyInventoryItemDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyInventoryItemDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "tooltipNotifications")]
    tooltip_notifications: Vec<DestinyItemTooltipNotification>,
    #[serde(rename = "collectibleHash")]
    collectible_hash: u32,
    #[serde(rename = "iconWatermark")]
    icon_watermark: String,
    #[serde(rename = "iconWatermarkedShelved")]
    icon_watermark_shelved: String,
    #[serde(rename = "secondaryIcon")]
    secondary_icon: String,
    #[serde(rename = "secondaryOverlay")]
    secondary_overlay: String,
    #[serde(rename = "secondarySpecial")]
    secondary_special: String,
    #[serde(rename = "backgroundColor")]
    background_color: DestinyColor,
    screenshot: String,
    #[serde(rename = "itemTypeDisplayName")]
    item_type_display_name: String,
    #[serde(rename = "flavorText")]
    flavor_text: String,
    #[serde(rename = "uiItemDisplayStyle")]
    ui_item_display_style: String,
    #[serde(rename = "itemTypeAndTierDisplayName")]
    item_type_and_tier_display_name: String,
    #[serde(rename = "displaySource")]
    display_source: String,
    #[serde(rename = "tooltipStyle")]
    tooltip_style: String,
    action: DestinyItemActionBlockDefinition,
    crafting: DestinyItemCraftingBlockDefinition,
    inventory: DestinyItemInventoryBlockDefinition,
    #[serde(rename = "setData")]
    set_data: DestinyItemSetBlockDefinition,
    stats: DestinyItemStatBlockDefinition,
    #[serde(rename = "emblemObjectiveHash")]
    emblem_objective_hash: u32,
    #[serde(rename = "equippingBlock")]
    equipping_block: DestinyEquippingBlockDefinition,
    #[serde(rename = "translationBlock")]
    translation_block: DestinyItemTranslationBlockDefinition,
    preview: DestinyItemPreviewBlockDefinition,
    quality: DestinyItemQualityBlockDefinition,
    value: DestinyItemValueBlockDefinition,
    #[serde(rename = "sourceData")]
    source_data: DestinyItemSourceBlockDefinition,
    objectives: DestinyItemObjectiveBlockDefinition,
    metrics: DestinyItemMetricBlockDefinition,
    plug: DestinyItemPlugDefinition,
    gearset: DestinyItemGearsetBlockDefinition,
    sack: DestinyItemSackBlockDefinition,
    sockets: DestinyItemSocketBlockDefinition,
    summary: DestinyItemSummaryBlockDefinition,
    #[serde(rename = "talentGrid")]
    talent_grid: DestinyItemTalentGridBlockDefinition,
    #[serde(rename = "investmentStats")]
    investment_stats: Vec<DestinyItemInvestmentStatDefinition>,
    perks: Vec<DestinyItemPerkEntryDefinition>,
    #[serde(rename = "loreHash")]
    lore_hash: u32,
    #[serde(rename = "summaryItemHash")]
    summary_item_hash: u32,
    animations: Vec<DestinyAnimationReference>,
    #[serde(rename = "allowActions")]
    allow_actions: bool,
    links: Vec<HyperlinkReference>,
    #[serde(rename = "doesPostmasterPullHaveSideEffects")]
    does_postmaster_pull_have_side_effects: bool,
    #[serde(rename = "nonTransferrable")]
    non_transferrable: bool,
    #[serde(rename = "itemCategoryHashes")]
    item_category_hashes: Vec<u32>,
    #[serde(rename = "specialItemType")]
    special_item_type: i32,
    #[serde(rename = "itemType")]
    item_type: i32,
    #[serde(rename = "itemSubType")]
    item_sub_type: i32,
    #[serde(rename = "classType")]
    class_type: i32,
    #[serde(rename = "breakerType")]
    breaker_type: i32,
    #[serde(rename = "breakerTypeHash")]
    breaker_type_hash: u32,
    equippable: bool,
    #[serde(rename = "damageTypeHashes")]
    damage_type_hashes: Vec<u32>,
    #[serde(rename = "damageTypes")]
    damage_types: Vec<i32>,
    #[serde(rename = "defaultDamageType")]
    default_damage_type: i32,
    #[serde(rename = "defaultDamageTypeHash")]
    default_damage_type_hash: u32,
    #[serde(rename = "seasonHash")]
    season_hash: u32,
    #[serde(rename = "isWrapper")]
    is_wrapper: bool,
    #[serde(rename = "traitIds")]
    traid_ids: Vec<String>,
    #[serde(rename = "traitHashes")]
    trait_hashes: Vec<u32>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyInventoryItemStatDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyInventoryItemStatDefinition {
    #[serde(rename = "statHash")]
    stat_hash: u32,
    value: i32,
    minimum: i32,
    maximum: i32,
    #[serde(rename = "displayMaximum")]
    display_maximum: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyLinkedGraphDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyLinkedGraphDefinition {
    description: String,
    name: String,
    #[serde(rename = "unlockExpression")]
    unlock_expression: DestinyUnlockExpressionDefinition,
    #[serde(rename = "linkedGraphId")]
    linked_graph_id: u32,
    #[serde(rename = "linkedGraphs")]
    linked_graphs: Vec<DestinyLinkedGraphEntryDefinition>,
    overview: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyLinkedGraphEntryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyLinkedGraphEntryDefinition {
    #[serde(rename = "activityGraphHash")]
    activity_graph_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyLocationDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyLocationDefinition {
    #[serde(rename = "vendorHash")]
    vendor_hash: u32,
    #[serde(rename = "locationReleases")]
    location_releases: Vec<DestinyLocationReleaseDefinition>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyLocationReleaseDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyLocationReleaseDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "smallTransparentIcon")]
    small_transparent_icon: String,
    #[serde(rename = "mapIcon")]
    map_icon: String,
    #[serde(rename = "largeTransparentIcon")]
    large_transparent_icon: String,
    #[serde(rename = "spawnPoint")]
    spawn_point: u32,
    #[serde(rename = "destinationHash")]
    destination_hash: u32,
    #[serde(rename = "activityHash")]
    activity_hash: u32,
    #[serde(rename = "activityGraphHash")]
    activity_graph_hash: u32,
    #[serde(rename = "activityGraphNodeHash")]
    activity_graph_node_hash: u32,
    #[serde(rename = "activityBubbleName")]
    activity_bubble_name: u32,
    #[serde(rename = "activityPathBundle")]
    activity_path_bundle: u32,
    #[serde(rename = "activityPathDestination")]
    activity_path_destination: u32,
    #[serde(rename = "navPointType")]
    nav_point_type: i32,
    #[serde(rename = "worldPosition")]
    world_position: Vec<i32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Lore.DestinyLoreDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyLoreDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    subtitle: String,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyMaterialRequirement
#[derive(Debug, Deserialize)]
pub struct DestinyMaterialRequirement {
    #[serde(rename = "itemHash")]
    item_hash: u32,
    #[serde(rename = "deleteOnAction")]
    delete_on_action: bool,
    count: i32,
    #[serde(rename = "countIsConstant")]
    count_is_constant: bool,
    #[serde(rename = "omitFromRequirements")]
    omit_from_requirements: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyMaterialRequirementSetDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyMaterialRequirementSetDefinition {
    materials: Vec<DestinyMaterialRequirement>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyMedalTierDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyMedalTierDefinition {
    #[serde(rename = "tierName")]
    tier_name: String,
    order: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Metrics.DestinyMetricDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyMetricDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "trackingObjectiveHash")]
    tracking_objective_hash: u32,
    #[serde(rename = "lowerValueIsBetter")]
    lower_value_is_better: bool,
    #[serde(rename = "presentationNodeType")]
    presentation_node_type: i32,
    #[serde(rename = "traitIds")]
    trait_ids: Vec<String>,
    #[serde(rename = "traitHashes")]
    trait_hashes: Vec<u32>,
    #[serde(rename = "parentNodeHashes")]
    parent_node_hashes: Vec<u32>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneActivityDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneActivityDefinition {
    #[serde(rename = "conceptualActivityHash")]
    conceptual_activity_hash: u32,
    variants: HashMap<u32, DestinyMilestoneActivityVariantDefinition>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneActivityVariantDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneActivityVariantDefinition {
    #[serde(rename = "activityHash")]
    activity_hash: u32,
    order: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneChallengeActivityDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneChallengeActivityDefinition {
    #[serde(rename = "activityHash")]
    activity_hash: u32,
    challenges: Vec<DestinyMilestoneChallengeDefinition>,
    #[serde(rename = "activityGraphNodes")]
    activity_graph_nodes: Vec<DestinyMilestoneChallengeActivityGraphNodeEntry>,
    phases: Vec<DestinyMilestoneChallengeActivityPhase>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneChallengeActivityGraphNodeEntry
#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneChallengeActivityGraphNodeEntry {
    #[serde(rename = "activityGraphHash")]
    activity_graph_hash: u32,
    #[serde(rename = "activityGraphNodeHash")]
    activity_graph_node_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneChallengeActivityPhase
#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneChallengeActivityPhase {
    #[serde(rename = "phaseHash")]
    phase_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneChallengeDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneChallengeDefinition {
    #[serde(rename = "challengeObjectiveHash")]
    challenge_objective_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "displayPreference")]
    display_preference: i32,
    image: String,
    #[serde(rename = "milestoneType")]
    milestone_type: i32,
    recruitable: bool,
    #[serde(rename = "friendlyName")]
    friendly_name: String,
    #[serde(rename = "showInExplorer")]
    show_in_explorer: bool,
    #[serde(rename = "showInMilestones")]
    show_in_milestones: bool,
    #[serde(rename = "explorePrioritizesActivityImage")]
    explore_prioritizes_activity_image: bool,
    #[serde(rename = "hasPredictableDates")]
    has_predictable_dates: bool,
    quests: HashMap<u32, DestinyMilestoneQuestDefinition>,
    rewards: HashMap<u32, DestinyMilestoneRewardCategoryDefinition>,
    #[serde(rename = "vendorsDisplayTitle")]
    vendors_display_title: String,
    vendors: Vec<DestinyMilestoneVendorDefinition>,
    values: HashMap<String, DestinyMilestoneValueDefinition>,
    #[serde(rename = "isInGameMilestone")]
    is_in_game_milestone: bool,
    activities: Vec<DestinyMilestoneChallengeActivityDefinition>,
    #[serde(rename = "defaultOrder")]
    default_order: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneQuestDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneQuestDefinition {
    #[serde(rename = "questItemHash")]
    quest_item_hash: u32,
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "overrideImage")]
    override_image: String,
    #[serde(rename = "questRewards")]
    quest_rewards: DestinyMilestoneQuestRewardsDefinition,
    activities: HashMap<u32, DestinyMilestoneActivityDefinition>,
    #[serde(rename = "destinationHash")]
    destination_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneQuestRewardsDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneQuestRewardsDefinition {
    items: Vec<DestinyMilestoneQuestRewardItem>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneQuestRewardItem
#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneQuestRewardItem {
    #[serde(rename = "vendorHash")]
    vendor_hash: u32,
    #[serde(rename = "vendorItemIndex")]
    vendor_item_index: i32,
    #[serde(rename = "itemHash")]
    item_hash: u32,
    #[serde(rename = "itemInstanceId")]
    item_instance_id: i64,
    quantity: i32,
    #[serde(rename = "hasConditionalVisibility")]
    has_conditional_visibility: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneRewardCategoryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneRewardCategoryDefinition {
    #[serde(rename = "categoryHash")]
    category_hash: u32,
    #[serde(rename = "categoryIdentifier")]
    category_identifier: String,
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "rewardEntries")]
    reward_entries: HashMap<u32, DestinyMilestoneRewardEntryDefinition>,
    order: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneRewardEntryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneRewardEntryDefinition {
    #[serde(rename = "rewardEntryHash")]
    reward_entry_hash: u32,
    #[serde(rename = "rewardEntryIdentifier")]
    reward_entry_identifier: String,
    items: Vec<DestinyItemQuantity>,
    #[serde(rename = "vendorHash")]
    vendor_hash: u32,
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    order: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneValueDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneValueDefinition {
    key: String,
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneVendorDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneVendorDefinition {
    #[serde(rename = "vendorHash")]
    vendor_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyObjectiveDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyObjectiveDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "completionValue")]
    completion_value: i32,
    scope: i32,
    #[serde(rename = "locationHash")]
    location_hash: u32,
    #[serde(rename = "allowNegativeValue")]
    allow_negative_value: bool,
    #[serde(rename = "allowValueChangeWhenCompleted")]
    allow_value_change_when_completed: bool,
    #[serde(rename = "isCountingDownward")]
    is_counting_downward: bool,
    #[serde(rename = "valueStyle")]
    value_style: i32,
    #[serde(rename = "progressDescription")]
    progress_description: String,
    perks: DestinyObjectivePerkEntryDefinition,
    stats: DestinyObjectiveStatEntryDefinition,
    #[serde(rename = "minimumVisibilityThreshold")]
    minimum_visibility_threshold: i32,
    #[serde(rename = "allowOvercompletion")]
    allow_overcompletion: bool,
    #[serde(rename = "showValueOnComplete")]
    show_value_on_complete: bool,
    #[serde(rename = "completedValueStyle")]
    completed_value_style: i32,
    #[serde(rename = "inProgressValueStyle")]
    in_progress_value_style: i32,
    #[serde(rename = "uiLabel")]
    ui_label: String,
    #[serde(rename = "uiStyle")]
    ui_style: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyObjectiveDisplayProperties
#[derive(Debug, Deserialize)]
pub struct DestinyObjectiveDisplayProperties {
    #[serde(rename = "activityHash")]
    activity_hash: u32,
    #[serde(rename = "displayOnItemPreviewScreen")]
    display_on_item_preview_screen: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyObjectivePerkEntryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyObjectivePerkEntryDefinition {
    #[serde(rename = "perkHash")]
    perk_hash: u32,
    style: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyObjectiveStatEntryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyObjectiveStatEntryDefinition {
    stat: DestinyItemInvestmentStatDefinition,
    style: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyParentItemOverride
#[derive(Debug, Deserialize)]
pub struct DestinyParentItemOverride {
    #[serde(rename = "additionalEquipRequirementsDisplayStrings")]
    additional_equip_requirements_display_strings: Vec<String>,
    #[serde(rename = "pipIcon")]
    pip_icon: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyPlaceDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyPlaceDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyPlugItemCraftingRequirements
#[derive(Debug, Deserialize)]
pub struct DestinyPlugItemCraftingRequirements {
    #[serde(rename = "unlockRequirements")]
    unlock_requirements: Vec<DestinyPlugItemCraftingUnlockRequirement>,
    #[serde(rename = "requiredLevel")]
    required_level: i32,
    #[serde(rename = "materialRequirementHashes")]
    material_requirement_hashes: Vec<u32>,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyPlugRuleDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyPlugRuleDefinition {
    #[serde(rename = "failureMessage")]
    failure_message: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinyPlugSetDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyPlugSetDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "reusablePlugItems")]
    reusable_plug_items: Vec<DestinyItemSocketEntryPlugItemRandomizedDefinition>,
    #[serde(rename = "isFakePlugSet")]
    is_fake_plug_set: bool,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Common.DestinyPositionDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyPositionDefinition {
    x: i32,
    y: i32,
    z: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.PowerCaps.DestinyPowerCapDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyPowerCapDefinition {
    #[serde(rename = "powerCap")]
    power_cap: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationChildBlock
#[derive(Debug, Deserialize)]
pub struct DestinyPresentationChildBlock {
    #[serde(rename = "presentationNodeType")]
    presentation_node_type: i32,
    #[serde(rename = "parentPresentationNodeHashes")]
    parent_presentation_node_hashes: Vec<u32>,
    #[serde(rename = "displayStyle")]
    display_style: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyPresentationNodeDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "originalIcon")]
    original_icon: String,
    #[serde(rename = "rootViewIcon")]
    root_view_icon: String,
    #[serde(rename = "nodeType")]
    node_type: i32,
    scope: i32,
    #[serde(rename = "objectiveHash")]
    objective_hash: u32,
    #[serde(rename = "completionRecordHash")]
    completion_record_hash: u32,
    children: DestinyPresentationNodeChildrenBlock,
    #[serde(rename = "displayStyle")]
    display_style: i32,
    #[serde(rename = "screenStyle")]
    screen_style: i32,
    requirements: DestinyPresentationNodeRequirementsBlock,
    #[serde(rename = "disableChildSubscreenNavigation")]
    disable_child_subscreen_navigation: bool,
    #[serde(rename = "maxCategoryRecordScore")]
    max_category_record_score: i32,
    #[serde(rename = "presentationNodeType")]
    presentation_node_type: i32,
    #[serde(rename = "traitIds")]
    trait_ids: Vec<String>,
    #[serde(rename = "traitHashes")]
    trait_hashes: Vec<u32>,
    #[serde(rename = "parentNodeHashes")]
    parent_node_hashes: Vec<u32>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeRequirementsBlock
#[derive(Debug, Deserialize)]
pub struct DestinyPresentationNodeRequirementsBlock {
    #[serde(rename = "entitlementUnavailableMessage")]
    entitlement_unavailable_message: String,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyProgressionDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyProgressionDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    scope: i32,
    #[serde(rename = "repeatLastStep")]
    repeat_last_step: bool,
    source: String,
    steps: Vec<DestinyProgressionStepDefinition>,
    visible: bool,
    #[serde(rename = "factionHash")]
    faction_hash: u32,
    color: DestinyColor,
    #[serde(rename = "rankIcon")]
    rank_icon: String,
    #[serde(rename = "rewardItems")]
    reward_items: Vec<DestinyProgressionRewardItemQuantity>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Progression.DestinyProgressionLevelRequirementDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyProgressionLevelRequirementDefinition {
    #[serde(rename = "requirementCurve")]
    requirement_curve: Vec<InterpolationPointFloat>,
    #[serde(rename = "progressionHash")]
    progression_hash: u32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyProgressionMappingDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyProgressionMappingDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "displayUnits")]
    display_units: String,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyProgressionRewardDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyProgressionRewardDefinition {
    #[serde(rename = "progressionMappingHash")]
    progression_mapping_hash: u32,
    amount: i32,
    #[serde(rename = "applyThrottles")]
    apply_throttles: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyRaceDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyRaceDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "raceType")]
    race_type: i32,
    #[serde(rename = "genderedRaceNames")]
    gendered_race_names: HashMap<i32, String>,
    #[serde(rename = "genderedRaceNamesByGenderHash")]
    gendered_race_names_by_gender_hash: HashMap<u32, String>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.DestinyRecordDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyRecordDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    scope: i32,
    #[serde(rename = "presentationInfo")]
    presentation_info: DestinyPresentationChildBlock,
    #[serde(rename = "loreHash")]
    lore_hash: u32,
    #[serde(rename = "objectiveHashes")]
    objective_hashes: Vec<u32>,
    #[serde(rename = "recordValueStyle")]
    record_value_style: i32,
    #[serde(rename = "forTitleGilding")]
    for_title_gilding: bool,
    #[serde(rename = "shouldShowLargeIcons")]
    should_show_large_icons: bool,
    #[serde(rename = "titleInfo")]
    title_info: DestinyRecordTitleBlock,
    #[serde(rename = "completionInfo")]
    completion_info: DestinyRecordCompletionBlock,
    #[serde(rename = "stateInfo")]
    state_info: SchemaRecordStateBlock,
    requirements: DestinyPresentationNodeRequirementsBlock,
    #[serde(rename = "expirationInfo")]
    expiration_info: DestinyRecordExpirationBlock,
    #[serde(rename = "intervalInfo")]
    interval_info: DestinyRecordIntervalBlock,
    #[serde(rename = "rewardItems")]
    reward_items: Vec<DestinyItemQuantity>,
    #[serde(rename = "presentationNodeType")]
    presentation_node_type: i32,
    #[serde(rename = "traitIds")]
    trait_ids: Vec<String>,
    #[serde(rename = "traitHashes")]
    trait_hashes: Vec<u32>,
    #[serde(rename = "parentNodeHashes")]
    parent_node_hashes: Vec<u32>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Reporting.DestinyReportReasonCategoryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyReportReasonCategoryDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    reasons: HashMap<u32, DestinyReportReasonDefinition>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyRewardSourceDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyRewardSourceDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    category: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinySandboxPatternDefinition
#[derive(Debug, Deserialize)]
pub struct DestinySandboxPatternDefinition {
    #[serde(rename = "patternHash")]
    pattern_hash: u32,
    #[serde(rename = "patternGlobalTagIdHash")]
    pattern_global_tag_id_hash: u32,
    #[serde(rename = "weaponContentGroupHash")]
    weapon_content_group_hash: u32,
    #[serde(rename = "weaponTranslationGroupHash")]
    weapon_translation_group_hash: u32,
    #[serde(rename = "weaponTypeHash")]
    weapon_type_hash: u32,
    #[serde(rename = "weaponType")]
    weapon_type: i32,
    filters: Vec<DestinyArrangementRegionFilterDefinition>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinySandboxPerkDefinition
#[derive(Debug, Deserialize)]
pub struct DestinySandboxPerkDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "perkIdentifier")]
    perk_identifier: String,
    #[serde(rename = "isDisplayable")]
    is_displayable: bool,
    #[serde(rename = "damageType")]
    damage_type: i32,
    #[serde(rename = "damageTypeHash")]
    damage_type_hash: u32,
    #[serde(rename = "perkGroups")]
    perk_groups: DestinyTalentNodeStepGroups,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Seasons.DestinySeasonDefinition
#[derive(Debug, Deserialize)]
pub struct DestinySeasonDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "backgroundImagePath")]
    background_image_path: String,
    #[serde(rename = "seasonNumber")]
    season_number: i32,
    #[serde(rename = "startDate")]
    start_date: DateTime<Utc>,
    #[serde(rename = "endDate")]
    end_date: DateTime<Utc>,
    #[serde(rename = "seasonPassHash")]
    season_pass_hash: u32,
    #[serde(rename = "seasonPassProgressionHash")]
    season_pass_progression_hash: u32,
    #[serde(rename = "artifactItemHash")]
    artifact_item_hash: u32,
    #[serde(rename = "sealPresentationNodeHash")]
    seal_presentation_node_hash: u32,
    #[serde(rename = "seasonalChallengesPresentationNodeHash")]
    seasonal_challenges_presentation_node_hash: u32,
    preview: DestinySeasonPreviewDefinition,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Seasons.DestinySeasonPassDefinition
#[derive(Debug, Deserialize)]
pub struct DestinySeasonPassDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "rewardProgressionHash")]
    reward_progression_hash: u32,
    #[serde(rename = "prestigeProgressionHash")]
    prestige_progression_hash: u32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinySocketCategoryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinySocketCategoryDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "uiCategoryStyle")]
    ui_category_style: u32,
    #[serde(rename = "categoryStyle")]
    category_style: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinySocketTypeDefinition
#[derive(Debug, Deserialize)]
pub struct DestinySocketTypeDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "insertAction")]
    insert_action: DestinyInsertPlugActionDefinition,
    #[serde(rename = "plugWhitelist")]
    plug_whitelist: Vec<DestinyPlugWhitelistEntryDefinition>,
    #[serde(rename = "socketCategoryHash")]
    socket_category_hash: u32,
    visibility: i32,
    #[serde(rename = "alwaysRandomizeSockets")]
    always_randomize_sockets: bool,
    #[serde(rename = "isPreviewEnabled")]
    is_preview_enabled: bool,
    #[serde(rename = "hideDuplicateReusablePlugs")]
    hide_duplicate_reusable_plugs: bool,
    #[serde(rename = "overridesUiAppearance")]
    overrides_ui_appearance: bool,
    #[serde(rename = "avoidDuplicatesOnInitialization")]
    avoid_duplicates_on_initialization: bool,
    #[serde(rename = "currencyScalars")]
    currency_scalars: Vec<DestinySocketTypeScalarMaterialRequirementEntry>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyStatDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyStatDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "aggregationType")]
    aggregation_type: i32,
    #[serde(rename = "hasComputedBlock")]
    has_computed_block: bool,
    #[serde(rename = "statCategory")]
    stat_category: i32,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyStatGroupDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyStatGroupDefinition {
    #[serde(rename = "maximumValue")]
    maximum_value: i32,
    #[serde(rename = "uiPosition")]
    ui_position: i32,
    #[serde(rename = "scaledStats")]
    scaled_stats: Vec<DestinyStatDisplayDefinition>,
    overrides: HashMap<u32, DestinyStatOverrideDefinition>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyTalentGridDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyTalentGridDefinition {
    #[serde(rename = "maxGridLevel")]
    max_grid_level: i32,
    #[serde(rename = "gridLevelPerColumn")]
    grid_level_per_column: i32,
    #[serde(rename = "progressionHash")]
    progression_hash: u32,
    nodes: Vec<DestinyTalentNodeDefinition>,
    #[serde(rename = "exclusiveSets")]
    exclusive_sets: Vec<DestinyTalentNodeExclusiveSetDefinition>,
    #[serde(rename = "independentNodeIndexes")]
    independent_node_indexes: Vec<i32>,
    groups: HashMap<u32, DestinyTalentExclusiveGroup>,
    #[serde(rename = "nodeCategories")]
    node_categories: Vec<DestinyTalentNodeCategory>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Traits.DestinyTraitDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyTraitDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "traitCategoryId")]
    trait_category_id: String,
    #[serde(rename = "traitCategoryHash")]
    trait_category_hash: u32,
    #[serde(rename = "displayHint")]
    display_hint: String,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Traits.DestinyTraitCategoryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyTraitCategoryDefinition {
    #[serde(rename = "traitCategoryId")]
    trait_category_id: String,
    #[serde(rename = "traitHashes")]
    trait_hashes: Vec<u32>,
    #[serde(rename = "traitIds")]
    trait_ids: Vec<String>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyUnlockDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyUnlockDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyUnlockExpressionDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyUnlockExpressionDefinition {
    scope: i32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyUnlockValueDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyUnlockValueDefinition {
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyVendorDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "vendorProgressionType")]
    vendor_progression_type: i32,
    #[serde(rename = "buyString")]
    buy_string: String,
    #[serde(rename = "sellString")]
    sell_string: String,
    #[serde(rename = "displayItemHash")]
    display_item_hash: u32,
    #[serde(rename = "inhibitBuying")]
    inhibit_buying: bool,
    #[serde(rename = "inhibitSelling")]
    inhibit_selling: bool,
    #[serde(rename = "factionHash")]
    faction_hash: u32,
    #[serde(rename = "resetIntervalMinutes")]
    reset_interval_minutes: i32,
    #[serde(rename = "resetOffsetMinutes")]
    reset_offset_minutes: i32,
    #[serde(rename = "failureStrings")]
    failure_strings: Vec<String>,
    #[serde(rename = "unlockRanges")]
    unlock_ranges: Vec<DateRange>,
    #[serde(rename = "vendorIdentifier")]
    vendor_identifier: String,
    #[serde(rename = "vendorPortrait")]
    vendor_portrait: String,
    #[serde(rename = "vendorBanner")]
    vendor_banner: String,
    enabled: bool,
    visible: bool,
    #[serde(rename = "vendorSubcategoryIdentifier")]
    vendor_subcategory_identifier: String,
    #[serde(rename = "consolidateCategories")]
    consolidate_categories: bool,
    actions: Vec<DestinyVendorActionDefinition>,
    categories: Vec<DestinyVendorCategoryEntryDefinition>,
    #[serde(rename = "originalCategories")]
    original_categories: Vec<DestinyVendorCategoryEntryDefinition>,
    #[serde(rename = "displayCategories")]
    display_categories: Vec<DestinyDisplayCategoryDefinition>,
    interactions: Vec<DestinyVendorInteractionDefinition>,
    #[serde(rename = "inventoryFlyouts")]
    inventory_flyouts: Vec<DestinyVendorInventoryFlyoutDefinition>,
    #[serde(rename = "itemList")]
    item_list: Vec<DestinyVendorItemDefinition>,
    services: Vec<DestinyVendorServiceDefinition>,
    #[serde(rename = "acceptedItems")]
    accepted_items: Vec<DestinyVendorAcceptedItemDefinition>,
    #[serde(rename = "returnWithVendorRequest")]
    return_with_vendor_request: bool,
    locations: Vec<DestinyVendorLocationDefinition>,
    groups: Vec<DestinyVendorGroupReference>,
    #[serde(rename = "ignoreSaleItemHashes")]
    ignore_sale_item_hashes: Vec<u32>,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorGroupDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyVendorGroupDefinition {
    order: i32,
    #[serde(rename = "categoryName")]
    category_name: String,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.DyeReference
#[derive(Debug, Deserialize)]
pub struct DyeReference {
    #[serde(rename = "channelHash")]
    channel_hash: u32,
    #[serde(rename = "dyeHash")]
    dye_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Links.HyperlinkReference
#[derive(Debug, Deserialize)]
pub struct HyperlinkReference {
    title: String,
    url: String,
}
