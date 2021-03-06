use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::bnet::entities::{
    dates::DateRange, interpolation::InterpolationPoint, links::HyperlinkReference,
};

use self::{
    animations::DestinyAnimationReference,
    common::DestinyDisplayPropertiesDefinition,
    items::{DestinyDerivedItemCategoryDefinition, DestinyItemPlugDefinition},
    sources::DestinyItemSourceDefinition,
    vendors::DestinyVendorLocationDefinition,
};

use super::{
    constants::DestinyEnvironmentLocationMapping, misc::DestinyColor, DestinyGender,
    DestinyItemQuantity, DyeReference,
};

pub mod activity_modifiers;
pub mod animations;
pub mod artifacts;
pub mod breaker_types;
pub mod checklists;
pub mod collectibles;
pub mod common;
pub mod director;
pub mod energy_types;
pub mod items;
pub mod lore;
pub mod metrics;
pub mod milestones;
pub mod power_caps;
pub mod presentation;
pub mod progression;
pub mod records;
pub mod reporting;
pub mod seasons;
pub mod sockets;
pub mod sources;
pub mod traits;
pub mod vendors;

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

/// The static data about Activities in Destiny 2.
///
/// Note that an Activity must be combined with an ActivityMode to know - from
/// a Gameplay perspective - what the user is "Playing".
///
/// In most PvE activities, this is fairly straightforward. A Story Activity
/// can only be played in the Story Activity Mode.
///
/// However, in PvP activities, the Activity alone only tells you the map being
/// played, or the Playlist that the user chose to enter. You'll need to know
/// the Activity Mode they're playing to know that they're playing Mode X on Map Y.
///
/// Activity Definitions tell a great deal of information about what *could* be
/// relevant to a user: what rewards they can earn, what challenges could be
/// performed, what modifiers could be applied. To figure out which of these
/// properties is actually live, you'll need to combine the definition with
/// "Live" data from one of the Destiny endpoints.
///
/// Activities also have Activity Types, but unfortunately in Destiny 2 these
/// are even less reliable of a source of information than they were in Destiny 1.
/// I will be looking into ways to provide more reliable sources for type
/// information as time goes on, but for now we're going to have to deal with
/// the limitations. See DestinyActivityTypeDefinition for more information.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityDefinition {
    activity_graph_list: Option<Vec<DestinyActivityGraphListEntryDefinition>>,
    activity_light_level: Option<i32>,
    activity_location_mappings: Option<Vec<DestinyEnvironmentLocationMapping>>,
    activity_mode_hashes: Option<Vec<u32>>,
    activity_mode_types: Option<Vec<i32>>,
    activity_type_hash: Option<u32>,
    challenges: Option<Vec<DestinyActivityChallengeDefinition>>,
    destination_hash: Option<u32>,
    direct_activity_mode_hash: Option<u32>,
    direct_activity_mode_type: Option<i32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    guided_game: Option<DestinyActivityGuidedBlockDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    insertion_points: Option<Vec<DestinyActivityInsertionPointDefinition>>,
    is_playlist: Option<bool>,
    is_pvp: Option<bool>,
    loadouts: Option<Vec<DestinyActivityLoadoutRequirementSet>>,
    matchmaking: Option<DestinyActivityMatchmakingBlockDefinition>,
    modifiers: Option<Vec<DestinyActivityModifierReferenceDefinition>>,
    optional_unlock_strings: Option<Vec<DestinyActivityUnlockStringDefinition>>,
    original_display_properties: Option<DestinyDisplayPropertiesDefinition>,
    pgcr_image: Option<String>,
    place_hash: Option<u32>,
    playlist_items: Option<Vec<DestinyActivityPlaylistItemDefinition>>,
    redacted: Option<bool>,
    release_icon: Option<String>,
    release_time: Option<i32>,
    rewards: Option<Vec<DestinyActivityRewardDefinition>>,
    selection_screen_display_properties: Option<DestinyDisplayPropertiesDefinition>,
    tier: Option<i32>,
}

impl DestinyActivityDefinition {
    /// Unfortunately, in practice this is almost never populated. In theory,
    /// this is supposed to tell which Activity Graph to show if you bring up
    /// the director while in this activity.
    pub fn activity_graph_list(&self) -> Option<&Vec<DestinyActivityGraphListEntryDefinition>> {
        self.activity_graph_list.as_ref()
    }

    /// The recommended light level for this activity.
    pub fn activity_light_level(&self) -> Option<i32> {
        self.activity_light_level
    }

    /// A list of location mappings that are affected by this activity. Pulled
    /// out of DestinyLocationDefinitions for our/your lookup convenience.
    pub fn activity_location_mappings(&self) -> Option<&Vec<DestinyEnvironmentLocationMapping>> {
        self.activity_location_mappings.as_ref()
    }

    /// The hash identifiers for Activity Modes relevant to this activity.
    ///
    /// Note that if this is a playlist, the specific playlist entry chosen
    /// will determine the actual activity modes that end up being relevant.
    pub fn activity_mode_hashes(&self) -> Option<&Vec<u32>> {
        self.activity_mode_hashes.as_ref()
    }

    /// The activity modes - if any - in enum form. Because we can't seem to
    /// escape the enums.
    pub fn activity_mode_types(&self) -> Option<&Vec<i32>> {
        self.activity_mode_types.as_ref()
    }

    /// The hash identifier for the Activity Type of this Activity.
    ///
    /// You may use it to look up the DestinyActivityTypeDefinition for human
    /// readable info, but be forewarned: Playlists and many PVP Map Activities
    /// will map to generic Activity Types. You'll have to use your knowledge
    /// of the Activity Mode being played to get more specific information about
    /// what the user is playing
    pub fn activity_type_hash(&self) -> Option<u32> {
        self.activity_type_hash
    }

    /// An activity can have many Challenges, of which any subset of them may
    /// be active for play at any given period of time. This gives the
    /// information about the challenges and data that we use to understand
    /// when they're active and what rewards they provide. Sadly, at the moment
    /// there's no central definition for challenges: much like "Skulls" were in
    /// Destiny 1, these are defined on individual activities and there can be
    /// many duplicates/near duplicates across the Destiny 2 ecosystem. I have
    /// it in mind to centralize these in a future revision of the API, but we
    /// are out of time.
    pub fn challenges(&self) -> Option<&Vec<DestinyActivityChallengeDefinition>> {
        self.challenges.as_ref()
    }

    /// The hash identifier for the Destination on which this Activity is played.
    ///
    /// Use it to look up the DestinyDestinationDefinition for human readable
    /// info about the destination. A Destination can be thought of as a more
    /// specific location than a "Place". For instance, if the "Place" is Earth,
    /// the "Destination" would be a specific city or region on Earth.
    pub fn destination_hash(&self) -> Option<u32> {
        self.destination_hash
    }

    /// If this activity had an activity mode directly defined on it, this will
    /// be the hash of that mode.
    pub fn direct_activity_mode_hash(&self) -> Option<u32> {
        self.direct_activity_mode_hash
    }

    /// If the activity had an activity mode directly defined on it, this will
    /// be the enum value of that mode.
    pub fn direct_activity_mode_type(&self) -> Option<i32> {
        self.direct_activity_mode_type
    }

    /// The title, subtitle, and icon for the activity.
    ///
    /// We do a little post-processing on this to try and account for Activities
    /// where the designers have left this data too minimal to determine what
    /// activity is actually being played.
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    /// This block of data, if it exists, provides information about the guided
    /// game experience and restrictions for this activity. If it doesn't exist,
    /// the game is not able to be played as a guided game.
    pub fn guided_game(&self) -> Option<&DestinyActivityGuidedBlockDefinition> {
        self.guided_game.as_ref()
    }

    /// The unique identifier for this entity. Guaranteed to be unique for the
    /// type of entity, but not globally. When entities refer to each other in
    /// Destiny content, it is this hash that they are referring to.
    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    /// The index of the entity as it was found in the investment tables.
    pub fn index(&self) -> Option<i32> {
        self.index
    }

    /// The list of phases or points of entry into an activity, along with
    /// information we can use to determine their gating and availability.
    pub fn insertion_points(&self) -> Option<&Vec<DestinyActivityInsertionPointDefinition>> {
        self.insertion_points.as_ref()
    }

    /// If True, this Activity is actually a Playlist that refers to multiple
    /// possible specific Activities and Activity Modes. For instance,
    /// a Crucible Playlist may have references to multiple Activities (Maps)
    /// with multiple Activity Modes (specific PvP gameplay modes). If this is
    /// true, refer to the playlistItems property for the specific entries in
    /// the playlist.
    pub fn is_playlist(&self) -> Option<bool> {
        self.is_playlist
    }

    /// If true, this activity is a PVP activity or playlist.
    pub fn is_pvp(&self) -> Option<bool> {
        self.is_pvp
    }

    /// The set of all possible loadout requirements that could be active for
    /// this activity. Only one will be active at any given time, and you can
    /// discover which one through activity-associated data such as Milestones
    /// that have activity info on them.
    pub fn loadouts(&self) -> Option<&Vec<DestinyActivityLoadoutRequirementSet>> {
        self.loadouts.as_ref()
    }

    /// This block of data provides information about the Activity's matchmaking
    /// attributes: how many people can join and such.
    pub fn matchmaking(&self) -> Option<&DestinyActivityMatchmakingBlockDefinition> {
        self.matchmaking.as_ref()
    }

    /// Activities can have Modifiers, as defined in
    /// DestinyActivityModifierDefinition. These are references to the modifiers
    /// that *can* be applied to that activity, along with data that we use to
    /// determine if that modifier is actually active at any given point in time.
    pub fn modifiers(&self) -> Option<&Vec<DestinyActivityModifierReferenceDefinition>> {
        self.modifiers.as_ref()
    }

    /// If there are status strings related to the activity and based on
    /// internal state of the game, account, or character, then this will be
    /// the definition of those strings and the states needed in order for the
    /// strings to be shown.
    pub fn optional_unlock_strings(&self) -> Option<&Vec<DestinyActivityUnlockStringDefinition>> {
        self.optional_unlock_strings.as_ref()
    }

    /// The unadulterated form of the display properties, as they ought to be
    /// shown in the Director (if the activity appears in the director).
    pub fn original_display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.original_display_properties.as_ref()
    }

    /// When Activities are completed, we generate a "Post-Game Carnage Report",
    /// or PGCR, with details about what happened in that activity (how many
    /// kills someone got, which team won, etc...) We use this image as the
    /// background when displaying PGCR information, and often use it when we
    /// refer to the Activity in general.
    pub fn pgcr_image(&self) -> Option<&String> {
        self.pgcr_image.as_ref()
    }

    /// The hash identifier for the "Place" on which this Activity is played.
    ///
    /// Use it to look up the DestinyPlaceDefinition for human readable info
    /// about the Place. A Place is the largest-scoped concept for location
    /// information. For instance, if the "Place" is Earth, the "Destination"
    /// would be a specific city or region on Earth.
    pub fn place_hash(&self) -> Option<u32> {
        self.place_hash
    }

    /// Represents all of the possible activities that could be played in the
    /// Playlist, along with information that we can use to determine if they
    /// are active at the present time.
    pub fn playlist_items(&self) -> Option<&Vec<DestinyActivityPlaylistItemDefinition>> {
        self.playlist_items.as_ref()
    }

    /// If this is true, then there is an entity with this identifier/type
    /// combination, but BNet is not yet allowed to show it. Sorry!
    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    /// If the activity has an icon associated with a specific release (such as
    /// a DLC), this is the path to that release's icon.
    pub fn release_icon(&self) -> Option<&String> {
        self.release_icon.as_ref()
    }

    /// If the activity will not be visible until a specific and known time,
    /// this will be the seconds since the Epoch when it will become visible.
    pub fn release_time(&self) -> Option<i32> {
        self.release_time
    }

    /// The expected possible rewards for the activity.
    ///
    /// These rewards may or may not be accessible for an individual player
    /// based on their character state, the account state, and even the game's
    /// state overall. But it is a useful reference for possible rewards you
    /// can earn in the activity. These match up to rewards displayed when you
    /// hover over the Activity in the in-game Director, and often refer to
    /// Placeholder or "Dummy" items: items that tell you what you can earn in
    /// vague terms rather than what you'll specifically be earning (partly
    /// because the game doesn't even know what you'll earn specifically until
    /// you roll for it at the end)
    pub fn rewards(&self) -> Option<&Vec<DestinyActivityRewardDefinition>> {
        self.rewards.as_ref()
    }

    /// The title, subtitle, and icon for the activity as determined by
    /// Selection Screen data, if there is any for this activity. There won't
    /// be data in this field if the activity is never shown in a
    /// selection/options screen.
    pub fn selection_screen_display_properties(
        &self,
    ) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.selection_screen_display_properties.as_ref()
    }

    /// The difficulty tier of the activity.
    pub fn tier(&self) -> Option<i32> {
        self.tier
    }
}

/// Represents a reference to a Challenge, which for now is just an Objective.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityChallengeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityChallengeDefinition {
    dummy_rewards: Option<Vec<DestinyItemQuantity>>,
    objective_hash: Option<u32>,
}

impl DestinyActivityChallengeDefinition {
    /// The rewards as they're represented in the UI.
    ///
    /// Note that they generally link to "dummy" items that give a summary of
    /// rewards rather than direct, real items themselves.
    ///
    /// If the quantity is 0, don't show the quantity.
    pub fn dummy_rewards(&self) -> Option<&Vec<DestinyItemQuantity>> {
        self.dummy_rewards.as_ref()
    }

    /// The hash for the Objective that matches this challenge. Use it to look
    /// up the DestinyObjectiveDefinition.
    pub fn objective_hash(&self) -> Option<u32> {
        self.objective_hash
    }
}

/// Destinations and Activities may have default Activity Graphs that should be
/// shown when you bring up the Director and are playing in either.
///
/// This contract defines the graph referred to and the gating for when it is relevant.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityGraphListEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphListEntryDefinition {
    activity_graph_hash: Option<u32>,
}

impl DestinyActivityGraphListEntryDefinition {
    /// The hash identifier of the DestinyActivityGraphDefinition that should
    /// be shown when opening the director.
    pub fn activity_graph_hash(&self) -> Option<u32> {
        self.activity_graph_hash
    }
}

/// Guided Game information for this activity.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityGuidedBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGuidedBlockDefinition {
    guided_disband_count: Option<i32>,
    guided_max_lobby_size: Option<i32>,
    guided_min_lobby_size: Option<i32>,
}

impl DestinyActivityGuidedBlockDefinition {
    /// If -1, the guided group cannot be disbanded. Otherwise, take the total
    /// # of players in the activity and subtract this number: that is the
    /// total # of votes needed for the guided group to disband.
    pub fn guided_disband_count(&self) -> Option<i32> {
        self.guided_disband_count
    }

    /// The maximum amount of people that can be in the waiting lobby.
    pub fn guided_max_lobby_size(&self) -> Option<i32> {
        self.guided_max_lobby_size
    }

