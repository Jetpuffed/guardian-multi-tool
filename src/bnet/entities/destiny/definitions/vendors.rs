use serde::{Deserialize, Serialize};

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Vendors.DestinyVendorLocationDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyVendorLocationDefinition {
    background_image_path: Option<String>,
    destination_hash: Option<u32>,
}

impl DestinyVendorLocationDefinition {
    pub fn background_image_path(&self) -> Option<&String> {
        self.background_image_path.as_ref()
    }

    pub fn destination_hash(&self) -> Option<u32> {
        self.destination_hash
    }
}
