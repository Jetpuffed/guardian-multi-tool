use reqwest::Client;

use crate::{Result, bnet::entities::destiny::config::DestinyManifest};

use super::Response;

pub struct Destiny<'a>(pub &'a Client);

impl Destiny<'_> {
    /// Returns the current version of the manifest as a json object.
    pub async fn get_destiny_manifest(&self) -> Result<Response<DestinyManifest>> {
        match self.0.get("https://www.bungie.net/platform/destiny2/manifest/").send().await {
            Ok(resp) => Ok(resp.json::<Response<DestinyManifest>>().await?),
            Err(e) => Err(Box::new(e)),
        }
    }
}
