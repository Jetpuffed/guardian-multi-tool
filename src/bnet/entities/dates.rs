use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// https://bungie-net.github.io/#/components/schemas/Dates.DateRange
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateRange {
    end: Option<DateTime<Utc>>,
    start: Option<DateTime<Utc>>,
}

impl DateRange {
    pub fn end(&self) -> Option<DateTime<Utc>> {
        self.end
    }

    pub fn start(&self) -> Option<DateTime<Utc>> {
        self.start
    }
}
