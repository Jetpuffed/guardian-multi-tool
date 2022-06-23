use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::bnet::entities::destiny::entities::items::DestinyItemComponent;

/// https://bungie-net.github.io/#/components/schemas/Destiny.Components.Inventory.DestinyPlatformSilverComponent
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyPlatformSilverComponent {
    platform_silver: HashMap<i32, DestinyItemComponent>,
}

impl DestinyPlatformSilverComponent {
    /// If a Profile is played on multiple platforms, this is the silver they
    /// have for each platform, keyed by Membership Type.
    pub fn platform_silver(&self) -> &HashMap<i32, DestinyItemComponent> {
        &self.platform_silver
    }
}
