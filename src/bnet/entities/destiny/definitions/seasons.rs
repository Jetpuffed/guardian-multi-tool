use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::common::DestinyDisplayPropertiesDefinition;

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Seasons.DestinySeasonDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySeasonDefinition {
    artifact_item_hash: Option<u32>,
    background_image_path: Option<String>,
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    end_date: Option<DateTime<Utc>>,
    hash: Option<u32>,
    index: Option<i32>,
    preview: Option<DestinySeasonPreviewDefinition>,
    redacted: Option<bool>,
    seal_presentation_node_hash: Option<u32>,
    season_number: Option<i32>,
    season_pass_hash: Option<u32>,
    season_pass_progression_hash: Option<u32>,
    seasonal_challenges_presentation_node_hash: Option<u32>,
    start_date: Option<DateTime<Utc>>,
}

impl DestinySeasonDefinition {
    pub fn artifact_item_hash(&self) -> Option<u32> {
        self.artifact_item_hash
    }

    pub fn background_image_path(&self) -> Option<&String> {
        self.background_image_path.as_ref()
    }

    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn end_date(&self) -> Option<DateTime<Utc>> {
        self.end_date
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn preview(&self) -> Option<&DestinySeasonPreviewDefinition> {
        self.preview.as_ref()
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn seal_presentation_node_hash(&self) -> Option<u32> {
        self.seal_presentation_node_hash
    }

    pub fn season_number(&self) -> Option<i32> {
        self.season_number
    }

    pub fn season_pass_hash(&self) -> Option<u32> {
        self.season_pass_hash
    }

    pub fn season_pass_progression_hash(&self) -> Option<u32> {
        self.season_pass_progression_hash
    }

    pub fn seasonal_challenges_presentation_node_hash(&self) -> Option<u32> {
        self.seasonal_challenges_presentation_node_hash
    }

    pub fn start_date(&self) -> Option<DateTime<Utc>> {
        self.start_date
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Seasons.DestinySeasonPassDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySeasonPassDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    prestige_progression_hash: Option<u32>,
    redacted: Option<bool>,
    reward_progression_hash: Option<u32>,
}

impl DestinySeasonPassDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn prestige_progression_hash(&self) -> Option<u32> {
        self.prestige_progression_hash
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }

    pub fn reward_progression_hash(&self) -> Option<u32> {
        self.reward_progression_hash
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Seasons.DestinySeasonPreviewDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySeasonPreviewDefinition {
    description: Option<String>,
    images: Option<Vec<DestinySeasonPreviewImageDefinition>>,
    link_path: Option<String>,
    video_link: Option<String>,
}

impl DestinySeasonPreviewDefinition {
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn images(&self) -> Option<&Vec<DestinySeasonPreviewImageDefinition>> {
        self.images.as_ref()
    }

    pub fn link_path(&self) -> Option<&String> {
        self.link_path.as_ref()
    }

    pub fn video_link(&self) -> Option<&String> {
        self.video_link.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Seasons.DestinySeasonPreviewImageDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinySeasonPreviewImageDefinition {
    high_res_image: Option<String>,
    thumbnail_image: Option<String>,
}

impl DestinySeasonPreviewImageDefinition {
    pub fn high_res_image(&self) -> Option<&String> {
        self.high_res_image.as_ref()
    }

    pub fn thumbnail_image(&self) -> Option<&String> {
        self.thumbnail_image.as_ref()
    }
}
