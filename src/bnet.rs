use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DestinyNodeStepSummaryDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyArtDyeChannelDefinition {}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyArtDyeReference
#[derive(Debug, Deserialize)]
pub struct DestinyArtDyeReferenceDefinition {
    #[serde(rename = "artDyeChannelHash")]
    art_dye_channel_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Common.DestinyIconSequenceDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyIconSequenceDefinition {
    frames: Vec<String>,
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

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyPlaceDefinition
#[derive(Debug, Deserialize)]
pub struct DestinyPlaceDefinition {
    #[serde(rename = "displayProperties")]
    display_properties: DestinyDisplayPropertiesDefinition,
    hash: u32,
    index: i32,
    redacted: bool,
}

#[derive(Debug, Deserialize)]
struct DestinyActivityDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyActivityTypeDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyClassDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyGenderDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyInventoryBucketDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyRaceDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyTalentGridDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyUnlockDefinition {}

#[derive(Debug, Deserialize)]
struct DestinySandboxPerkDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyStatGroupDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyProgressionMappingDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyFactionDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyVendorGroupDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyRewardSourceDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyUnlockValueDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyRewardMappingDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyRewardSheetDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyItemCategoryDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyDamageTypeDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyActivityModeDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyMedalTierDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyAchievementDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyActivityGraphDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyActivityInteractableDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyBondDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyCharacterCustomizationCategoryDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyCharacterCustomizationOptionDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyCollectibleDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyDestinationDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyEntitlementOfferDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyEquipmentSlotDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyStatDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyInventoryItemDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyInventoryItemLiteDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyItemTierTypeDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyLocationDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyLoreDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyMaterialRequirementSetDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyMetricDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyObjectiveDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyPlatformBucketMappingDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyPlugSetDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyPowerCapDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyPresentationNodeDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyProgressionDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyProgressionLevelRequirementDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyRecordDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyRewardAdjusterPointerDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyRewardAdjusterProgressionMapDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyRewardItemListDefinition {}

#[derive(Debug, Deserialize)]
struct DestinySackRewardItemListDefinition {}

#[derive(Debug, Deserialize)]
struct DestinySandboxPatternDefinition {}

#[derive(Debug, Deserialize)]
struct DestinySeasonDefinition {}

#[derive(Debug, Deserialize)]
struct DestinySeasonPassDefinition {}

#[derive(Debug, Deserialize)]
struct DestinySocketCategoryDefinition {}

#[derive(Debug, Deserialize)]
struct DestinySocketTypeDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyTraitDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyTraitCategoryDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyUnlockCountMappingDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyUnlockEventDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyUnlockExpressionMappingDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyVendorDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyMilestoneDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyActivityModifierDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyReportReasonCategoryDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyArtifactDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyBreakerTypeDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyChecklistDefinition {}

#[derive(Debug, Deserialize)]
struct DestinyEnergyTypeDefinition {}
