use std::collections::HashMap;

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

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityTypeDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityTypeDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    hash: u32,
    index: i32,
    redacted: bool,
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
    bubble_settings: Vec<DestinyDestinationBubbleSettingsDefinition>,
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

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyFactionDefinition
#[derive(Debug, Deserialize)]
struct DestinyFactionDefinition {}

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

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemCategoryDefinition
#[derive(Debug, Deserialize)]
struct DestinyItemCategoryDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Items.DestinyItemTierTypeDefinition
#[derive(Debug, Deserialize)]
struct DestinyItemTierTypeDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyInventoryBucketDefinition
#[derive(Debug, Deserialize)]
struct DestinyInventoryBucketDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyInventoryItemDefinition
#[derive(Debug, Deserialize)]
struct DestinyInventoryItemDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyLocationDefinition
#[derive(Debug, Deserialize)]
struct DestinyLocationDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Lore.DestinyLoreDefinition
#[derive(Debug, Deserialize)]
struct DestinyLoreDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyMaterialRequirementSetDefinition
#[derive(Debug, Deserialize)]
struct DestinyMaterialRequirementSetDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyMedalTierDefinition
#[derive(Debug, Deserialize)]
struct DestinyMedalTierDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Metrics.DestinyMetricDefinition
#[derive(Debug, Deserialize)]
struct DestinyMetricDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Milestones.DestinyMilestoneDefinition
#[derive(Debug, Deserialize)]
struct DestinyMilestoneDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyObjectiveDefinition
#[derive(Debug, Deserialize)]
struct DestinyObjectiveDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyPlaceDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyPlaceDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    hash: u32,
    index: i32,
    redacted: bool,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinyPlugSetDefinition
#[derive(Debug, Deserialize)]
struct DestinyPlugSetDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.PowerCaps.DestinyPowerCapDefinition
#[derive(Debug, Deserialize)]
struct DestinyPowerCapDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Presentation.DestinyPresentationNodeDefinition
#[derive(Debug, Deserialize)]
struct DestinyPresentationNodeDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyProgressionDefinition
#[derive(Debug, Deserialize)]
struct DestinyProgressionDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Progression.DestinyProgressionLevelRequirementDefinition
#[derive(Debug, Deserialize)]
struct DestinyProgressionLevelRequirementDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyProgressionMappingDefinition
#[derive(Debug, Deserialize)]
struct DestinyProgressionMappingDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyRaceDefinition
#[derive(Debug, Deserialize)]
struct DestinyRaceDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.DestinyRecordDefinition
#[derive(Debug, Deserialize)]
struct DestinyRecordDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Reporting.DestinyReportReasonCategoryDefinition
#[derive(Debug, Deserialize)]
struct DestinyReportReasonCategoryDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyRewardSourceDefinition
#[derive(Debug, Deserialize)]
struct DestinyRewardSourceDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinySandboxPatternDefinition
#[derive(Debug, Deserialize)]
struct DestinySandboxPatternDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinySandboxPerkDefinition
#[derive(Debug, Deserialize)]
struct DestinySandboxPerkDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Seasons.DestinySeasonDefinition
#[derive(Debug, Deserialize)]
struct DestinySeasonDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Seasons.DestinySeasonPassDefinition
#[derive(Debug, Deserialize)]
struct DestinySeasonPassDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinySocketCategoryDefinition
#[derive(Debug, Deserialize)]
struct DestinySocketCategoryDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Sockets.DestinySocketTypeDefinition
#[derive(Debug, Deserialize)]
struct DestinySocketTypeDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyStatDefinition
#[derive(Debug, Deserialize)]
struct DestinyStatDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyStatGroupDefinition
#[derive(Debug, Deserialize)]
struct DestinyStatGroupDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyTalentGridDefinition
#[derive(Debug, Deserialize)]
struct DestinyTalentGridDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Traits.DestinyTraitDefinition
#[derive(Debug, Deserialize)]
struct DestinyTraitDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Traits.DestinyTraitCategoryDefinition
#[derive(Debug, Deserialize)]
struct DestinyTraitCategoryDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyUnlockDefinition
#[derive(Debug, Deserialize)]
struct DestinyUnlockDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyUnlockValueDefinition
#[derive(Debug, Deserialize)]
struct DestinyUnlockValueDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorDefinition
#[derive(Debug, Deserialize)]
struct DestinyVendorDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorGroupDefinition
#[derive(Debug, Deserialize)]
struct DestinyVendorGroupDefinition {}
