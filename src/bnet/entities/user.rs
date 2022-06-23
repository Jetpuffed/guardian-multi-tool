use serde::{Deserialize, Serialize};

/// This contract supplies basic information commonly used to display a minimal
/// amount of information about a user. Take care to not add more properties
/// here unless the property applies in all (or at least the majority) of the
/// situations where UserInfoCard is used. Avoid adding game specific or platform
/// specific details here. In cases where UserInfoCard is a subset of the data
/// needed in a contract, use UserInfoCard as a property of other contracts.
///
/// https://bungie-net.github.io/#/components/schemas/User.UserInfoCard
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoCard {
    applicable_membership_types: Vec<i32>,
    bungie_global_display_name: String,
    bungie_global_display_name_code: i16,
    cross_save_override: i32,
    display_name: String,
    icon_path: String,
    is_public: bool,
    membership_id: i64,
    membership_type: i32,
    supplemental_display_name: String,
}

impl UserInfoCard {
    /// The list of Membership Types indicating the platforms on which this
    /// Membership can be used.
    pub fn applicable_membership_types(&self) -> &[i32] {
        self.applicable_membership_types.as_ref()
    }

    /// The bungie global display name, if set.
    pub fn bungie_global_display_name(&self) -> &str {
        self.bungie_global_display_name.as_ref()
    }

    /// The bungie global display name code, if set.
    pub fn bungie_global_display_name_code(&self) -> i16 {
        self.bungie_global_display_name_code
    }

    /// If there is a cross save override in effect, this value will tell you
    /// the type that is overridding this one.
    pub fn cross_save_override(&self) -> i32 {
        self.cross_save_override
    }

    /// Display Name the player has chosen for themselves. The display name is
    /// optional when the data type is used as input to a platform API.
    pub fn display_name(&self) -> &str {
        self.display_name.as_ref()
    }

    /// URL the Icon if available.
    pub fn icon_path(&self) -> &str {
        self.icon_path.as_ref()
    }

    /// If True, this is a public user membership.
    pub fn is_public(&self) -> bool {
        self.is_public
    }

    /// Membership ID as they user is known in the Accounts service
    pub fn membership_id(&self) -> i64 {
        self.membership_id
    }

    /// Type of the membership. Not necessarily the native type.
    pub fn membership_type(&self) -> i32 {
        self.membership_type
    }

    /// A platform specific additional display name - ex: psn Real Name, bnet
    /// Unique Name, etc.
    pub fn supplemental_display_name(&self) -> &str {
        self.supplemental_display_name.as_ref()
    }
}
