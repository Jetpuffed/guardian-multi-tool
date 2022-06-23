use serde::{Deserialize, Serialize};

/// https://bungie-net.github.io/#/components/schemas/Destiny.Misc.DestinyColor
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyColor {
    alpha: Option<u8>,
    blue: Option<u8>,
    green: Option<u8>,
    red: Option<u8>,
}

impl DestinyColor {
    pub fn alpha(&self) -> Option<u8> {
        self.alpha
    }

    pub fn blue(&self) -> Option<u8> {
        self.blue
    }

    pub fn green(&self) -> Option<u8> {
        self.green
    }

    pub fn red(&self) -> Option<u8> {
        self.red
    }
}
