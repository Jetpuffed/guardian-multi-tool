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

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityGraphListEntryDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyActivityGraphListEntryDefinition {
    #[serde(rename = "activityGraphHash")]
    activity_graph_hash: u32,
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

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.PowerCaps.DestinyPowerCapDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyPowerCapDefinition {
    #[serde(rename = "powerCap")]
    power_cap: i32,
    hash: u32,
    index: i32,
    redacted: bool,
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
