use serde::{Deserialize, Serialize};

/// Returns data about a character's status with a given Objective. Combine
/// with DestinyObjectiveDefinition static data for display purposes.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Quests.DestinyObjectiveProgress
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyObjectiveProgress {
    activity_hash: u32,
    complete: bool,
    completion_value: i32,
    destination_hash: u32,
    objective_hash: u32,
    progress: i32,
    visible: bool,
}

impl DestinyObjectiveProgress {
    /// If the Objective has an Activity associated with it, this is the unique
    /// identifier of the Activity being referred to. Use to look up the
    /// DestinyActivityDefinition in static data. This will give localized data
    /// about *what* you should be playing for the objective to be achieved.
    pub fn activity_hash(&self) -> u32 {
        self.activity_hash
    }

    /// Whether or not the Objective is completed.
    pub fn complete(&self) -> bool {
        self.complete
    }

    /// As of Forsaken, objectives' completion value is determined dynamically
    /// at runtime.
    ///
    /// This value represents the threshold of progress you need to surpass in
    /// order for this objective to be considered "complete".
    ///
    /// If you were using objective data, switch from using the
    /// DestinyObjectiveDefinition's "completionValue" to this value.
    pub fn completion_value(&self) -> i32 {
        self.completion_value
    }

    /// If the Objective has a Destination associated with it, this is the
    /// unique identifier of the Destination being referred to. Use to look up
    /// the DestinyDestinationDefinition in static data. This will give
    /// localized data about *where* in the universe the objective should be
    /// achieved.
    pub fn destination_hash(&self) -> u32 {
        self.destination_hash
    }

    /// The unique identifier of the Objective being referred to. Use to look
    /// up the DestinyObjectiveDefinition in static data.
    pub fn objective_hash(&self) -> u32 {
        self.objective_hash
    }

    /// If progress has been made, and the progress can be measured numerically,
    /// this will be the value of that progress. You can compare it to the
    /// DestinyObjectiveDefinition.completionValue property for current vs.
    /// upper bounds, and use DestinyObjectiveDefinition.inProgressValueStyle or
    /// completedValueStyle to determine how this should be rendered. Note that
    /// progress, in Destiny 2, need not be a literal numeric progression. It
    /// could be one of a number of possible values, even a Timestamp. Always
    /// examine DestinyObjectiveDefinition.inProgressValueStyle or
    /// completedValueStyle before rendering progress.
    pub fn progress(&self) -> i32 {
        self.progress
    }

    /// If this is true, the objective is visible in-game. Otherwise, it's not
    /// yet visible to the player. Up to you if you want to honor this property.
    pub fn visible(&self) -> bool {
        self.visible
    }
}