    /// The minimum amount of people that can be in the waiting lobby.
    pub fn guided_min_lobby_size(&self) -> Option<i32> {
        self.guided_min_lobby_size
    }
}

/// A point of entry into an activity, gated by an unlock flag and with some
/// more-or-less useless (for our purposes) phase information. I'm including
/// it in case we end up being able to bolt more useful information onto it
/// in the future.
///
/// UPDATE: Turns out this information isn't actually useless, and is in fact
/// actually useful for people. Who would have thought? We still don't have
/// localized info for it, but at least this will help people when they're
/// looking at phase indexes in stats data, or when they want to know what
/// phases have been completed on a weekly achievement.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityInsertionPointDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityInsertionPointDefinition {
    phase_hash: Option<u32>,
}

impl DestinyActivityInsertionPointDefinition {
    /// A unique hash value representing the phase. This can be useful for,
    /// for example, comparing how different instances of Raids have phases
    /// in different orders!
    pub fn phase_hash(&self) -> Option<u32> {
        self.phase_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityLoadoutRequirement
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityLoadoutRequirement {
    allowed_equipped_item_hashes: Option<Vec<u32>>,
    allowed_weapon_sub_types: Option<Vec<i32>>,
    equipment_slot_hash: Option<u32>,
}

impl DestinyActivityLoadoutRequirement {
    pub fn allowed_equipped_item_hashes(&self) -> Option<&Vec<u32>> {
        self.allowed_equipped_item_hashes.as_ref()
    }

    pub fn allowed_weapon_sub_types(&self) -> Option<&Vec<i32>> {
        self.allowed_weapon_sub_types.as_ref()
    }

    pub fn equipment_slot_hash(&self) -> Option<u32> {
        self.equipment_slot_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityLoadoutRequirementSet
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityLoadoutRequirementSet {
    requirements: Option<Vec<DestinyActivityLoadoutRequirement>>,
}

impl DestinyActivityLoadoutRequirementSet {
    /// The set of requirements that will be applied on the activity if this
    /// requirement set is active.
    pub fn requirements(&self) -> Option<&Vec<DestinyActivityLoadoutRequirement>> {
        self.requirements.as_ref()
    }
}

/// Information about matchmaking and party size for the activity.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityMatchmakingBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityMatchmakingBlockDefinition {
    is_matchmade: Option<bool>,
    max_party: Option<i32>,
    max_players: Option<i32>,
    min_party: Option<i32>,
    requires_guardian_oath: Option<bool>,
}

impl DestinyActivityMatchmakingBlockDefinition {
    /// If TRUE, the activity is matchmade. Otherwise, it requires explicit
    /// forming of a party.
    pub fn is_matchmade(&self) -> Option<bool> {
        self.is_matchmade
    }

    /// The maximum # of people allowed in a Fireteam.
    pub fn max_party(&self) -> Option<i32> {
        self.max_party
    }

    /// The maximum # of people allowed across all teams in the activity.
    pub fn max_players(&self) -> Option<i32> {
        self.max_players
    }

    /// The minimum # of people in the fireteam for the activity to launch.
    pub fn min_party(&self) -> Option<i32> {
        self.min_party
    }

    /// If true, you have to Solemnly Swear to be up to Nothing But Good(tm) to play.
    pub fn requires_guardian_oath(&self) -> Option<bool> {
        self.requires_guardian_oath
    }
}

/// This definition represents an "Activity Mode" as it exists in the
/// Historical Stats endpoints. An individual Activity Mode represents a
/// collection of activities that are played in a certain way. For example,
/// Nightfall Strikes are part of a "Nightfall" activity mode, and any
/// activities played as the PVP mode "Clash" are part of the Clash activity mode.
///
/// Activity modes are nested under each other in a hierarchy, so that if you
/// ask for - for example - "AllPvP", you will get any PVP activities that the
/// user has played, regardless of what specific PVP mode was being played.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityModeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityModeDefinition {
    activity_mode_category: Option<i32>,
    activity_mode_mappings: Option<HashMap<u32, i32>>,
    display: Option<bool>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    friendly_name: Option<String>,
    hash: Option<u32>,
    index: Option<i32>,
    is_aggregate_mode: Option<bool>,
    is_team_based: Option<bool>,
    mode_type: Option<i32>,
    order: Option<i32>,
    parent_hashes: Option<Vec<u32>>,
    pgcr_image: Option<String>,
    redacted: Option<bool>,
}

impl DestinyActivityModeDefinition {
    /// The type of play being performed in broad terms (PVP, PVE)
    pub fn activity_mode_category(&self) -> Option<i32> {
        self.activity_mode_category
    }

    /// If this exists, the mode has specific Activities (referred to by the
    /// Key) that should instead map to other Activity Modes when they are
    /// played. This was useful in D1 for Private Matches, where we wanted to
    /// have Private Matches as an activity mode while still referring to the
    /// specific mode being played.
    pub fn activity_mode_mappings(&self) -> Option<&HashMap<u32, i32>> {
        self.activity_mode_mappings.as_ref()
    }

    /// If FALSE, we want to ignore this type when we're showing activity modes
    /// in BNet UI. It will still be returned in case 3rd parties want to use
    /// it for any purpose.
    pub fn display(&self) -> Option<bool> {
        self.display
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    /// A Friendly identifier you can use for referring to this Activity Mode.
    /// We really only used this in our URLs, so... you know, take that for
    /// whatever it's worth.
    pub fn friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// The unique identifier for this entity. Guaranteed to be unique for the
    /// type of entity, but not globally.
    ///
    /// When entities refer to each other in Destiny content, it is this hash
    /// that they are referring to.
    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    /// The index of the entity as it was found in the investment tables.
    pub fn index(&self) -> Option<i32> {
        self.index
    }

    /// If true, this mode is an aggregation of other, more specific modes
    /// rather than being a mode in itself. This includes modes that group
    /// Features/Events rather than Gameplay, such as Trials of The Nine:
    /// Trials of the Nine being an Event that is interesting to see aggregate
    /// data for, but when you play the activities within Trials of the Nine
    /// they are more specific activity modes such as Clash.
    pub fn is_aggregate_mode(&self) -> Option<bool> {
        self.is_aggregate_mode
    }

    /// If True, this mode has oppositional teams fighting against each other
    /// rather than "Free-For-All" or Co-operative modes of play.
    ///
    /// Note that Aggregate modes are never marked as team based, even if they
    /// happen to be team based at the moment. At any time, an aggregate whose
    /// subordinates are only team based could be changed so that one or more
    /// aren't team based, and then this boolean won't make much sense (the
    /// aggregation would become "sometimes team based"). Let's not deal with
    /// that right now.
    pub fn is_team_based(&self) -> Option<bool> {
        self.is_team_based
    }

    /// The Enumeration value for this Activity Mode. Pass this identifier into
    /// Stats endpoints to get aggregate stats for this mode.
    pub fn mode_type(&self) -> Option<i32> {
        self.mode_type
    }

    /// The relative ordering of activity modes.
    pub fn order(&self) -> Option<i32> {
        self.order
    }

    /// The hash identifiers of the DestinyActivityModeDefinitions that
    /// represent all of the "parent" modes for this mode. For instance, the
    /// Nightfall Mode is also a member of AllStrikes and AllPvE.
    pub fn parent_hashes(&self) -> Option<&Vec<u32>> {
        self.parent_hashes.as_ref()
    }

    /// If this activity mode has a related PGCR image, this will be the path
    /// to said image.
    pub fn pgcr_image(&self) -> Option<&String> {
        self.pgcr_image.as_ref()
    }

    /// If this is true, then there is an entity with this identifier/type
    /// combination, but BNet is not yet allowed to show it. Sorry!
    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityModifierReferenceDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityModifierReferenceDefinition {
    activity_modifier_hash: Option<u32>,
}

impl DestinyActivityModifierReferenceDefinition {
    pub fn activity_modifier_hash(&self) -> Option<u32> {
        self.activity_modifier_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityPlaylistItemDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityPlaylistItemDefinition {
    activity_hash: Option<u32>,
    activity_mode_hashes: Option<Vec<u32>>,
    activity_mode_types: Option<Vec<i32>>,
    direct_activity_mode_hash: Option<u32>,
    direct_activity_mode_type: Option<i32>,
}

impl DestinyActivityPlaylistItemDefinition {
    pub fn activity_hash(&self) -> Option<u32> {
        self.activity_hash
    }

    pub fn activity_mode_hashes(&self) -> Option<&Vec<u32>> {
        self.activity_mode_hashes.as_ref()
    }

    pub fn activity_mode_types(&self) -> Option<&Vec<i32>> {
        self.activity_mode_types.as_ref()
    }

    pub fn direct_activity_mode_hash(&self) -> Option<u32> {
        self.direct_activity_mode_hash
    }

    pub fn direct_activity_mode_type(&self) -> Option<i32> {
        self.direct_activity_mode_type
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityRewardDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityRewardDefinition {
    reward_items: Option<Vec<DestinyItemQuantity>>,
    reward_text: Option<String>,
}

impl DestinyActivityRewardDefinition {
    pub fn reward_items(&self) -> Option<&Vec<DestinyItemQuantity>> {
        self.reward_items.as_ref()
    }

    pub fn reward_text(&self) -> Option<&String> {
        self.reward_text.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityTypeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityTypeDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
}

impl DestinyActivityTypeDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyActivityUnlockStringDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityUnlockStringDefinition {
    display_string: Option<String>,
}

impl DestinyActivityUnlockStringDefinition {
    pub fn display_string(&self) -> Option<&String> {
        self.display_string.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyArrangementRegionFilterDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyArrangementRegionFilterDefinition {
    arrangement_index_by_stat_value: Option<HashMap<i32, i32>>,
    art_arrangement_region_hash: Option<u32>,
    art_arrangement_region_index: Option<i32>,
    stat_hash: Option<u32>,
}

impl DestinyArrangementRegionFilterDefinition {
    pub fn arrangement_index_by_stat_value(&self) -> Option<&HashMap<i32, i32>> {
        self.arrangement_index_by_stat_value.as_ref()
    }

    pub fn art_arrangement_region_hash(&self) -> Option<u32> {
        self.art_arrangement_region_hash
    }

    pub fn art_arrangement_region_index(&self) -> Option<i32> {
        self.art_arrangement_region_index
    }

    pub fn stat_hash(&self) -> Option<u32> {
        self.stat_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyArtDyeReference
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyArtDyeReference {
    art_dye_channel_hash: Option<u32>,
}

impl DestinyArtDyeReference {
    pub fn art_dye_channel_hash(&self) -> Option<u32> {
        self.art_dye_channel_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyBubbleDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyBubbleDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
}

impl DestinyBubbleDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyClassDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyClassDefinition {
    class_type: Option<i32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    gendered_class_names: Option<HashMap<DestinyGender, String>>,
    gendered_class_names_by_gender_hash: Option<HashMap<u32, String>>,
    hash: Option<u32>,
    index: Option<i32>,
    mentor_vendor_hash: Option<u32>,
    redacted: Option<bool>,
}

impl DestinyClassDefinition {
    pub fn class_type(&self) -> Option<i32> {
        self.class_type
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn gendered_class_names(&self) -> Option<&HashMap<DestinyGender, String>> {
        self.gendered_class_names.as_ref()
    }

    pub fn gendered_class_names_by_gender_hash(&self) -> Option<&HashMap<u32, String>> {
        self.gendered_class_names_by_gender_hash.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn mentor_vendor_hash(&self) -> Option<u32> {
        self.mentor_vendor_hash
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyDamageTypeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyDamageTypeDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    enum_value: Option<i32>,
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
    show_icon: Option<bool>,
    transparent_icon_path: Option<String>,
}

impl DestinyDamageTypeDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn enum_value(&self) -> Option<i32> {
        self.enum_value
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn show_icon(&self) -> Option<bool> {
        self.show_icon
    }

    pub fn transparent_icon_path(&self) -> Option<&String> {
        self.transparent_icon_path.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyDestinationBubbleSettingDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyDestinationBubbleSettingDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
}

impl DestinyDestinationBubbleSettingDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyDestinationDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyDestinationDefinition {
    activity_graph_entries: Option<Vec<DestinyActivityGraphListEntryDefinition>>,
    bubble_settings: Option<Vec<DestinyDestinationBubbleSettingDefinition>>,
    bubbles: Option<Vec<DestinyBubbleDefinition>>,
    default_freeroam_activity_hash: Option<u32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    place_hash: Option<u32>,
    redacted: Option<bool>,
}

impl DestinyDestinationDefinition {
    pub fn activity_graph_entries(&self) -> Option<&Vec<DestinyActivityGraphListEntryDefinition>> {
        self.activity_graph_entries.as_ref()
    }

    pub fn bubble_settings(&self) -> Option<&Vec<DestinyDestinationBubbleSettingDefinition>> {
        self.bubble_settings.as_ref()
    }

    pub fn bubbles(&self) -> Option<&Vec<DestinyBubbleDefinition>> {
        self.bubbles.as_ref()
    }

    pub fn default_freeroam_activity_hash(&self) -> Option<u32> {
        self.default_freeroam_activity_hash
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn place_hash(&self) -> Option<u32> {
        self.place_hash
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyDisplayCategoryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyDisplayCategoryDefinition {
    display_category_hash: Option<u32>,
    display_in_banner: Option<bool>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    display_style_hash: Option<u32>,
    display_style_identifier: Option<String>,
    identifier: Option<String>,
    index: Option<i32>,
    progression_hash: Option<u32>,
    sort_order: Option<u32>,
}

impl DestinyDisplayCategoryDefinition {
    pub fn display_category_hash(&self) -> Option<u32> {
        self.display_category_hash
    }

    pub fn display_in_banner(&self) -> Option<bool> {
        self.display_in_banner
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn display_style_hash(&self) -> Option<u32> {
        self.display_style_hash
    }

    pub fn display_style_identifier(&self) -> Option<&String> {
        self.display_style_identifier.as_ref()
    }

    pub fn identifier(&self) -> Option<&String> {
        self.identifier.as_ref()
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn progression_hash(&self) -> Option<u32> {
        self.progression_hash
    }

    pub fn sort_order(&self) -> Option<u32> {
        self.sort_order
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyEquipmentSlotDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyEquipmentSlotDefinition {
    apply_custom_art_dyes: Option<bool>,
    art_dye_channels: Option<Vec<DestinyArtDyeReference>>,
    bucket_type_hash: Option<u32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    equipment_category_hash: Option<u32>,
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
}

impl DestinyEquipmentSlotDefinition {
    pub fn apply_custom_art_dyes(&self) -> Option<bool> {
        self.apply_custom_art_dyes
    }

    pub fn art_dye_channels(&self) -> Option<&Vec<DestinyArtDyeReference>> {
        self.art_dye_channels.as_ref()
    }

    pub fn bucket_type_hash(&self) -> Option<u32> {
        self.bucket_type_hash
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn equipment_category_hash(&self) -> Option<u32> {
        self.equipment_category_hash
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyEquippingBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyEquippingBlockDefinition {
    ammo_type: Option<i32>,
    attributes: Option<i32>,
    display_strings: Option<Vec<String>>,
    equipment_slot_type_hash: Option<u32>,
    gearset_item_hash: Option<u32>,
    unique_label: Option<String>,
    unique_label_hash: Option<u32>,
}

impl DestinyEquippingBlockDefinition {
    pub fn ammo_type(&self) -> Option<i32> {
        self.ammo_type
    }

    pub fn attributes(&self) -> Option<i32> {
        self.attributes
    }

    pub fn display_strings(&self) -> Option<&Vec<String>> {
        self.display_strings.as_ref()
    }

    pub fn equipment_slot_type_hash(&self) -> Option<u32> {
        self.equipment_slot_type_hash
    }

    pub fn gearset_item_hash(&self) -> Option<u32> {
        self.gearset_item_hash
    }

    pub fn unique_label(&self) -> Option<&String> {
        self.unique_label.as_ref()
    }

    pub fn unique_label_hash(&self) -> Option<u32> {
        self.unique_label_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyFactionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyFactionDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    progression_hash: Option<u32>,
    redacted: Option<bool>,
    reward_item_hash: Option<u32>,
    reward_vendor_hash: Option<u32>,
    token_values: Option<HashMap<u32, u32>>,
    vendors: Option<Vec<DestinyFactionVendorDefinition>>,
}

impl DestinyFactionDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn progression_hash(&self) -> Option<u32> {
        self.progression_hash
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn reward_item_hash(&self) -> Option<u32> {
        self.reward_item_hash
    }

    pub fn reward_vendor_hash(&self) -> Option<u32> {
        self.reward_vendor_hash
    }

    pub fn token_values(&self) -> Option<&HashMap<u32, u32>> {
        self.token_values.as_ref()
    }

    pub fn vendors(&self) -> Option<&Vec<DestinyFactionVendorDefinition>> {
        self.vendors.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyFactionVendorDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyFactionVendorDefinition {
    background_image_path: Option<String>,
    destination_hash: Option<u32>,
    vendor_hash: Option<u32>,
}

impl DestinyFactionVendorDefinition {
    pub fn background_image_path(&self) -> Option<&String> {
        self.background_image_path.as_ref()
    }

    pub fn destination_hash(&self) -> Option<u32> {
        self.destination_hash
    }

    pub fn vendor_hash(&self) -> Option<u32> {
        self.vendor_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyGearArtArrangementReference
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyGearArtArrangementReference {
    art_arrangement_hash: Option<u32>,
    class_hash: Option<u32>,
}

impl DestinyGearArtArrangementReference {
    pub fn art_arrangement_hash(&self) -> Option<u32> {
        self.art_arrangement_hash
    }

    pub fn class_hash(&self) -> Option<u32> {
        self.class_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyGenderDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyGenderDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    gender_type: Option<i32>,
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
}

impl DestinyGenderDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn gender_type(&self) -> Option<i32> {
        self.gender_type
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyInventoryBucketDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyInventoryBucketDefinition {
    bucket_order: Option<i32>,
    category: Option<i32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    enabled: Option<bool>,
    fifo: Option<bool>,
    has_transfer_destination: Option<bool>,
    hash: Option<u32>,
    index: Option<i32>,
    item_count: Option<i32>,
    location: Option<i32>,
    redacted: Option<bool>,
    scope: Option<i32>,
}

impl DestinyInventoryBucketDefinition {
    pub fn bucket_order(&self) -> Option<i32> {
        self.bucket_order
    }

    pub fn category(&self) -> Option<i32> {
        self.category
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn enabled(&self) -> Option<bool> {
        self.enabled
    }

    pub fn fifo(&self) -> Option<bool> {
        self.fifo
    }

    pub fn has_transfer_destination(&self) -> Option<bool> {
        self.has_transfer_destination
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn item_count(&self) -> Option<i32> {
        self.item_count
    }

    pub fn location(&self) -> Option<i32> {
        self.location
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn scope(&self) -> Option<i32> {
        self.scope
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyInventoryItemDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyInventoryItemDefinition {
    action: Option<DestinyItemActionBlockDefinition>,
    allow_actions: Option<bool>,
    animations: Option<Vec<DestinyAnimationReference>>,
    background_color: Option<DestinyColor>,
    breaker_type: Option<i32>,
    breaker_type_hash: Option<u32>,
    class_type: Option<i32>,
    collectible_hash: Option<u32>,
    crafting: Option<DestinyItemCraftingBlockDefinition>,
    damage_type_hashes: Option<Vec<u32>>,
    damage_types: Option<Vec<i32>>,
    default_damage_type: Option<i32>,
    default_damage_type_hash: Option<u32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    display_source: Option<String>,
    does_postmaster_pull_have_side_effects: Option<bool>,
    emblem_objective_hash: Option<u32>,
    equippable: Option<bool>,
    equipping_block: Option<DestinyEquippingBlockDefinition>,
    flavor_text: Option<String>,
    gearset: Option<DestinyItemGearsetBlockDefinition>,
    hash: Option<u32>,
    icon_watermark: Option<String>,
    icon_watermark_shelved: Option<String>,
    index: Option<i32>,
    inventory: Option<DestinyItemInventoryBlockDefinition>,
    investment_stats: Option<Vec<DestinyItemInvestmentStatDefinition>>,
    is_wrapper: Option<bool>,
    item_category_hashes: Option<Vec<u32>>,
    item_sub_type: Option<i32>,
    item_type: Option<i32>,
    item_type_and_tier_display_name: Option<String>,
    item_type_display_name: Option<String>,
    links: Option<Vec<HyperlinkReference>>,
    lore_hash: Option<u32>,
    metrics: Option<DestinyItemMetricBlockDefinition>,
    non_transferrable: Option<bool>,
    objectives: Option<DestinyItemObjectiveBlockDefinition>,
    perks: Option<Vec<DestinyItemPerkEntryDefinition>>,
    plug: Option<DestinyItemPlugDefinition>,
    preview: Option<DestinyItemPreviewBlockDefinition>,
    quality: Option<DestinyItemQualityBlockDefinition>,
    redacted: Option<bool>,
    sack: Option<DestinyItemSackBlockDefinition>,
    screenshot: Option<String>,
    season_hash: Option<u32>,
    secondary_icon: Option<String>,
    secondary_overlay: Option<String>,
    secondary_special: Option<String>,
    set_data: Option<DestinyItemSetBlockDefinition>,
    sockets: Option<DestinyItemSocketBlockDefinition>,
    source_data: Option<DestinyItemSourceBlockDefinition>,
    special_item_type: Option<i32>,
    stats: Option<DestinyItemStatBlockDefinition>,
    summary: Option<DestinyItemSummaryBlockDefinition>,
    summary_item_hash: Option<u32>,
    talent_grid: Option<DestinyItemTalentGridBlockDefinition>,
    tooltip_notifications: Option<Vec<DestinyItemTooltipNotification>>,
    tooltip_style: Option<String>,
    trait_hashes: Option<Vec<u32>>,
    trait_ids: Option<Vec<String>>,
    translation_block: Option<DestinyItemTranslationBlockDefinition>,
    ui_item_display_style: Option<String>,
    value: Option<DestinyItemValueBlockDefinition>,
}

impl DestinyInventoryItemDefinition {
    pub fn action(&self) -> Option<&DestinyItemActionBlockDefinition> {
        self.action.as_ref()
    }

    pub fn allow_actions(&self) -> Option<bool> {
        self.allow_actions
    }

    pub fn animations(&self) -> Option<&Vec<DestinyAnimationReference>> {
        self.animations.as_ref()
    }

    pub fn background_color(&self) -> Option<&DestinyColor> {
        self.background_color.as_ref()
    }

    pub fn breaker_type(&self) -> Option<i32> {
        self.breaker_type
    }

    pub fn breaker_type_hash(&self) -> Option<u32> {
        self.breaker_type_hash
    }

    pub fn class_type(&self) -> Option<i32> {
        self.class_type
    }

    pub fn collectible_hash(&self) -> Option<u32> {
        self.collectible_hash
    }

    pub fn crafting(&self) -> Option<&DestinyItemCraftingBlockDefinition> {
        self.crafting.as_ref()
    }

    pub fn damage_type_hashes(&self) -> Option<&Vec<u32>> {
        self.damage_type_hashes.as_ref()
    }

    pub fn damage_types(&self) -> Option<&Vec<i32>> {
        self.damage_types.as_ref()
    }

    pub fn default_damage_type(&self) -> Option<i32> {
        self.default_damage_type
    }

    pub fn default_damage_type_hash(&self) -> Option<u32> {
        self.default_damage_type_hash
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn display_source(&self) -> Option<&String> {
        self.display_source.as_ref()
    }

    pub fn does_postmaster_pull_have_side_effects(&self) -> Option<bool> {
        self.does_postmaster_pull_have_side_effects
    }

    pub fn emblem_objective_hash(&self) -> Option<u32> {
        self.emblem_objective_hash
    }

    pub fn equippable(&self) -> Option<bool> {
        self.equippable
    }

    pub fn equipping_block(&self) -> Option<&DestinyEquippingBlockDefinition> {
        self.equipping_block.as_ref()
    }

    pub fn flavor_text(&self) -> Option<&String> {
        self.flavor_text.as_ref()
    }

    pub fn gearset(&self) -> Option<&DestinyItemGearsetBlockDefinition> {
        self.gearset.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn icon_watermark(&self) -> Option<&String> {
        self.icon_watermark.as_ref()
    }

    pub fn icon_watermark_shelved(&self) -> Option<&String> {
        self.icon_watermark_shelved.as_ref()
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn inventory(&self) -> Option<&DestinyItemInventoryBlockDefinition> {
        self.inventory.as_ref()
    }

    pub fn investment_stats(&self) -> Option<&Vec<DestinyItemInvestmentStatDefinition>> {
        self.investment_stats.as_ref()
    }

    pub fn is_wrapper(&self) -> Option<bool> {
        self.is_wrapper
    }

    pub fn item_category_hashes(&self) -> Option<&Vec<u32>> {
        self.item_category_hashes.as_ref()
    }

    pub fn item_sub_type(&self) -> Option<i32> {
        self.item_sub_type
    }

    pub fn item_type(&self) -> Option<i32> {
        self.item_type
    }

    pub fn item_type_and_tier_display_name(&self) -> Option<&String> {
        self.item_type_and_tier_display_name.as_ref()
    }

    pub fn item_type_display_name(&self) -> Option<&String> {
        self.item_type_display_name.as_ref()
    }

    pub fn links(&self) -> Option<&Vec<HyperlinkReference>> {
        self.links.as_ref()
    }

    pub fn lore_hash(&self) -> Option<u32> {
        self.lore_hash
    }

    pub fn metrics(&self) -> Option<&DestinyItemMetricBlockDefinition> {
        self.metrics.as_ref()
    }

    pub fn non_transferrable(&self) -> Option<bool> {
        self.non_transferrable
    }

    pub fn objectives(&self) -> Option<&DestinyItemObjectiveBlockDefinition> {
        self.objectives.as_ref()
    }

    pub fn perks(&self) -> Option<&Vec<DestinyItemPerkEntryDefinition>> {
        self.perks.as_ref()
    }

    pub fn plug(&self) -> Option<&DestinyItemPlugDefinition> {
        self.plug.as_ref()
    }

    pub fn preview(&self) -> Option<&DestinyItemPreviewBlockDefinition> {
        self.preview.as_ref()
    }

    pub fn quality(&self) -> Option<&DestinyItemQualityBlockDefinition> {
        self.quality.as_ref()
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn sack(&self) -> Option<&DestinyItemSackBlockDefinition> {
        self.sack.as_ref()
    }

    pub fn screenshot(&self) -> Option<&String> {
        self.screenshot.as_ref()
    }

    pub fn season_hash(&self) -> Option<u32> {
        self.season_hash
    }

    pub fn secondary_icon(&self) -> Option<&String> {
        self.secondary_icon.as_ref()
    }

    pub fn secondary_overlay(&self) -> Option<&String> {
        self.secondary_overlay.as_ref()
    }

    pub fn secondary_special(&self) -> Option<&String> {
        self.secondary_special.as_ref()
    }

    pub fn set_data(&self) -> Option<&DestinyItemSetBlockDefinition> {
        self.set_data.as_ref()
    }

    pub fn sockets(&self) -> Option<&DestinyItemSocketBlockDefinition> {
        self.sockets.as_ref()
    }

    pub fn source_data(&self) -> Option<&DestinyItemSourceBlockDefinition> {
        self.source_data.as_ref()
    }

    pub fn special_item_type(&self) -> Option<i32> {
        self.special_item_type
    }

    pub fn stats(&self) -> Option<&DestinyItemStatBlockDefinition> {
        self.stats.as_ref()
    }

    pub fn summary(&self) -> Option<&DestinyItemSummaryBlockDefinition> {
        self.summary.as_ref()
    }

    pub fn summary_item_hash(&self) -> Option<u32> {
        self.summary_item_hash
    }

    pub fn talent_grid(&self) -> Option<&DestinyItemTalentGridBlockDefinition> {
        self.talent_grid.as_ref()
    }

    pub fn tooltip_notifications(&self) -> Option<&Vec<DestinyItemTooltipNotification>> {
        self.tooltip_notifications.as_ref()
    }

    pub fn tooltip_style(&self) -> Option<&String> {
        self.tooltip_style.as_ref()
    }

    pub fn trait_hashes(&self) -> Option<&Vec<u32>> {
        self.trait_hashes.as_ref()
    }

    pub fn trait_ids(&self) -> Option<&Vec<String>> {
        self.trait_ids.as_ref()
    }

    pub fn translation_block(&self) -> Option<&DestinyItemTranslationBlockDefinition> {
        self.translation_block.as_ref()
    }

    pub fn ui_item_display_style(&self) -> Option<&String> {
        self.ui_item_display_style.as_ref()
    }

    pub fn value(&self) -> Option<&DestinyItemValueBlockDefinition> {
        self.value.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyInventoryItemStatDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyInventoryItemStatDefinition {
    display_maximum: Option<i32>,
    maximum: Option<i32>,
    minimum: Option<i32>,
    stat_hash: Option<u32>,
    value: Option<i32>,
}

impl DestinyInventoryItemStatDefinition {
    pub fn display_maximum(&self) -> Option<i32> {
        self.display_maximum
    }

    pub fn maximum(&self) -> Option<i32> {
        self.maximum
    }

    pub fn minimum(&self) -> Option<i32> {
        self.minimum
    }

    pub fn stat_hash(&self) -> Option<u32> {
        self.stat_hash
    }

    pub fn value(&self) -> Option<i32> {
        self.value
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemActionBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemActionBlockDefinition {
    action_type_label: Option<String>,
    consume_entire_stack: Option<bool>,
    delete_on_action: Option<bool>,
    is_positive: Option<bool>,
    overlay_icon: Option<String>,
    overlay_screen_name: Option<String>,
    progression_rewards: Option<Vec<DestinyProgressionRewardDefinition>>,
    required_cooldown_hash: Option<u32>,
    required_cooldown_seconds: Option<i32>,
    required_items: Option<Vec<DestinyItemActionRequiredItemDefinition>>,
    required_location: Option<String>,
    use_on_acquire: Option<bool>,
    verb_description: Option<String>,
    verb_name: Option<String>,
}

impl DestinyItemActionBlockDefinition {
    pub fn action_type_label(&self) -> Option<&String> {
        self.action_type_label.as_ref()
    }

    pub fn consume_entire_stack(&self) -> Option<bool> {
        self.consume_entire_stack
    }

    pub fn delete_on_action(&self) -> Option<bool> {
        self.delete_on_action
    }

    pub fn is_positive(&self) -> Option<bool> {
        self.is_positive
    }

    pub fn overlay_icon(&self) -> Option<&String> {
        self.overlay_icon.as_ref()
    }

    pub fn overlay_screen_name(&self) -> Option<&String> {
        self.overlay_screen_name.as_ref()
    }

    pub fn progression_rewards(&self) -> Option<&Vec<DestinyProgressionRewardDefinition>> {
        self.progression_rewards.as_ref()
    }

    pub fn required_cooldown_hash(&self) -> Option<u32> {
        self.required_cooldown_hash
    }

    pub fn required_cooldown_seconds(&self) -> Option<i32> {
        self.required_cooldown_seconds
    }

    pub fn required_items(&self) -> Option<&Vec<DestinyItemActionRequiredItemDefinition>> {
        self.required_items.as_ref()
    }

    pub fn required_location(&self) -> Option<&String> {
        self.required_location.as_ref()
    }

    pub fn use_on_acquire(&self) -> Option<bool> {
        self.use_on_acquire
    }

    pub fn verb_description(&self) -> Option<&String> {
        self.verb_description.as_ref()
    }

    pub fn verb_name(&self) -> Option<&String> {
        self.verb_name.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemActionRequiredItemDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemActionRequiredItemDefinition {
    count: Option<i32>,
    delete_on_action: Option<bool>,
    item_hash: Option<u32>,
}

impl DestinyItemActionRequiredItemDefinition {
    pub fn count(&self) -> Option<i32> {
        self.count
    }

    pub fn delete_on_action(&self) -> Option<bool> {
        self.delete_on_action
    }

    pub fn item_hash(&self) -> Option<u32> {
        self.item_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemCategoryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemCategoryDefinition {
    deprecated: Option<bool>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    grant_destiny_breaker_type: Option<i32>,
    grant_destiny_class: Option<i32>,
    grant_destiny_item_type: Option<i32>,
    grant_destiny_sub_type: Option<i32>,
    group_category_only: Option<bool>,
    grouped_category_hashes: Option<Vec<u32>>,
    hash: Option<u32>,
    index: Option<i32>,
    item_type_regex: Option<String>,
    item_type_regex_not: Option<String>,
    origin_bucket_identifier: Option<String>,
    parent_category_hashes: Option<Vec<u32>>,
    plug_category_identifier: Option<String>,
    redacted: Option<bool>,
    short_title: Option<String>,
    trait_id: Option<String>,
    visible: Option<bool>,
}

impl DestinyItemCategoryDefinition {
    pub fn deprecated(&self) -> Option<bool> {
        self.deprecated
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn grant_destiny_breaker_type(&self) -> Option<i32> {
        self.grant_destiny_breaker_type
    }

    pub fn grant_destiny_class(&self) -> Option<i32> {
        self.grant_destiny_class
    }

    pub fn grant_destiny_item_type(&self) -> Option<i32> {
        self.grant_destiny_item_type
    }

    pub fn grant_destiny_sub_type(&self) -> Option<i32> {
        self.grant_destiny_sub_type
    }

    pub fn group_category_only(&self) -> Option<bool> {
        self.group_category_only
    }

    pub fn grouped_category_hashes(&self) -> Option<&Vec<u32>> {
        self.grouped_category_hashes.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn item_type_regex(&self) -> Option<&String> {
        self.item_type_regex.as_ref()
    }

    pub fn item_type_regex_not(&self) -> Option<&String> {
        self.item_type_regex_not.as_ref()
    }

    pub fn origin_bucket_identifier(&self) -> Option<&String> {
        self.origin_bucket_identifier.as_ref()
    }

    pub fn parent_category_hashes(&self) -> Option<&Vec<u32>> {
        self.parent_category_hashes.as_ref()
    }

    pub fn plug_category_identifier(&self) -> Option<&String> {
        self.plug_category_identifier.as_ref()
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn short_title(&self) -> Option<&String> {
        self.short_title.as_ref()
    }

    pub fn trait_id(&self) -> Option<&String> {
        self.trait_id.as_ref()
    }

    pub fn visible(&self) -> Option<bool> {
        self.visible
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemCraftingBlockBonusPlugDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemCraftingBlockBonusPlugDefinition {
    plug_item_hash: Option<u32>,
    socket_type_hash: Option<u32>,
}

impl DestinyItemCraftingBlockBonusPlugDefinition {
    pub fn plug_item_hash(&self) -> Option<u32> {
        self.plug_item_hash
    }

    pub fn socket_type_hash(&self) -> Option<u32> {
        self.socket_type_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemCraftingBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemCraftingBlockDefinition {
    base_material_requirements: Option<u32>,
    bonus_plugs: Option<Vec<DestinyItemCraftingBlockBonusPlugDefinition>>,
    failed_requirement_strings: Option<Vec<String>>,
    output_item_hash: Option<u32>,
    required_socket_type_hashes: Option<Vec<u32>>,
}

impl DestinyItemCraftingBlockDefinition {
    pub fn base_material_requirements(&self) -> Option<u32> {
        self.base_material_requirements
    }

    pub fn bonus_plugs(&self) -> Option<&Vec<DestinyItemCraftingBlockBonusPlugDefinition>> {
        self.bonus_plugs.as_ref()
    }

    pub fn failed_requirement_strings(&self) -> Option<&Vec<String>> {
        self.failed_requirement_strings.as_ref()
    }

    pub fn output_item_hash(&self) -> Option<u32> {
        self.output_item_hash
    }

    pub fn required_socket_type_hashes(&self) -> Option<&Vec<u32>> {
        self.required_socket_type_hashes.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemCreationEntryLevelDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemCreationEntryLevelDefinition {
    level: Option<i32>,
}

impl DestinyItemCreationEntryLevelDefinition {
    pub fn level(&self) -> Option<i32> {
        self.level
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemGearsetBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemGearsetBlockDefinition {
    item_list: Option<Vec<u32>>,
    tracking_value_max: Option<i32>,
}

impl DestinyItemGearsetBlockDefinition {
    pub fn item_list(&self) -> Option<&Vec<u32>> {
        self.item_list.as_ref()
    }

    pub fn tracking_value_max(&self) -> Option<i32> {
        self.tracking_value_max
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemIntrinsicSocketEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemIntrinsicSocketEntryDefinition {
    default_visible: Option<bool>,
    plug_item_hash: Option<u32>,
    socket_type_hash: Option<u32>,
}

impl DestinyItemIntrinsicSocketEntryDefinition {
    pub fn default_visible(&self) -> Option<bool> {
        self.default_visible
    }

    pub fn plug_item_hash(&self) -> Option<u32> {
        self.plug_item_hash
    }

    pub fn socket_type_hash(&self) -> Option<u32> {
        self.socket_type_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemInventoryBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemInventoryBlockDefinition {
    bucket_type_hash: Option<u32>,
    expiration_tooltip: Option<String>,
    expired_in_activity_message: Option<String>,
    expired_in_orbit_message: Option<String>,
    is_instance_item: Option<bool>,
    max_stack_size: Option<i32>,
    recipe_item_hash: Option<u32>,
    recovery_bucket_type_hash: Option<u32>,
    stack_unique_label: Option<String>,
    suppress_expiration_when_objectives_complete: Option<bool>,
    tier_type: Option<i32>,
    tier_type_hash: Option<u32>,
    tier_type_name: Option<String>,
}

impl DestinyItemInventoryBlockDefinition {
    pub fn bucket_type_hash(&self) -> Option<u32> {
        self.bucket_type_hash
    }

    pub fn expiration_tooltip(&self) -> Option<&String> {
        self.expiration_tooltip.as_ref()
    }

    pub fn expired_in_activity_message(&self) -> Option<&String> {
        self.expired_in_activity_message.as_ref()
    }

    pub fn expired_in_orbit_message(&self) -> Option<&String> {
        self.expired_in_orbit_message.as_ref()
    }

    pub fn is_instance_item(&self) -> Option<bool> {
        self.is_instance_item
    }

    pub fn max_stack_size(&self) -> Option<i32> {
        self.max_stack_size
    }

    pub fn recipe_item_hash(&self) -> Option<u32> {
        self.recipe_item_hash
    }

    pub fn recovery_bucket_type_hash(&self) -> Option<u32> {
        self.recovery_bucket_type_hash
    }

    pub fn stack_unique_label(&self) -> Option<&String> {
        self.stack_unique_label.as_ref()
    }

    pub fn suppress_expiration_when_objectives_complete(&self) -> Option<bool> {
        self.suppress_expiration_when_objectives_complete
    }

    pub fn tier_type(&self) -> Option<i32> {
        self.tier_type
    }

    pub fn tier_type_hash(&self) -> Option<u32> {
        self.tier_type_hash
    }

    pub fn tier_type_name(&self) -> Option<&String> {
        self.tier_type_name.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemInvestmentStatDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemInvestmentStatDefinition {
    is_conditionally_active: Option<bool>,
    stat_type_hash: Option<u32>,
    value: Option<i32>,
}

impl DestinyItemInvestmentStatDefinition {
    pub fn is_conditionally_active(&self) -> Option<bool> {
        self.is_conditionally_active
    }

    pub fn stat_type_hash(&self) -> Option<u32> {
        self.stat_type_hash
    }

    pub fn value(&self) -> Option<i32> {
        self.value
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemMetricBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemMetricBlockDefinition {
    available_metric_category_node_hashes: Option<Vec<u32>>,
}

impl DestinyItemMetricBlockDefinition {
    pub fn available_metric_category_node_hashes(&self) -> Option<&Vec<u32>> {
        self.available_metric_category_node_hashes.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemObjectiveBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemObjectiveBlockDefinition {
    display_activity_hashes: Option<Vec<u32>>,
    display_as_stat_tracker: Option<bool>,
    narrative: Option<String>,
    objective_hashes: Option<Vec<u32>>,
    objective_verb_name: Option<String>,
    per_objective_display_properties: Option<Vec<DestinyObjectiveDisplayProperties>>,
    quest_type_hash: Option<u32>,
    quest_type_identifier: Option<String>,
    questline_item_hash: Option<u32>,
    require_full_objective_completion: Option<bool>,
}

impl DestinyItemObjectiveBlockDefinition {
    pub fn display_activity_hashes(&self) -> Option<&Vec<u32>> {
        self.display_activity_hashes.as_ref()
    }

    pub fn display_as_stat_tracker(&self) -> Option<bool> {
        self.display_as_stat_tracker
    }

    pub fn narrative(&self) -> Option<&String> {
        self.narrative.as_ref()
    }

    pub fn objective_hashes(&self) -> Option<&Vec<u32>> {
        self.objective_hashes.as_ref()
    }

    pub fn objective_verb_name(&self) -> Option<&String> {
        self.objective_verb_name.as_ref()
    }

    pub fn per_objective_display_properties(
        &self,
    ) -> Option<&Vec<DestinyObjectiveDisplayProperties>> {
        self.per_objective_display_properties.as_ref()
    }

    pub fn quest_type_hash(&self) -> Option<u32> {
        self.quest_type_hash
    }

    pub fn quest_type_identifier(&self) -> Option<&String> {
        self.quest_type_identifier.as_ref()
    }

    pub fn questline_item_hash(&self) -> Option<u32> {
        self.questline_item_hash
    }

    pub fn require_full_objective_completion(&self) -> Option<bool> {
        self.require_full_objective_completion
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemPerkEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemPerkEntryDefinition {
    perk_hash: Option<u32>,
    perk_visibility: Option<i32>,
    requirement_display_string: Option<String>,
}

impl DestinyItemPerkEntryDefinition {
    pub fn perk_hash(&self) -> Option<u32> {
        self.perk_hash
    }

    pub fn perk_visibility(&self) -> Option<i32> {
        self.perk_visibility
    }

    pub fn requirement_display_string(&self) -> Option<&String> {
        self.requirement_display_string.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemPreviewBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemPreviewBlockDefinition {
    artifact_hash: Option<u32>,
    derived_item_categories: Option<Vec<DestinyDerivedItemCategoryDefinition>>,
    preview_action_string: Option<String>,
    preview_vendor_hash: Option<u32>,
    screen_style: Option<String>,
}

impl DestinyItemPreviewBlockDefinition {
    pub fn artifact_hash(&self) -> Option<u32> {
        self.artifact_hash
    }

    pub fn derived_item_categories(&self) -> Option<&Vec<DestinyDerivedItemCategoryDefinition>> {
        self.derived_item_categories.as_ref()
    }

    pub fn preview_action_string(&self) -> Option<&String> {
        self.preview_action_string.as_ref()
    }

    pub fn preview_vendor_hash(&self) -> Option<u32> {
        self.preview_vendor_hash
    }

    pub fn screen_style(&self) -> Option<&String> {
        self.screen_style.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemQualityBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemQualityBlockDefinition {
    current_version: Option<u32>,
    display_version_watermark_icons: Option<Vec<String>>,
    infusion_category_hash: Option<u32>,
    infusion_category_hashes: Option<Vec<u32>>,
    infusion_category_name: Option<String>,
    item_levels: Option<Vec<i32>>,
    progression_level_requirement: Option<u32>,
    quality_level: Option<i32>,
    versions: Option<Vec<DestinyItemVersionDefinition>>,
}

impl DestinyItemQualityBlockDefinition {
    pub fn current_version(&self) -> Option<u32> {
        self.current_version
    }

    pub fn display_version_watermark_icons(&self) -> Option<&Vec<String>> {
        self.display_version_watermark_icons.as_ref()
    }

    pub fn infusion_category_hash(&self) -> Option<u32> {
        self.infusion_category_hash
    }

    pub fn infusion_category_hashes(&self) -> Option<&Vec<u32>> {
        self.infusion_category_hashes.as_ref()
    }

    pub fn infusion_category_name(&self) -> Option<&String> {
        self.infusion_category_name.as_ref()
    }

    pub fn item_levels(&self) -> Option<&Vec<i32>> {
        self.item_levels.as_ref()
    }

    pub fn progression_level_requirement(&self) -> Option<u32> {
        self.progression_level_requirement
    }

    pub fn quality_level(&self) -> Option<i32> {
        self.quality_level
    }

    pub fn versions(&self) -> Option<&Vec<DestinyItemVersionDefinition>> {
        self.versions.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSackBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSackBlockDefinition {
    detail_action: Option<String>,
    open_action: Option<String>,
    open_on_acquire: Option<bool>,
    select_item_count: Option<i32>,
    vendor_sack_type: Option<String>,
}

impl DestinyItemSackBlockDefinition {
    pub fn detail_action(&self) -> Option<&String> {
        self.detail_action.as_ref()
    }

    pub fn open_action(&self) -> Option<&String> {
        self.open_action.as_ref()
    }

    pub fn open_on_acquire(&self) -> Option<bool> {
        self.open_on_acquire
    }

    pub fn select_item_count(&self) -> Option<i32> {
        self.select_item_count
    }

    pub fn vendor_sack_type(&self) -> Option<&String> {
        self.vendor_sack_type.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSetBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSetBlockDefinition {
    item_list: Option<Vec<DestinyItemSetBlockEntryDefinition>>,
    quest_line_description: Option<String>,
    quest_line_name: Option<String>,
    quest_step_summary: Option<String>,
    require_ordered_set_item_add: Option<bool>,
    set_is_featured: Option<bool>,
    set_type: Option<String>,
}

impl DestinyItemSetBlockDefinition {
    pub fn item_list(&self) -> Option<&Vec<DestinyItemSetBlockEntryDefinition>> {
        self.item_list.as_ref()
    }

    pub fn quest_line_description(&self) -> Option<&String> {
        self.quest_line_description.as_ref()
    }

    pub fn quest_line_name(&self) -> Option<&String> {
        self.quest_line_name.as_ref()
    }

    pub fn quest_step_summary(&self) -> Option<&String> {
        self.quest_step_summary.as_ref()
    }

    pub fn require_ordered_set_item_add(&self) -> Option<bool> {
        self.require_ordered_set_item_add
    }

    pub fn set_is_featured(&self) -> Option<bool> {
        self.set_is_featured
    }

    pub fn set_type(&self) -> Option<&String> {
        self.set_type.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSetBlockEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSetBlockEntryDefinition {
    item_hash: Option<u32>,
    tracking_value: Option<i32>,
}

impl DestinyItemSetBlockEntryDefinition {
    pub fn item_hash(&self) -> Option<u32> {
        self.item_hash
    }

    pub fn tracking_value(&self) -> Option<i32> {
        self.tracking_value
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSocketBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSocketBlockDefinition {
    detail: Option<String>,
    intrinsic_sockets: Option<Vec<DestinyItemIntrinsicSocketEntryDefinition>>,
    socket_categories: Option<Vec<DestinyItemSocketCategoryDefinition>>,
    socket_entries: Option<Vec<DestinyItemSocketEntryDefinition>>,
}

impl DestinyItemSocketBlockDefinition {
    pub fn detail(&self) -> Option<&String> {
        self.detail.as_ref()
    }

    pub fn intrinsic_sockets(&self) -> Option<&Vec<DestinyItemIntrinsicSocketEntryDefinition>> {
        self.intrinsic_sockets.as_ref()
    }

    pub fn socket_categories(&self) -> Option<&Vec<DestinyItemSocketCategoryDefinition>> {
        self.socket_categories.as_ref()
    }

    pub fn socket_entries(&self) -> Option<&Vec<DestinyItemSocketEntryDefinition>> {
        self.socket_entries.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSocketCategoryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSocketCategoryDefinition {
    socket_category_hash: Option<u32>,
    socket_indexes: Option<Vec<i32>>,
}

impl DestinyItemSocketCategoryDefinition {
    pub fn socket_category_hash(&self) -> Option<u32> {
        self.socket_category_hash
    }

    pub fn socket_indexes(&self) -> Option<&Vec<i32>> {
        self.socket_indexes.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSocketEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSocketEntryDefinition {
    default_visible: Option<bool>,
    hide_perks_in_item_tooltip: Option<bool>,
    plug_sources: Option<i32>,
    prevent_initialization_on_vendor_purchase: Option<bool>,
    randomized_plug_set_hash: Option<u32>,
    reusable_plug_items: Option<Vec<DestinyItemSocketEntryPlugItemDefinition>>,
    reusable_plug_set_hash: Option<u32>,
    single_initial_item_hash: Option<u32>,
    socket_type_hash: Option<u32>,
}

impl DestinyItemSocketEntryDefinition {
    pub fn default_visible(&self) -> Option<bool> {
        self.default_visible
    }

    pub fn hide_perks_in_item_tooltip(&self) -> Option<bool> {
        self.hide_perks_in_item_tooltip
    }

    pub fn plug_sources(&self) -> Option<i32> {
        self.plug_sources
    }

    pub fn prevent_initialization_on_vendor_purchase(&self) -> Option<bool> {
        self.prevent_initialization_on_vendor_purchase
    }

    pub fn randomized_plug_set_hash(&self) -> Option<u32> {
        self.randomized_plug_set_hash
    }

    pub fn reusable_plug_items(&self) -> Option<&Vec<DestinyItemSocketEntryPlugItemDefinition>> {
        self.reusable_plug_items.as_ref()
    }

    pub fn reusable_plug_set_hash(&self) -> Option<u32> {
        self.reusable_plug_set_hash
    }

    pub fn single_initial_item_hash(&self) -> Option<u32> {
        self.single_initial_item_hash
    }

    pub fn socket_type_hash(&self) -> Option<u32> {
        self.socket_type_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSocketEntryPlugItemDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSocketEntryPlugItemDefinition {
    plug_item_hash: Option<u32>,
}

impl DestinyItemSocketEntryPlugItemDefinition {
    pub fn plug_item_hash(&self) -> Option<u32> {
        self.plug_item_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSocketEntryPlugItemRandomizedDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSocketEntryPlugItemRandomizedDefinition {
    crafting_requirements: Option<DestinyPlugItemCraftingRequirements>,
    currently_can_roll: Option<bool>,
    plug_item_hash: Option<u32>,
}

impl DestinyItemSocketEntryPlugItemRandomizedDefinition {
    pub fn crafting_requirements(&self) -> Option<&DestinyPlugItemCraftingRequirements> {
        self.crafting_requirements.as_ref()
    }

    pub fn currently_can_roll(&self) -> Option<bool> {
        self.currently_can_roll
    }

    pub fn plug_item_hash(&self) -> Option<u32> {
        self.plug_item_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSourceBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSourceBlockDefinition {
    exclusive: Option<i32>,
    source_hashes: Option<Vec<u32>>,
    sources: Option<Vec<DestinyItemSourceDefinition>>,
    vendor_sources: Option<Vec<DestinyItemVendorSourceReference>>,
}

impl DestinyItemSourceBlockDefinition {
    pub fn exclusive(&self) -> Option<i32> {
        self.exclusive
    }

    pub fn source_hashes(&self) -> Option<&Vec<u32>> {
        self.source_hashes.as_ref()
    }

    pub fn sources(&self) -> Option<&Vec<DestinyItemSourceDefinition>> {
        self.sources.as_ref()
    }

    pub fn vendor_sources(&self) -> Option<&Vec<DestinyItemVendorSourceReference>> {
        self.vendor_sources.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemStatBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemStatBlockDefinition {
    disable_primary_stat_display: Option<bool>,
    has_displayable_stats: Option<bool>,
    primary_base_stat_hash: Option<u32>,
    stat_group_hash: Option<u32>,
    stats: Option<HashMap<u32, DestinyInventoryItemStatDefinition>>,
}

impl DestinyItemStatBlockDefinition {
    pub fn disable_primary_stat_display(&self) -> Option<bool> {
        self.disable_primary_stat_display
    }

    pub fn has_displayable_stats(&self) -> Option<bool> {
        self.has_displayable_stats
    }

    pub fn primary_base_stat_hash(&self) -> Option<u32> {
        self.primary_base_stat_hash
    }

    pub fn stat_group_hash(&self) -> Option<u32> {
        self.stat_group_hash
    }

    pub fn stats(&self) -> Option<&HashMap<u32, DestinyInventoryItemStatDefinition>> {
        self.stats.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemSummaryBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemSummaryBlockDefinition {
    sort_priority: Option<i32>,
}

impl DestinyItemSummaryBlockDefinition {
    pub fn sort_priority(&self) -> Option<i32> {
        self.sort_priority
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemTalentGridBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemTalentGridBlockDefinition {
    build_name: Option<String>,
    hud_damage_type: Option<i32>,
    hud_icon: Option<String>,
    item_detail_string: Option<String>,
    talent_grid_hash: Option<u32>,
}

impl DestinyItemTalentGridBlockDefinition {
    pub fn build_name(&self) -> Option<&String> {
        self.build_name.as_ref()
    }

    pub fn hud_damage_type(&self) -> Option<i32> {
        self.hud_damage_type
    }

    pub fn hud_icon(&self) -> Option<&String> {
        self.hud_icon.as_ref()
    }

    pub fn item_detail_string(&self) -> Option<&String> {
        self.item_detail_string.as_ref()
    }

    pub fn talent_grid_hash(&self) -> Option<u32> {
        self.talent_grid_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemTooltipNotification
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemTooltipNotification {
    display_string: Option<String>,
    display_style: Option<String>,
}

impl DestinyItemTooltipNotification {
    pub fn display_string(&self) -> Option<&String> {
        self.display_string.as_ref()
    }

    pub fn display_style(&self) -> Option<&String> {
        self.display_style.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemTranslationBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemTranslationBlockDefinition {
    arrangements: Option<Vec<DestinyGearArtArrangementReference>>,
    custom_dyes: Option<Vec<DyeReference>>,
    default_dyes: Option<Vec<DyeReference>>,
    has_geometry: Option<bool>,
    locked_dyes: Option<Vec<DyeReference>>,
    weapon_pattern_hash: Option<u32>,
    weapon_pattern_identifier: Option<String>,
}

impl DestinyItemTranslationBlockDefinition {
    pub fn arrangements(&self) -> Option<&Vec<DestinyGearArtArrangementReference>> {
        self.arrangements.as_ref()
    }

    pub fn custom_dyes(&self) -> Option<&Vec<DyeReference>> {
        self.custom_dyes.as_ref()
    }

    pub fn default_dyes(&self) -> Option<&Vec<DyeReference>> {
        self.default_dyes.as_ref()
    }

    pub fn has_geometry(&self) -> Option<bool> {
        self.has_geometry
    }

    pub fn locked_dyes(&self) -> Option<&Vec<DyeReference>> {
        self.locked_dyes.as_ref()
    }

    pub fn weapon_pattern_hash(&self) -> Option<u32> {
        self.weapon_pattern_hash
    }

    pub fn weapon_pattern_identifier(&self) -> Option<&String> {
        self.weapon_pattern_identifier.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemValueBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemValueBlockDefinition {
    item_value: Option<Vec<DestinyItemQuantity>>,
    value_description: Option<String>,
}

impl DestinyItemValueBlockDefinition {
    pub fn item_value(&self) -> Option<&Vec<DestinyItemQuantity>> {
        self.item_value.as_ref()
    }

    pub fn value_description(&self) -> Option<&String> {
        self.value_description.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemVendorSourceReference
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemVendorSourceReference {
    vendor_hash: Option<u32>,
    vendor_item_indexes: Option<Vec<i32>>,
}

impl DestinyItemVendorSourceReference {
    pub fn vendor_hash(&self) -> Option<u32> {
        self.vendor_hash
    }

    pub fn vendor_item_indexes(&self) -> Option<&Vec<i32>> {
        self.vendor_item_indexes.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemVersionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemVersionDefinition {
    power_cap_hash: Option<u32>,
}

impl DestinyItemVersionDefinition {
    pub fn power_cap_hash(&self) -> Option<u32> {
        self.power_cap_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyLocationDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyLocationDefinition {
    hash: Option<u32>,
    index: Option<i32>,
    location_releases: Option<Vec<DestinyLocationReleaseDefinition>>,
    redacted: Option<bool>,
    vendor_hash: Option<u32>,
}

impl DestinyLocationDefinition {
    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn location_releases(&self) -> Option<&Vec<DestinyLocationReleaseDefinition>> {
        self.location_releases.as_ref()
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn vendor_hash(&self) -> Option<u32> {
        self.vendor_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyLocationReleaseDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyLocationReleaseDefinition {
    activity_bubble_name: Option<u32>,
    activity_graph_hash: Option<u32>,
    activity_graph_node_hash: Option<u32>,
    activity_hash: Option<u32>,
    activity_path_bundle: Option<u32>,
    activity_path_destination: Option<u32>,
    destination_hash: Option<u32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    large_transparent_icon: Option<String>,
    map_icon: Option<String>,
    nav_point_type: Option<i32>,
    small_transparent_icon: Option<String>,
    spawn_point: Option<u32>,
    world_position: Option<Vec<i32>>,
}

impl DestinyLocationReleaseDefinition {
    pub fn activity_bubble_name(&self) -> Option<u32> {
        self.activity_bubble_name
    }

    pub fn activity_graph_hash(&self) -> Option<u32> {
        self.activity_graph_hash
    }

    pub fn activity_graph_node_hash(&self) -> Option<u32> {
        self.activity_graph_node_hash
    }

    pub fn activity_hash(&self) -> Option<u32> {
        self.activity_hash
    }

    pub fn activity_path_bundle(&self) -> Option<u32> {
        self.activity_path_bundle
    }

    pub fn activity_path_destination(&self) -> Option<u32> {
        self.activity_path_destination
    }

    pub fn destination_hash(&self) -> Option<u32> {
        self.destination_hash
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn large_transparent_icon(&self) -> Option<&String> {
        self.large_transparent_icon.as_ref()
    }

    pub fn map_icon(&self) -> Option<&String> {
        self.map_icon.as_ref()
    }

    pub fn nav_point_type(&self) -> Option<i32> {
        self.nav_point_type
    }

    pub fn small_transparent_icon(&self) -> Option<&String> {
        self.small_transparent_icon.as_ref()
    }

    pub fn spawn_point(&self) -> Option<u32> {
        self.spawn_point
    }

    pub fn world_position(&self) -> Option<&Vec<i32>> {
        self.world_position.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyMaterialRequirement
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMaterialRequirement {
    count: Option<i32>,
    count_is_constant: Option<bool>,
    delete_on_action: Option<bool>,
    item_hash: Option<u32>,
    omit_from_requirements: Option<bool>,
}

impl DestinyMaterialRequirement {
    pub fn count(&self) -> Option<i32> {
        self.count
    }

    pub fn count_is_constant(&self) -> Option<bool> {
        self.count_is_constant
    }

    pub fn delete_on_action(&self) -> Option<bool> {
        self.delete_on_action
    }

    pub fn item_hash(&self) -> Option<u32> {
        self.item_hash
    }

    pub fn omit_from_requirements(&self) -> Option<bool> {
        self.omit_from_requirements
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyMaterialRequirementSetDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMaterialRequirementSetDefinition {
    hash: Option<u32>,
    index: Option<i32>,
    materials: Option<Vec<DestinyMaterialRequirement>>,
    redacted: Option<bool>,
}

impl DestinyMaterialRequirementSetDefinition {
    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn materials(&self) -> Option<&Vec<DestinyMaterialRequirement>> {
        self.materials.as_ref()
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyMedalTierDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMedalTierDefinition {
    hash: Option<u32>,
    index: Option<i32>,
    order: Option<i32>,
    redacted: Option<bool>,
    tier_name: Option<String>,
}

impl DestinyMedalTierDefinition {
    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn order(&self) -> Option<i32> {
        self.order
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn tier_name(&self) -> Option<&String> {
        self.tier_name.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyNodeActivationRequirement
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyNodeActivationRequirement {
    grid_level: Option<i32>,
    material_requirement_hashes: Option<Vec<u32>>,
}

impl DestinyNodeActivationRequirement {
    pub fn grid_level(&self) -> Option<i32> {
        self.grid_level
    }

    pub fn material_requirement_hashes(&self) -> Option<&Vec<u32>> {
        self.material_requirement_hashes.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyNodeSocketReplaceResponse
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyNodeSocketReplaceResponse {
    plug_item_hash: Option<u32>,
    socket_type_hash: Option<u32>,
}

impl DestinyNodeSocketReplaceResponse {
    pub fn plug_item_hash(&self) -> Option<u32> {
        self.plug_item_hash
    }

    pub fn socket_type_hash(&self) -> Option<u32> {
        self.socket_type_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyNodeStepDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyNodeStepDefinition {
    activation_requirement: Option<DestinyNodeActivationRequirement>,
    affects_level: Option<bool>,
    affects_quality: Option<bool>,
    can_activate_next_step: Option<bool>,
    damage_type: Option<i32>,
    damage_type_hash: Option<u32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    interaction_description: Option<String>,
    is_next_step_random: Option<bool>,
    next_step_index: Option<i32>,
    node_step_hash: Option<u32>,
    perk_hashes: Option<Vec<u32>>,
    socket_replacements: Option<Vec<DestinyNodeSocketReplaceResponse>>,
    start_progression_bar_at_progress: Option<i32>,
    stat_hashes: Option<Vec<u32>>,
    step_groups: Option<DestinyTalentNodeStepGroups>,
    step_index: Option<i32>,
}

impl DestinyNodeStepDefinition {
    pub fn activation_requirement(&self) -> Option<&DestinyNodeActivationRequirement> {
        self.activation_requirement.as_ref()
    }

    pub fn affects_level(&self) -> Option<bool> {
        self.affects_level
    }

    pub fn affects_quality(&self) -> Option<bool> {
        self.affects_quality
    }

    pub fn can_activate_next_step(&self) -> Option<bool> {
        self.can_activate_next_step
    }

    pub fn damage_type(&self) -> Option<i32> {
        self.damage_type
    }

    pub fn damage_type_hash(&self) -> Option<u32> {
        self.damage_type_hash
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn interaction_description(&self) -> Option<&String> {
        self.interaction_description.as_ref()
    }

    pub fn is_next_step_random(&self) -> Option<bool> {
        self.is_next_step_random
    }

    pub fn next_step_index(&self) -> Option<i32> {
        self.next_step_index
    }

    pub fn node_step_hash(&self) -> Option<u32> {
        self.node_step_hash
    }

    pub fn perk_hashes(&self) -> Option<&Vec<u32>> {
        self.perk_hashes.as_ref()
    }

    pub fn socket_replacements(&self) -> Option<&Vec<DestinyNodeSocketReplaceResponse>> {
        self.socket_replacements.as_ref()
    }

    pub fn start_progression_bar_at_progress(&self) -> Option<i32> {
        self.start_progression_bar_at_progress
    }

    pub fn stat_hashes(&self) -> Option<&Vec<u32>> {
        self.stat_hashes.as_ref()
    }

    pub fn step_groups(&self) -> Option<&DestinyTalentNodeStepGroups> {
        self.step_groups.as_ref()
    }

    pub fn step_index(&self) -> Option<i32> {
        self.step_index
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyObjectiveDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyObjectiveDefinition {
    allow_negative_value: Option<bool>,
    allow_overcompletion: Option<bool>,
    allow_value_change_when_completed: Option<bool>,
    completed_value_style: Option<i32>,
    completion_value: Option<i32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    in_progress_value_style: Option<i32>,
    index: Option<i32>,
    is_counting_downward: Option<bool>,
    location_hash: Option<u32>,
    minimum_visibility_threshold: Option<i32>,
    perks: Option<DestinyObjectivePerkEntryDefinition>,
    progress_description: Option<String>,
    redacted: Option<bool>,
    scope: Option<i32>,
    show_value_on_complete: Option<bool>,
    stats: Option<DestinyObjectiveStatEntryDefinition>,
    ui_label: Option<String>,
    ui_style: Option<i32>,
    value_style: Option<i32>,
}

impl DestinyObjectiveDefinition {
    pub fn allow_negative_value(&self) -> Option<bool> {
        self.allow_negative_value
    }

    pub fn allow_overcompletion(&self) -> Option<bool> {
        self.allow_overcompletion
    }

    pub fn allow_value_change_when_completed(&self) -> Option<bool> {
        self.allow_value_change_when_completed
    }

    pub fn completed_value_style(&self) -> Option<i32> {
        self.completed_value_style
    }

    pub fn completion_value(&self) -> Option<i32> {
        self.completion_value
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn in_progress_value_style(&self) -> Option<i32> {
        self.in_progress_value_style
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn is_counting_downward(&self) -> Option<bool> {
        self.is_counting_downward
    }

    pub fn location_hash(&self) -> Option<u32> {
        self.location_hash
    }

    pub fn minimum_visibility_threshold(&self) -> Option<i32> {
        self.minimum_visibility_threshold
    }

    pub fn perks(&self) -> Option<&DestinyObjectivePerkEntryDefinition> {
        self.perks.as_ref()
    }

    pub fn progress_description(&self) -> Option<&String> {
        self.progress_description.as_ref()
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn scope(&self) -> Option<i32> {
        self.scope
    }

    pub fn show_value_on_complete(&self) -> Option<bool> {
        self.show_value_on_complete
    }

    pub fn stats(&self) -> Option<&DestinyObjectiveStatEntryDefinition> {
        self.stats.as_ref()
    }

    pub fn ui_label(&self) -> Option<&String> {
        self.ui_label.as_ref()
    }

    pub fn ui_style(&self) -> Option<i32> {
        self.ui_style
    }

    pub fn value_style(&self) -> Option<i32> {
        self.value_style
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyObjectiveDisplayProperties
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyObjectiveDisplayProperties {
    activity_hash: Option<u32>,
    display_on_item_preview_screen: Option<bool>,
}

impl DestinyObjectiveDisplayProperties {
    pub fn activity_hash(&self) -> Option<u32> {
        self.activity_hash
    }

    pub fn display_on_item_preview_screen(&self) -> Option<bool> {
        self.display_on_item_preview_screen
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyObjectivePerkEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyObjectivePerkEntryDefinition {
    perk_hash: Option<u32>,
    style: Option<i32>,
}

impl DestinyObjectivePerkEntryDefinition {
    pub fn perk_hash(&self) -> Option<u32> {
        self.perk_hash
    }

    pub fn style(&self) -> Option<i32> {
        self.style
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyObjectiveStatEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyObjectiveStatEntryDefinition {
    stat: Option<DestinyItemInvestmentStatDefinition>,
    style: Option<i32>,
}

impl DestinyObjectiveStatEntryDefinition {
    pub fn stat(&self) -> Option<&DestinyItemInvestmentStatDefinition> {
        self.stat.as_ref()
    }

    pub fn style(&self) -> Option<i32> {
        self.style
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyPlaceDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlaceDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
}

impl DestinyPlaceDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyPlugItemCraftingRequirements
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlugItemCraftingRequirements {
    material_requirement_hashes: Option<Vec<u32>>,
    required_level: Option<i32>,
    unlock_requirements: Option<Vec<DestinyPlugItemCraftingUnlockRequirement>>,
}

impl DestinyPlugItemCraftingRequirements {
    pub fn material_requirement_hashes(&self) -> Option<&Vec<u32>> {
        self.material_requirement_hashes.as_ref()
    }

    pub fn required_level(&self) -> Option<i32> {
        self.required_level
    }

    pub fn unlock_requirements(&self) -> Option<&Vec<DestinyPlugItemCraftingUnlockRequirement>> {
        self.unlock_requirements.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyPlugItemCraftingUnlockRequirement
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlugItemCraftingUnlockRequirement {
    failure_description: Option<String>,
}

impl DestinyPlugItemCraftingUnlockRequirement {
    pub fn failure_description(&self) -> Option<&String> {
        self.failure_description.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyProgressionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProgressionDefinition {
    color: Option<DestinyColor>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    faction_hash: Option<u32>,
    hash: Option<u32>,
    index: Option<i32>,
    rank_icon: Option<String>,
    redacted: Option<bool>,
    repeat_last_step: Option<bool>,
    reward_items: Option<Vec<DestinyProgressionRewardItemQuantity>>,
    scope: Option<i32>,
    source: Option<String>,
    steps: Option<Vec<DestinyProgressionStepDefinition>>,
    visible: Option<bool>,
}

impl DestinyProgressionDefinition {
    pub fn color(&self) -> Option<&DestinyColor> {
        self.color.as_ref()
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn faction_hash(&self) -> Option<u32> {
        self.faction_hash
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn rank_icon(&self) -> Option<&String> {
        self.rank_icon.as_ref()
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn repeat_last_step(&self) -> Option<bool> {
        self.repeat_last_step
    }

    pub fn reward_items(&self) -> Option<&Vec<DestinyProgressionRewardItemQuantity>> {
        self.reward_items.as_ref()
    }

    pub fn scope(&self) -> Option<i32> {
        self.scope
    }

    pub fn source(&self) -> Option<&String> {
        self.source.as_ref()
    }

    pub fn steps(&self) -> Option<&Vec<DestinyProgressionStepDefinition>> {
        self.steps.as_ref()
    }

    pub fn visible(&self) -> Option<bool> {
        self.visible
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyProgressionMappingDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProgressionMappingDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    display_units: Option<String>,
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
}

impl DestinyProgressionMappingDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn display_units(&self) -> Option<&String> {
        self.display_units.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyProgressionRewardDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProgressionRewardDefinition {
    amount: Option<i32>,
    apply_throttles: Option<bool>,
    progression_mapping_hash: Option<u32>,
}

impl DestinyProgressionRewardDefinition {
    pub fn amount(&self) -> Option<i32> {
        self.amount
    }

    pub fn apply_throttles(&self) -> Option<bool> {
        self.apply_throttles
    }

    pub fn progression_mapping_hash(&self) -> Option<u32> {
        self.progression_mapping_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyProgressionRewardItemQuantity
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProgressionRewardItemQuantity {
    acquisition_behavior: Option<i32>,
    claim_unlock_display_strings: Option<Vec<String>>,
    has_conditional_visibility: Option<bool>,
    item_hash: Option<u32>,
    item_instance_id: Option<i64>,
    quantity: Option<i32>,
    rewarded_at_progression_level: Option<i32>,
    ui_display_style: Option<String>,
}

impl DestinyProgressionRewardItemQuantity {
    pub fn acquisition_behavior(&self) -> Option<i32> {
        self.acquisition_behavior
    }

    pub fn claim_unlock_display_strings(&self) -> Option<&Vec<String>> {
        self.claim_unlock_display_strings.as_ref()
    }

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

    pub fn rewarded_at_progression_level(&self) -> Option<i32> {
        self.rewarded_at_progression_level
    }

    pub fn ui_display_style(&self) -> Option<&String> {
        self.ui_display_style.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyProgressionStepDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProgressionStepDefinition {
    display_effect_type: Option<i32>,
    icon: Option<String>,
    progress_total: Option<i32>,
    reward_items: Option<Vec<DestinyItemQuantity>>,
    step_name: Option<String>,
}

impl DestinyProgressionStepDefinition {
    pub fn display_effect_type(&self) -> Option<i32> {
        self.display_effect_type
    }

    pub fn icon(&self) -> Option<&String> {
        self.icon.as_ref()
    }

    pub fn progress_total(&self) -> Option<i32> {
        self.progress_total
    }

    pub fn reward_items(&self) -> Option<&Vec<DestinyItemQuantity>> {
        self.reward_items.as_ref()
    }

    pub fn step_name(&self) -> Option<&String> {
        self.step_name.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyRaceDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRaceDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    gendered_race_names: Option<HashMap<DestinyGender, String>>,
    gendered_race_names_by_gender_hash: Option<HashMap<u32, String>>,
    hash: Option<u32>,
    index: Option<i32>,
    race_type: Option<i32>,
    redacted: Option<bool>,
}

impl DestinyRaceDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn gendered_race_names(&self) -> Option<&HashMap<DestinyGender, String>> {
        self.gendered_race_names.as_ref()
    }

    pub fn gendered_race_names_by_gender_hash(&self) -> Option<&HashMap<u32, String>> {
        self.gendered_race_names_by_gender_hash.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn race_type(&self) -> Option<i32> {
        self.race_type
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyRewardSourceDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyRewardSourceDefinition {
    category: Option<i32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
}

impl DestinyRewardSourceDefinition {
    pub fn category(&self) -> Option<i32> {
        self.category
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinySandboxPatternDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySandboxPatternDefinition {
    filters: Option<Vec<DestinyArrangementRegionFilterDefinition>>,
    hash: Option<u32>,
    index: Option<i32>,
    pattern_global_tag_id_hash: Option<u32>,
    pattern_hash: Option<u32>,
    redacted: Option<bool>,
    weapon_content_group_hash: Option<u32>,
    weapon_translation_group_hash: Option<u32>,
    weapon_type: Option<i32>,
    weapon_type_hash: Option<u32>,
}

impl DestinySandboxPatternDefinition {
    pub fn filters(&self) -> Option<&Vec<DestinyArrangementRegionFilterDefinition>> {
        self.filters.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn pattern_global_tag_id_hash(&self) -> Option<u32> {
        self.pattern_global_tag_id_hash
    }

    pub fn pattern_hash(&self) -> Option<u32> {
        self.pattern_hash
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn weapon_content_group_hash(&self) -> Option<u32> {
        self.weapon_content_group_hash
    }

    pub fn weapon_translation_group_hash(&self) -> Option<u32> {
        self.weapon_translation_group_hash
    }

    pub fn weapon_type(&self) -> Option<i32> {
        self.weapon_type
    }

    pub fn weapon_type_hash(&self) -> Option<u32> {
        self.weapon_type_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinySandboxPerkDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySandboxPerkDefinition {
    damage_type: Option<i32>,
    damage_type_hash: Option<u32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    is_displayable: Option<bool>,
    perk_groups: Option<DestinyTalentNodeStepGroups>,
    perk_identifier: Option<String>,
    redacted: Option<bool>,
}

impl DestinySandboxPerkDefinition {
    pub fn damage_type(&self) -> Option<i32> {
        self.damage_type
    }

    pub fn damage_type_hash(&self) -> Option<u32> {
        self.damage_type_hash
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn is_displayable(&self) -> Option<bool> {
        self.is_displayable
    }

    pub fn perk_groups(&self) -> Option<&DestinyTalentNodeStepGroups> {
        self.perk_groups.as_ref()
    }

    pub fn perk_identifier(&self) -> Option<&String> {
        self.perk_identifier.as_ref()
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyStatDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyStatDefinition {
    aggregation_type: Option<i32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    has_computed_block: Option<bool>,
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
    stat_category: Option<i32>,
}

impl DestinyStatDefinition {
    pub fn aggregation_type(&self) -> Option<i32> {
        self.aggregation_type
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn has_computed_block(&self) -> Option<bool> {
        self.has_computed_block
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn stat_category(&self) -> Option<i32> {
        self.stat_category
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyStatDisplayDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyStatDisplayDefinition {
    display_as_numeric: Option<bool>,
    display_interpolation: Option<Vec<InterpolationPoint>>,
    maximum_value: Option<i32>,
    stat_hash: Option<u32>,
}

impl DestinyStatDisplayDefinition {
    pub fn display_as_numeric(&self) -> Option<bool> {
        self.display_as_numeric
    }

    pub fn display_interpolation(&self) -> Option<&Vec<InterpolationPoint>> {
        self.display_interpolation.as_ref()
    }

    pub fn maximum_value(&self) -> Option<i32> {
        self.maximum_value
    }

    pub fn stat_hash(&self) -> Option<u32> {
        self.stat_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyStatGroupDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyStatGroupDefinition {
    hash: Option<u32>,
    index: Option<i32>,
    maximum_value: Option<i32>,
    overrides: Option<HashMap<u32, DestinyStatOverrideDefinition>>,
    redacted: Option<bool>,
    scaled_stats: Option<Vec<DestinyStatDisplayDefinition>>,
    ui_position: Option<i32>,
}

impl DestinyStatGroupDefinition {
    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn maximum_value(&self) -> Option<i32> {
        self.maximum_value
    }

    pub fn overrides(&self) -> Option<&HashMap<u32, DestinyStatOverrideDefinition>> {
        self.overrides.as_ref()
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn scaled_stats(&self) -> Option<&Vec<DestinyStatDisplayDefinition>> {
        self.scaled_stats.as_ref()
    }

    pub fn ui_position(&self) -> Option<i32> {
        self.ui_position
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyStatOverrideDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyStatOverrideDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    stat_hash: Option<u32>,
}

impl DestinyStatOverrideDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn stat_hash(&self) -> Option<u32> {
        self.stat_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyTalentExclusiveGroup
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyTalentExclusiveGroup {
    group_hash: Option<u32>,
    lore_hash: Option<u32>,
    node_hashes: Option<Vec<u32>>,
    opposing_group_hashes: Option<Vec<u32>>,
    opposing_node_hashes: Option<Vec<u32>>,
}

impl DestinyTalentExclusiveGroup {
    pub fn group_hash(&self) -> Option<u32> {
        self.group_hash
    }

    pub fn lore_hash(&self) -> Option<u32> {
        self.lore_hash
    }

    pub fn node_hashes(&self) -> Option<&Vec<u32>> {
        self.node_hashes.as_ref()
    }

    pub fn opposing_group_hashes(&self) -> Option<&Vec<u32>> {
        self.opposing_group_hashes.as_ref()
    }

    pub fn opposing_node_hashes(&self) -> Option<&Vec<u32>> {
        self.opposing_node_hashes.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyTalentGridDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyTalentGridDefinition {
    exclusive_sets: Option<Vec<DestinyTalentNodeExclusiveSetDefinition>>,
    grid_level_per_column: Option<i32>,
    groups: Option<HashMap<u32, DestinyTalentExclusiveGroup>>,
    hash: Option<u32>,
    independent_node_indexes: Option<Vec<i32>>,
    index: Option<i32>,
    max_grid_level: Option<i32>,
    node_categories: Option<Vec<DestinyTalentNodeCategory>>,
    nodes: Option<Vec<DestinyTalentNodeDefinition>>,
    progression_hash: Option<u32>,
    redacted: Option<bool>,
}

impl DestinyTalentGridDefinition {
    pub fn exclusive_sets(&self) -> Option<&Vec<DestinyTalentNodeExclusiveSetDefinition>> {
        self.exclusive_sets.as_ref()
    }

    pub fn grid_level_per_column(&self) -> Option<i32> {
        self.grid_level_per_column
    }

    pub fn groups(&self) -> Option<&HashMap<u32, DestinyTalentExclusiveGroup>> {
        self.groups.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn independent_node_indexes(&self) -> Option<&Vec<i32>> {
        self.independent_node_indexes.as_ref()
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn max_grid_level(&self) -> Option<i32> {
        self.max_grid_level
    }

    pub fn node_categories(&self) -> Option<&Vec<DestinyTalentNodeCategory>> {
        self.node_categories.as_ref()
    }

    pub fn nodes(&self) -> Option<&Vec<DestinyTalentNodeDefinition>> {
        self.nodes.as_ref()
    }

    pub fn progression_hash(&self) -> Option<u32> {
        self.progression_hash
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyTalentNodeCategory
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyTalentNodeCategory {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    identifier: Option<String>,
    is_lore_driven: Option<bool>,
    node_hashes: Option<Vec<u32>>,
}

impl DestinyTalentNodeCategory {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn identifier(&self) -> Option<&String> {
        self.identifier.as_ref()
    }

    pub fn is_lore_driven(&self) -> Option<bool> {
        self.is_lore_driven
    }

    pub fn node_hashes(&self) -> Option<&Vec<u32>> {
        self.node_hashes.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyTalentNodeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyTalentNodeDefinition {
    auto_unlocks: Option<bool>,
    binary_pair_node_index: Option<i32>,
    column: Option<i32>,
    exclusive_with_node_hashes: Option<Vec<u32>>,
    group_hash: Option<u32>,
    ignore_for_completion: Option<bool>,
    is_random: Option<bool>,
    is_random_repurchasable: Option<bool>,
    last_step_repeats: Option<bool>,
    layout_identifier: Option<String>,
    lore_hash: Option<u32>,
    node_hash: Option<u32>,
    node_index: Option<i32>,
    node_style_identifier: Option<String>,
    prerequisite_node_indexes: Option<Vec<i32>>,
    random_activation_requirement: Option<DestinyNodeActivationRequirement>,
    random_start_progression_bar_at_progression: Option<i32>,
    row: Option<i32>,
    steps: Option<Vec<DestinyNodeStepDefinition>>,
}

impl DestinyTalentNodeDefinition {
    pub fn auto_unlocks(&self) -> Option<bool> {
        self.auto_unlocks
    }

    pub fn binary_pair_node_index(&self) -> Option<i32> {
        self.binary_pair_node_index
    }

    pub fn column(&self) -> Option<i32> {
        self.column
    }

    pub fn exclusive_with_node_hashes(&self) -> Option<&Vec<u32>> {
        self.exclusive_with_node_hashes.as_ref()
    }

    pub fn group_hash(&self) -> Option<u32> {
        self.group_hash
    }

    pub fn ignore_for_completion(&self) -> Option<bool> {
        self.ignore_for_completion
    }

    pub fn is_random(&self) -> Option<bool> {
        self.is_random
    }

    pub fn is_random_repurchasable(&self) -> Option<bool> {
        self.is_random_repurchasable
    }

    pub fn last_step_repeats(&self) -> Option<bool> {
        self.last_step_repeats
    }

    pub fn layout_identifier(&self) -> Option<&String> {
        self.layout_identifier.as_ref()
    }

    pub fn lore_hash(&self) -> Option<u32> {
        self.lore_hash
    }

    pub fn node_hash(&self) -> Option<u32> {
        self.node_hash
    }

    pub fn node_index(&self) -> Option<i32> {
        self.node_index
    }

    pub fn node_style_identifier(&self) -> Option<&String> {
        self.node_style_identifier.as_ref()
    }

    pub fn prerequisite_node_indexes(&self) -> Option<&Vec<i32>> {
        self.prerequisite_node_indexes.as_ref()
    }

    pub fn random_activation_requirement(&self) -> Option<&DestinyNodeActivationRequirement> {
        self.random_activation_requirement.as_ref()
    }

    pub fn random_start_progression_bar_at_progression(&self) -> Option<i32> {
        self.random_start_progression_bar_at_progression
    }

    pub fn row(&self) -> Option<i32> {
        self.row
    }

    pub fn steps(&self) -> Option<&Vec<DestinyNodeStepDefinition>> {
        self.steps.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyTalentNodeExclusiveSetDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyTalentNodeExclusiveSetDefinition {
    node_indexes: Option<Vec<i32>>,
}

impl DestinyTalentNodeExclusiveSetDefinition {
    pub fn node_indexes(&self) -> Option<&Vec<i32>> {
        self.node_indexes.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyTalentNodeStepGroups
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyTalentNodeStepGroups {
    damage_types: Option<i32>,
    guardian_attributes: Option<i32>,
    impact_effects: Option<i32>,
    light_abilities: Option<i32>,
    weapon_performance: Option<i32>,
}

impl DestinyTalentNodeStepGroups {
    pub fn damage_types(&self) -> Option<i32> {
        self.damage_types
    }

    pub fn guardian_attributes(&self) -> Option<i32> {
        self.guardian_attributes
    }

    pub fn impact_effects(&self) -> Option<i32> {
        self.impact_effects
    }

    pub fn light_abilities(&self) -> Option<i32> {
        self.light_abilities
    }

    pub fn weapon_performance(&self) -> Option<i32> {
        self.weapon_performance
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyUnlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyUnlockDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
}

impl DestinyUnlockDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyUnlockExpressionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyUnlockExpressionDefinition {
    scope: Option<i32>,
}

impl DestinyUnlockExpressionDefinition {
    pub fn scope(&self) -> Option<i32> {
        self.scope
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyUnlockValueDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyUnlockValueDefinition {
    hash: Option<u32>,
    index: Option<i32>,
    redacted: Option<bool>,
}

impl DestinyUnlockValueDefinition {
    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorAcceptedItemDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorAcceptedItemDefinition {
    accepted_inventory_bucket_hash: Option<u32>,
    destination_inventory_bucket_hash: Option<u32>,
}

impl DestinyVendorAcceptedItemDefinition {
    pub fn accepted_inventory_bucket_hash(&self) -> Option<u32> {
        self.accepted_inventory_bucket_hash
    }

    pub fn destination_inventory_bucket_hash(&self) -> Option<u32> {
        self.destination_inventory_bucket_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorActionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorActionDefinition {
    action_hash: Option<u32>,
    action_id: Option<String>,
    auto_perform_action: Option<bool>,
    description: Option<String>,
    execute_seconds: Option<i32>,
    icon: Option<String>,
    is_positive: Option<bool>,
    name: Option<String>,
    verb: Option<String>,
}

impl DestinyVendorActionDefinition {
    pub fn action_hash(&self) -> Option<u32> {
        self.action_hash
    }

    pub fn action_id(&self) -> Option<&String> {
        self.action_id.as_ref()
    }

    pub fn auto_perform_action(&self) -> Option<bool> {
        self.auto_perform_action
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn execute_seconds(&self) -> Option<i32> {
        self.execute_seconds
    }

    pub fn icon(&self) -> Option<&String> {
        self.icon.as_ref()
    }

    pub fn is_positive(&self) -> Option<bool> {
        self.is_positive
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn verb(&self) -> Option<&String> {
        self.verb.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorCategoryEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorCategoryEntryDefinition {
    buy_string_override: Option<String>,
    category_hash: Option<u32>,
    category_index: Option<i32>,
    disabled_description: Option<String>,
    display_title: Option<String>,
    hide_from_regular_purchase: Option<bool>,
    hide_if_no_currency: Option<bool>,
    is_display_only: Option<bool>,
    is_preview: Option<bool>,
    overlay: Option<DestinyVendorCategoryOverlayDefinition>,
    quantity_available: Option<i32>,
    reset_interval_minutes_override: Option<i32>,
    reset_offset_minutes_override: Option<i32>,
    show_unavailable_items: Option<bool>,
    sort_value: Option<i32>,
    vendor_item_indexes: Option<Vec<i32>>,
}

impl DestinyVendorCategoryEntryDefinition {
    pub fn buy_string_override(&self) -> Option<&String> {
        self.buy_string_override.as_ref()
    }

    pub fn category_hash(&self) -> Option<u32> {
        self.category_hash
    }

    pub fn category_index(&self) -> Option<i32> {
        self.category_index
    }

    pub fn disabled_description(&self) -> Option<&String> {
        self.disabled_description.as_ref()
    }

    pub fn display_title(&self) -> Option<&String> {
        self.display_title.as_ref()
    }

    pub fn hide_from_regular_purchase(&self) -> Option<bool> {
        self.hide_from_regular_purchase
    }

    pub fn hide_if_no_currency(&self) -> Option<bool> {
        self.hide_if_no_currency
    }

    pub fn is_display_only(&self) -> Option<bool> {
        self.is_display_only
    }

    pub fn is_preview(&self) -> Option<bool> {
        self.is_preview
    }

    pub fn overlay(&self) -> Option<&DestinyVendorCategoryOverlayDefinition> {
        self.overlay.as_ref()
    }

    pub fn quantity_available(&self) -> Option<i32> {
        self.quantity_available
    }

    pub fn reset_interval_minutes_override(&self) -> Option<i32> {
        self.reset_interval_minutes_override
    }

    pub fn reset_offset_minutes_override(&self) -> Option<i32> {
        self.reset_offset_minutes_override
    }

    pub fn show_unavailable_items(&self) -> Option<bool> {
        self.show_unavailable_items
    }

    pub fn sort_value(&self) -> Option<i32> {
        self.sort_value
    }

    pub fn vendor_item_indexes(&self) -> Option<&Vec<i32>> {
        self.vendor_item_indexes.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorCategoryOverlayDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorCategoryOverlayDefinition {
    choice_description: Option<String>,
    currency_item_hash: Option<u32>,
    description: Option<String>,
    icon: Option<String>,
    title: Option<String>,
}

impl DestinyVendorCategoryOverlayDefinition {
    pub fn choice_description(&self) -> Option<&String> {
        self.choice_description.as_ref()
    }

    pub fn currency_item_hash(&self) -> Option<u32> {
        self.currency_item_hash
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn icon(&self) -> Option<&String> {
        self.icon.as_ref()
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorDefinition {
    accepted_items: Option<Vec<DestinyVendorAcceptedItemDefinition>>,
    actions: Option<Vec<DestinyVendorActionDefinition>>,
    buy_string: Option<String>,
    categories: Option<Vec<DestinyVendorCategoryEntryDefinition>>,
    consolidate_categories: Option<bool>,
    display_categories: Option<Vec<DestinyDisplayCategoryDefinition>>,
    display_item_hash: Option<u32>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    enabled: Option<bool>,
    faction_hash: Option<u32>,
    failure_strings: Option<Vec<String>>,
    groups: Option<Vec<DestinyVendorGroupReference>>,
    hash: Option<u32>,
    ignore_sale_item_hashes: Option<Vec<u32>>,
    index: Option<i32>,
    inhibit_buying: Option<bool>,
    inhibit_selling: Option<bool>,
    interactions: Option<Vec<DestinyVendorInteractionDefinition>>,
    inventory_flyouts: Option<Vec<DestinyVendorInventoryFlyoutDefinition>>,
    item_list: Option<Vec<DestinyVendorItemDefinition>>,
    locations: Option<Vec<DestinyVendorLocationDefinition>>,
    original_categories: Option<Vec<DestinyVendorCategoryEntryDefinition>>,
    redacted: Option<bool>,
    reset_interval_minutes: Option<i32>,
    reset_offset_minutes: Option<i32>,
    return_with_vendor_request: Option<bool>,
    sell_string: Option<String>,
    services: Option<Vec<DestinyVendorServiceDefinition>>,
    unlock_ranges: Option<Vec<DateRange>>,
    vendor_banner: Option<String>,
    vendor_identifier: Option<String>,
    vendor_portrait: Option<String>,
    vendor_progression_type: Option<i32>,
    vendor_subcategory_identifier: Option<String>,
    visible: Option<bool>,
}

impl DestinyVendorDefinition {
    pub fn accepted_items(&self) -> Option<&Vec<DestinyVendorAcceptedItemDefinition>> {
        self.accepted_items.as_ref()
    }

    pub fn actions(&self) -> Option<&Vec<DestinyVendorActionDefinition>> {
        self.actions.as_ref()
    }

    pub fn buy_string(&self) -> Option<&String> {
        self.buy_string.as_ref()
    }

    pub fn categories(&self) -> Option<&Vec<DestinyVendorCategoryEntryDefinition>> {
        self.categories.as_ref()
    }

    pub fn consolidate_categories(&self) -> Option<bool> {
        self.consolidate_categories
    }

    pub fn display_categories(&self) -> Option<&Vec<DestinyDisplayCategoryDefinition>> {
        self.display_categories.as_ref()
    }

    pub fn display_item_hash(&self) -> Option<u32> {
        self.display_item_hash
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn enabled(&self) -> Option<bool> {
        self.enabled
    }

    pub fn faction_hash(&self) -> Option<u32> {
        self.faction_hash
    }

    pub fn failure_strings(&self) -> Option<&Vec<String>> {
        self.failure_strings.as_ref()
    }

    pub fn groups(&self) -> Option<&Vec<DestinyVendorGroupReference>> {
        self.groups.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn ignore_sale_item_hashes(&self) -> Option<&Vec<u32>> {
        self.ignore_sale_item_hashes.as_ref()
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn inhibit_buying(&self) -> Option<bool> {
        self.inhibit_buying
    }

    pub fn inhibit_selling(&self) -> Option<bool> {
        self.inhibit_selling
    }

    pub fn interactions(&self) -> Option<&Vec<DestinyVendorInteractionDefinition>> {
        self.interactions.as_ref()
    }

    pub fn inventory_flyouts(&self) -> Option<&Vec<DestinyVendorInventoryFlyoutDefinition>> {
        self.inventory_flyouts.as_ref()
    }

    pub fn item_list(&self) -> Option<&Vec<DestinyVendorItemDefinition>> {
        self.item_list.as_ref()
    }

    pub fn locations(&self) -> Option<&Vec<DestinyVendorLocationDefinition>> {
        self.locations.as_ref()
    }

    pub fn original_categories(&self) -> Option<&Vec<DestinyVendorCategoryEntryDefinition>> {
        self.original_categories.as_ref()
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn reset_interval_minutes(&self) -> Option<i32> {
        self.reset_interval_minutes
    }

    pub fn reset_offset_minutes(&self) -> Option<i32> {
        self.reset_offset_minutes
    }

    pub fn return_with_vendor_request(&self) -> Option<bool> {
        self.return_with_vendor_request
    }

    pub fn sell_string(&self) -> Option<&String> {
        self.sell_string.as_ref()
    }

    pub fn services(&self) -> Option<&Vec<DestinyVendorServiceDefinition>> {
        self.services.as_ref()
    }

    pub fn unlock_ranges(&self) -> Option<&Vec<DateRange>> {
        self.unlock_ranges.as_ref()
    }

    pub fn vendor_banner(&self) -> Option<&String> {
        self.vendor_banner.as_ref()
    }

    pub fn vendor_identifier(&self) -> Option<&String> {
        self.vendor_identifier.as_ref()
    }

    pub fn vendor_portrait(&self) -> Option<&String> {
        self.vendor_portrait.as_ref()
    }

    pub fn vendor_progression_type(&self) -> Option<i32> {
        self.vendor_progression_type
    }

    pub fn vendor_subcategory_identifier(&self) -> Option<&String> {
        self.vendor_subcategory_identifier.as_ref()
    }

    pub fn visible(&self) -> Option<bool> {
        self.visible
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorGroupDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorGroupDefinition {
    category_name: Option<String>,
    hash: Option<u32>,
    index: Option<i32>,
    order: Option<i32>,
    redacted: Option<bool>,
}

impl DestinyVendorGroupDefinition {
    pub fn category_name(&self) -> Option<&String> {
        self.category_name.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn order(&self) -> Option<i32> {
        self.order
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorGroupReference
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorGroupReference {
    vendor_group_hash: Option<u32>,
}

impl DestinyVendorGroupReference {
    pub fn vendor_group_hash(&self) -> Option<u32> {
        self.vendor_group_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorInteractionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorInteractionDefinition {
    flavor_line_one: Option<String>,
    flavor_line_two: Option<String>,
    header_display_properties: Option<DestinyDisplayPropertiesDefinition>,
    instructions: Option<String>,
    interaction_index: Option<i32>,
    interaction_type: Option<i32>,
    questline_item_hash: Option<u32>,
    replies: Option<Vec<DestinyVendorInteractionReplyDefinition>>,
    reward_block_label: Option<String>,
    reward_vendor_category_index: Option<i32>,
    sack_interaction_list: Option<Vec<DestinyVendorInteractionSackEntryDefinition>>,
    ui_interaction_type: Option<u32>,
    vendor_category_index: Option<i32>,
}

impl DestinyVendorInteractionDefinition {
    pub fn flavor_line_one(&self) -> Option<&String> {
        self.flavor_line_one.as_ref()
    }

    pub fn flavor_line_two(&self) -> Option<&String> {
        self.flavor_line_two.as_ref()
    }

    pub fn header_display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.header_display_properties.as_ref()
    }

    pub fn instructions(&self) -> Option<&String> {
        self.instructions.as_ref()
    }

    pub fn interaction_index(&self) -> Option<i32> {
        self.interaction_index
    }

    pub fn interaction_type(&self) -> Option<i32> {
        self.interaction_type
    }

    pub fn questline_item_hash(&self) -> Option<u32> {
        self.questline_item_hash
    }

    pub fn replies(&self) -> Option<&Vec<DestinyVendorInteractionReplyDefinition>> {
        self.replies.as_ref()
    }

    pub fn reward_block_label(&self) -> Option<&String> {
        self.reward_block_label.as_ref()
    }

    pub fn reward_vendor_category_index(&self) -> Option<i32> {
        self.reward_vendor_category_index
    }

    pub fn sack_interaction_list(
        &self,
    ) -> Option<&Vec<DestinyVendorInteractionSackEntryDefinition>> {
        self.sack_interaction_list.as_ref()
    }

    pub fn ui_interaction_type(&self) -> Option<u32> {
        self.ui_interaction_type
    }

    pub fn vendor_category_index(&self) -> Option<i32> {
        self.vendor_category_index
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorInteractionReplyDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorInteractionReplyDefinition {
    item_rewards_selection: Option<i32>,
    reply: Option<String>,
    reply_type: Option<i32>,
}

impl DestinyVendorInteractionReplyDefinition {
    pub fn item_rewards_selection(&self) -> Option<i32> {
        self.item_rewards_selection
    }

    pub fn reply(&self) -> Option<&String> {
        self.reply.as_ref()
    }

    pub fn reply_type(&self) -> Option<i32> {
        self.reply_type
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorInteractionSackEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorInteractionSackEntryDefinition {
    sack_type: Option<u32>,
}

impl DestinyVendorInteractionSackEntryDefinition {
    pub fn sack_type(&self) -> Option<u32> {
        self.sack_type
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorInventoryFlyoutBucketDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorInventoryFlyoutBucketDefinition {
    collapsible: Option<bool>,
    inventory_bucket_hash: Option<u32>,
    sort_items_by: Option<i32>,
}

impl DestinyVendorInventoryFlyoutBucketDefinition {
    pub fn collapsible(&self) -> Option<bool> {
        self.collapsible
    }

    pub fn inventory_bucket_hash(&self) -> Option<u32> {
        self.inventory_bucket_hash
    }

    pub fn sort_items_by(&self) -> Option<i32> {
        self.sort_items_by
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorInventoryFlyoutDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorInventoryFlyoutDefinition {
    buckets: Option<Vec<DestinyVendorInventoryFlyoutBucketDefinition>>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    equipment_slot_hash: Option<u32>,
    flyout_id: Option<u32>,
    locked_description: Option<String>,
    suppress_newness: Option<bool>,
}

impl DestinyVendorInventoryFlyoutDefinition {
    pub fn buckets(&self) -> Option<&Vec<DestinyVendorInventoryFlyoutBucketDefinition>> {
        self.buckets.as_ref()
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn equipment_slot_hash(&self) -> Option<u32> {
        self.equipment_slot_hash
    }

    pub fn flyout_id(&self) -> Option<u32> {
        self.flyout_id
    }

    pub fn locked_description(&self) -> Option<&String> {
        self.locked_description.as_ref()
    }

    pub fn suppress_newness(&self) -> Option<bool> {
        self.suppress_newness
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorItemDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorItemDefinition {
    action: Option<DestinyVendorSaleItemActionBlockDefinition>,
    category_index: Option<i32>,
    creation_levels: Option<Vec<DestinyItemCreationEntryLevelDefinition>>,
    currencies: Option<Vec<DestinyVendorItemQuantity>>,
    display_category: Option<String>,
    display_category_index: Option<i32>,
    exclusivity: Option<i32>,
    expiration_tooltip: Option<String>,
    failure_indexes: Option<Vec<i32>>,
    inventory_bucket_hash: Option<u32>,
    is_crm: Option<bool>,
    is_offer: Option<bool>,
    item_hash: Option<u32>,
    maximum_level: Option<i32>,
    minimum_level: Option<i32>,
    original_category_index: Option<i32>,
    purchasable_scope: Option<i32>,
    quantity: Option<i32>,
    redirect_to_sale_indexes: Option<Vec<i32>>,
    refund_policy: Option<i32>,
    refund_time_limit: Option<i32>,
    socket_overrides: Option<Vec<DestinyVendorItemSocketOverride>>,
    sort_value: Option<i32>,
    unpurchasable: Option<bool>,
    vendor_item_index: Option<i32>,
    visibility_scope: Option<i32>,
}

impl DestinyVendorItemDefinition {
    pub fn action(&self) -> Option<&DestinyVendorSaleItemActionBlockDefinition> {
        self.action.as_ref()
    }

    pub fn category_index(&self) -> Option<i32> {
        self.category_index
    }

    pub fn creation_levels(&self) -> Option<&Vec<DestinyItemCreationEntryLevelDefinition>> {
        self.creation_levels.as_ref()
    }

    pub fn currencies(&self) -> Option<&Vec<DestinyVendorItemQuantity>> {
        self.currencies.as_ref()
    }

    pub fn display_category(&self) -> Option<&String> {
        self.display_category.as_ref()
    }

    pub fn display_category_index(&self) -> Option<i32> {
        self.display_category_index
    }

    pub fn exclusivity(&self) -> Option<i32> {
        self.exclusivity
    }

    pub fn expiration_tooltip(&self) -> Option<&String> {
        self.expiration_tooltip.as_ref()
    }

    pub fn failure_indexes(&self) -> Option<&Vec<i32>> {
        self.failure_indexes.as_ref()
    }

    pub fn inventory_bucket_hash(&self) -> Option<u32> {
        self.inventory_bucket_hash
    }

    pub fn is_crm(&self) -> Option<bool> {
        self.is_crm
    }

    pub fn is_offer(&self) -> Option<bool> {
        self.is_offer
    }

    pub fn item_hash(&self) -> Option<u32> {
        self.item_hash
    }

    pub fn maximum_level(&self) -> Option<i32> {
        self.maximum_level
    }

    pub fn minimum_level(&self) -> Option<i32> {
        self.minimum_level
    }

    pub fn original_category_index(&self) -> Option<i32> {
        self.original_category_index
    }

    pub fn purchasable_scope(&self) -> Option<i32> {
        self.purchasable_scope
    }

    pub fn quantity(&self) -> Option<i32> {
        self.quantity
    }

    pub fn redirect_to_sale_indexes(&self) -> Option<&Vec<i32>> {
        self.redirect_to_sale_indexes.as_ref()
    }

    pub fn refund_policy(&self) -> Option<i32> {
        self.refund_policy
    }

    pub fn refund_time_limit(&self) -> Option<i32> {
        self.refund_time_limit
    }

    pub fn socket_overrides(&self) -> Option<&Vec<DestinyVendorItemSocketOverride>> {
        self.socket_overrides.as_ref()
    }

    pub fn sort_value(&self) -> Option<i32> {
        self.sort_value
    }

    pub fn unpurchasable(&self) -> Option<bool> {
        self.unpurchasable
    }

    pub fn vendor_item_index(&self) -> Option<i32> {
        self.vendor_item_index
    }

    pub fn visibility_scope(&self) -> Option<i32> {
        self.visibility_scope
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorItemQuantity
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorItemQuantity {
    has_conditional_visibility: Option<bool>,
    item_hash: Option<u32>,
    item_instance_id: Option<i64>,
    quantity: Option<i32>,
}

impl DestinyVendorItemQuantity {
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

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorItemSocketOverride
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorItemSocketOverride {
    randomized_options_count: Option<i32>,
    single_item_hash: Option<u32>,
    socket_type_hash: Option<u32>,
}

impl DestinyVendorItemSocketOverride {
    pub fn randomized_options_count(&self) -> Option<i32> {
        self.randomized_options_count
    }

    pub fn single_item_hash(&self) -> Option<u32> {
        self.single_item_hash
    }

    pub fn socket_type_hash(&self) -> Option<u32> {
        self.socket_type_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorSaleItemActionBlockDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorSaleItemActionBlockDefinition {
    execute_seconds: Option<f32>,
    is_positive: Option<bool>,
}

impl DestinyVendorSaleItemActionBlockDefinition {
    pub fn execute_seconds(&self) -> Option<f32> {
        self.execute_seconds
    }

    pub fn is_positive(&self) -> Option<bool> {
        self.is_positive
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyVendorServiceDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorServiceDefinition {
    name: Option<String>,
}

impl DestinyVendorServiceDefinition {
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}
