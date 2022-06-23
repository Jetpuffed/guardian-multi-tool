use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::bnet::entities::user::UserInfoCard;

use super::components::inventory::DestinyPlatformSilverComponent;

/// If a Destiny Profile can't be returned, but we're pretty certain it's a
/// valid Destiny account, this will contain as much info as we can get about
/// the profile for your use.
///
/// Assume that the most you'll get is the Error Code, the Membership Type and
/// the Membership ID.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Responses.DestinyErrorProfile
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyErrorProfile {
    error_code: i32,
    info_card: UserInfoCard,
}

impl DestinyErrorProfile {
    /// The error that we encountered. You should be able to look up localized
    /// text to show to the user for these failures.
    pub fn error_code(&self) -> i32 {
        self.error_code
    }

    /// Basic info about the account that failed. Don't expect anything other
    /// than membership ID, Membership Type, and displayName to be populated.
    pub fn info_card(&self) -> &UserInfoCard {
        &self.info_card
    }
}

/// I know what you seek. You seek linked accounts. Found them, you have.
///
/// This contract returns a minimal amount of data about Destiny Accounts that
/// are linked through your Bungie.Net account.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Responses.DestinyLinkedProfilesResponse
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyLinkedProfilesResponse {
    bnet_membership: UserInfoCard,
    profiles: Vec<DestinyProfileUserInfoCard>,
    profiles_with_errors: DestinyErrorProfile,
}

impl DestinyLinkedProfilesResponse {
    /// If the requested membership had a linked Bungie.Net membership ID, this
    /// is the basic information about that BNet account.
    ///
    /// I know, Tetron; I know this is mixing UserServices concerns with
    /// DestinyServices concerns. But it's so damn convenient!
    pub fn bnet_membership(&self) -> &UserInfoCard {
        &self.bnet_membership
    }

    /// Any Destiny account for whom we could successfully pull characters will
    /// be returned here, as the Platform-level summary of user data. (no
    /// character data, no Destiny account data other than the Membership ID
    /// and Type so you can make further queries)
    pub fn profiles(&self) -> &Vec<DestinyProfileUserInfoCard> {
        &self.profiles
    }

    /// This is brief summary info for profiles that we believe have valid
    /// Destiny info, but who failed to return data for some other reason and
    /// thus we know that subsequent calls for their info will also fail.
    pub fn profiles_with_errors(&self) -> &DestinyErrorProfile {
        &self.profiles_with_errors
    }
}

// /// The response for GetDestinyProfile, with components for character and item-level data.
// ///
// /// https://bungie-net.github.io/#/components/schemas/Destiny.Responses.DestinyProfileResponse
// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct DestinyProfileResponse {
//     vendor_receipts: ComponentResponse<DestinyVendorReceiptsComponent>,
//     profile_inventory: ComponentResponse<DestinyInventoryComponent>,
//     profile_currencies: ComponentResponse<DestinyInventoryComponent>,
//     profile: ComponentResponse<DestinyProfileComponent>,
//     platform_silver: ComponentResponse<DestinyPlatformSilverComponent>,
//     profile_kiosks: ComponentResponse<DestinyKiosksComponent>,
//     profile_plug_sets: ComponentResponse<DestinyPlugSetsComponent>,
//     profile_progression: ComponentResponse<DestinyProfileProgressionComponent>,
//     profile_presentation_nodes: ComponentResponse<DestinyPresentationNodesComponent>,
//     profile_records: ComponentResponse<DestinyProfileRecordsComponent>,
//     profile_collectibles: ComponentResponse<DestinyProfileCollectiblesComponent>,
//     profile_transitory_data: ComponentResponse<DestinyProfileTransitoryComponent>,
//     metrics: ComponentResponse<DestinyMetricsComponent>,
//     profile_string_variables: ComponentResponse<DestinyStringVariablesComponent>,
//     characters: ComponentResponse<HashMap<i64, DestinyCharacterComponent>>,
//     character_inventories: ComponentResponse<HashMap<i64, DestinyInventoryComponent>>,
//     character_progressions: ComponentResponse<HashMap<i64, DestinyCharacterProgressionComponent>>,
//     character_render_data: ComponentResponse<HashMap<i64, DestinyCharacterRenderComponent>>,
//     character_activities: ComponentResponse<HashMap<i64, DestinyCharacterActivitiesComponent>>,
//     character_equipment: ComponentResponse<HashMap<i64, DestinyInventoryComponent>>,
//     character_kiosks: ComponentResponse<HashMap<i64, DestinyKiosksComponent>>,
//     character_plug_sets: ComponentResponse<HashMap<i64, DestinyPlugSetsComponent>>,
//     character_uninstanced_item_components:
// }

/// https://bungie-net.github.io/#/components/schemas/Destiny.Responses.DestinyProfileUserInfoCard
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProfileUserInfoCard {
    applicable_membership_types: Vec<i32>,
    bungie_global_display_name: String,
    bungie_global_display_name_code: i16,
    cross_save_override: i32,
    date_last_played: DateTime<Utc>,
    display_name: String,
    icon_path: String,
    is_cross_save_primary: bool,
    is_overridden: bool,
    is_public: bool,
    membership_type: i64,
    platform_silver: DestinyPlatformSilverComponent,
    supplemental_display_name: String,
    unpaired_game_versions: i32,
}

impl DestinyProfileUserInfoCard {
    pub fn applicable_membership_types(&self) -> &[i32] {
        self.applicable_membership_types.as_ref()
    }

    pub fn bungie_global_display_name(&self) -> &str {
        self.bungie_global_display_name.as_ref()
    }

    pub fn bungie_global_display_name_code(&self) -> i16 {
        self.bungie_global_display_name_code
    }

    pub fn cross_save_override(&self) -> i32 {
        self.cross_save_override
    }

    pub fn date_last_played(&self) -> DateTime<Utc> {
        self.date_last_played
    }

    pub fn display_name(&self) -> &str {
        self.display_name.as_ref()
    }

    pub fn icon_path(&self) -> &str {
        self.icon_path.as_ref()
    }

    pub fn is_cross_save_primary(&self) -> bool {
        self.is_cross_save_primary
    }

    pub fn is_overridden(&self) -> bool {
        self.is_overridden
    }

    pub fn is_public(&self) -> bool {
        self.is_public
    }

    pub fn membership_type(&self) -> i64 {
        self.membership_type
    }

    pub fn platform_silver(&self) -> &DestinyPlatformSilverComponent {
        &self.platform_silver
    }

    pub fn supplemental_display_name(&self) -> &str {
        self.supplemental_display_name.as_ref()
    }

    pub fn unpaired_game_versions(&self) -> i32 {
        self.unpaired_game_versions
    }
}
