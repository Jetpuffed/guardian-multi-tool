use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// DestinyManifest is the external-facing contract for just the properties
/// needed by those calling the Destiny Platform.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Config.DestinyManifest
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyManifest {
    icon_image_pyramid_info: Vec<ImagePyramidEntry>,
    json_world_component_content_paths: HashMap<String, HashMap<String, String>>,
    json_world_content_paths: HashMap<String, String>,
    mobile_asset_content_path: String,
    mobile_clan_banner_database_path: String,
    mobile_gear_asset_data_bases: Vec<GearAssetDataBaseDefinition>,
    mobile_gear_c_d_n: HashMap<String, String>,
    mobile_world_content_paths: HashMap<String, String>,
    version: String,
}

impl DestinyManifest {
    /// Information about the "Image Pyramid" for Destiny icons.
    ///
    /// Where possible, we create smaller versions of Destiny icons. These are
    /// found as subfolders under the location of the "original/full size"
    /// Destiny images, with the same file name and extension as the original
    /// image itself. (this lets us avoid sending largely redundant path info
    /// with every entity, at the expense of the smaller versions of the image
    /// being less discoverable)
    pub fn icon_image_pyramid_info(&self) -> &Vec<ImagePyramidEntry> {
        &self.icon_image_pyramid_info
    }

    /// This points to the generated JSON that contains all the Definitions.
    ///
    /// Each key is a locale. The value is a dictionary, where the key is a
    /// definition type by name, and the value is the path to the file for that
    /// definition.
    ///
    /// WARNING: This is unsafe and subject to change - do not depend on data in these files staying around long-term
    pub fn json_world_component_content_paths(&self) -> &HashMap<String, HashMap<String, String>> {
        &self.json_world_component_content_paths
    }

    /// This points to the generated JSON that contains all the Definitions.
    ///
    /// Each key is a locale. The value is a path to the aggregated world
    /// definitions (warning: large file!)
    pub fn json_world_content_paths(&self) -> &HashMap<String, String> {
        &self.json_world_content_paths
    }

    pub fn mobile_asset_content_path(&self) -> &str {
        self.mobile_asset_content_path.as_ref()
    }

    pub fn mobile_clan_banner_database_path(&self) -> &str {
        self.mobile_clan_banner_database_path.as_ref()
    }

    pub fn mobile_gear_asset_data_bases(&self) -> &Vec<GearAssetDataBaseDefinition> {
        &self.mobile_gear_asset_data_bases
    }

    pub fn mobile_gear_cdn(&self) -> &HashMap<String, String> {
        &self.mobile_gear_c_d_n
    }

    pub fn mobile_world_content_paths(&self) -> &HashMap<String, String> {
        &self.mobile_world_content_paths
    }

    pub fn version(&self) -> &str {
        self.version.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Config.GearAssetDataBaseDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GearAssetDataBaseDefinition {
    path: Option<String>,
    version: Option<i32>,
}

impl GearAssetDataBaseDefinition {
    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    pub fn version(&self) -> Option<i32> {
        self.version
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Config.ImagePyramidEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagePyramidEntry {
    name: Option<String>,
    factor: Option<f32>,
}

impl ImagePyramidEntry {
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn factor(&self) -> Option<f32> {
        self.factor
    }
}
