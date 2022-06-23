use serde::{Deserialize, Serialize};

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Animations.DestinyAnimationReference
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyAnimationReference {
    anim_identifier: Option<String>,
    anim_name: Option<String>,
    path: Option<String>,
}

impl DestinyAnimationReference {
    pub fn anim_identifier(&self) -> Option<&String> {
        self.anim_identifier.as_ref()
    }

    pub fn anim_name(&self) -> Option<&String> {
        self.anim_name.as_ref()
    }

    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }
}
