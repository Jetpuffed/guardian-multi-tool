use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::bnet::entities::destiny::quests::DestinyObjectiveProgress;

/// The base item component, filled with properties that are generally useful
/// to know in any item request or that don't feel worthwhile to put in their
/// own component.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Entities.Items.DestinyItemComponent
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyItemComponent {
    bind_status: i32,
    bucket_hash: u32,
    expiration_date: DateTime<Utc>,
    is_wrapper: bool,
    item_hash: u32,
    item_instance_id: i64,
    item_value_visibility: Vec<bool>,
    location: i32,
    lockable: bool,
    metric_hash: u32,
    metric_objective: DestinyObjectiveProgress,
    override_style_item_hash: u32,
    quantity: i32,
    state: i32,
    tooltip_notification_indexes: Vec<i32>,
    transfer_status: i32,
    version_number: i32,
}

impl DestinyItemComponent {
    pub fn bind_status(&self) -> i32 {
        self.bind_status
    }

    pub fn bucket_hash(&self) -> u32 {
        self.bucket_hash
    }

    pub fn expiration_date(&self) -> DateTime<Utc> {
        self.expiration_date
    }

    pub fn is_wrapper(&self) -> bool {
        self.is_wrapper
    }

    pub fn item_hash(&self) -> u32 {
        self.item_hash
    }

    pub fn item_instance_id(&self) -> i64 {
        self.item_instance_id
    }

    pub fn item_value_visibility(&self) -> &[bool] {
        self.item_value_visibility.as_ref()
    }

    pub fn location(&self) -> i32 {
        self.location
    }

    pub fn lockable(&self) -> bool {
        self.lockable
    }

    pub fn metric_hash(&self) -> u32 {
        self.metric_hash
    }

    pub fn metric_objective(&self) -> &DestinyObjectiveProgress {
        &self.metric_objective
    }

    pub fn override_style_item_hash(&self) -> u32 {
        self.override_style_item_hash
    }

    pub fn quantity(&self) -> i32 {
        self.quantity
    }

    pub fn state(&self) -> i32 {
        self.state
    }

    pub fn tooltip_notification_indexes(&self) -> &[i32] {
        self.tooltip_notification_indexes.as_ref()
    }

    pub fn transfer_status(&self) -> i32 {
        self.transfer_status
    }

    pub fn version_number(&self) -> i32 {
        self.version_number
    }
}
