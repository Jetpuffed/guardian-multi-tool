use serde::{Deserialize, Serialize};

pub mod dates;
pub mod destiny;
pub mod interpolation;
pub mod links;
pub mod user;

/// The types of membership the Accounts system supports. This is the external
/// facing enum used in place of the internal-only Bungie.SharedDefinitions.MembershipType.
///
/// https://bungie-net.github.io/#/components/schemas/BungieMembershipType
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BungieMembershipType {
    None = 0,
    TigerXbox = 1,
    TigerPsn = 2,
    TigerSteam = 3,
    TigerBlizzard = 4,
    TigerStadia = 5,
    TigerDemon = 10,
    BungieNext = 254,

    /// "All" is only valid for searching capabilities: you need to pass the
    /// actual matching BungieMembershipType for any query where you pass a
    /// known membershipId.
    All = -1,
}
