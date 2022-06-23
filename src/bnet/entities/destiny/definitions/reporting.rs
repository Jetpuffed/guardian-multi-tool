use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::DestinyDisplayPropertiesDefinition;

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Reporting.DestinyReportReasonCategoryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyReportReasonCategoryDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    hash: Option<u32>,
    index: Option<i32>,
    reasons: Option<HashMap<u32, DestinyReportReasonDefinition>>,
    redacted: Option<bool>,
}

impl DestinyReportReasonCategoryDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    pub fn index(&self) -> Option<i32> {
        self.index
    }

    pub fn reasons(&self) -> Option<&HashMap<u32, DestinyReportReasonDefinition>> {
        self.reasons.as_ref()
    }

    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Reporting.DestinyReportReasonDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyReportReasonDefinition {
    display_properties: Option<DestinyDisplayPropertiesDefinition>,
    reason_hash: Option<u32>,
}

impl DestinyReportReasonDefinition {
    pub fn display_properties(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.display_properties.as_ref()
    }

    pub fn reason_hash(&self) -> Option<u32> {
        self.reason_hash
    }
}
