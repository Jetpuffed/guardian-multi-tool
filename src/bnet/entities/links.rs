use serde::{Deserialize, Serialize};

/// https://bungie-net.github.io/#/components/schemas/Links.HyperlinkReference
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperlinkReference {
    title: Option<String>,
    url: Option<String>,
}

impl HyperlinkReference {
    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn url(&self) -> Option<&String> {
        self.url.as_ref()
    }
}
