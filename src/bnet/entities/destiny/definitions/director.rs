use serde::{Deserialize, Serialize};

use super::{
    common::{DestinyDisplayPropertiesDefinition, DestinyPositionDefinition},
    DestinyUnlockExpressionDefinition,
};

/// These Art Elements are meant to represent one-off visual effects overlaid
/// on the map. Currently, we do not have a pipeline to import the assets for
/// these overlays, so this info exists as a placeholder for when such a
/// pipeline exists (if it ever will)
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphArtElementDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphArtElementDefinition {
    position: Option<DestinyPositionDefinition>,
}

impl DestinyActivityGraphArtElementDefinition {
    /// The position on the map of the art element.
    pub fn position(&self) -> Option<&DestinyPositionDefinition> {
        self.position.as_ref()
    }
}

/// Nodes on a graph can be visually connected: this appears to be the
/// information about which nodes to link. It appears to lack more detailed
/// information, such as the path for that linking.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphConnectionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphConnectionDefinition {
    dest_node_hash: Option<u32>,
    source_node_hash: Option<u32>,
}

impl DestinyActivityGraphConnectionDefinition {
    pub fn dest_node_hash(&self) -> Option<u32> {
        self.dest_node_hash
    }

    pub fn source_node_hash(&self) -> Option<u32> {
        self.source_node_hash
    }
}

/// Represents a Map View in the director: be them overview views, destination
/// views, or other.
///
/// They have nodes which map to activities, and other various visual elements
/// that we (or others) may or may not be able to use.
///
/// Activity graphs, most importantly, have nodes which can have activities in
/// various states of playability.
///
/// Unfortunately, activity graphs are combined at runtime with Game UI-only
/// assets such as fragments of map images, various in-game special effects,
/// decals etc... that we don't get in these definitions.
///
/// If we end up having time, we may end up trying to manually populate those
/// here: but the last time we tried that, before the lead-up to D1, it proved
/// to be unmaintainable as the game's content changed. So don't bet the farm
/// on us providing that content in this definition.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphDefinition {
    art_elements: Option<Vec<DestinyActivityGraphArtElementDefinition>>,
    connections: Option<Vec<DestinyActivityGraphConnectionDefinition>>,
    display_objectives: Option<Vec<DestinyActivityGraphDisplayObjectiveDefinition>>,
    display_progressions: Option<Vec<DestinyActivityGraphDisplayProgressionDefinition>>,
    hash: Option<u32>,
    index: Option<i32>,
    linked_graphs: Option<Vec<DestinyLinkedGraphDefinition>>,
    nodes: Option<Vec<DestinyActivityGraphNodeDefinition>>,
    redacted: Option<bool>,
}

impl DestinyActivityGraphDefinition {
    /// Represents one-off/special UI elements that appear on the map.
    pub fn art_elements(&self) -> Option<&Vec<DestinyActivityGraphArtElementDefinition>> {
        self.art_elements.as_ref()
    }

    /// Represents connections between graph nodes. However, it lacks context
    /// that we'd need to make good use of it.
    pub fn connections(&self) -> Option<&Vec<DestinyActivityGraphConnectionDefinition>> {
        self.connections.as_ref()
    }

    /// Objectives can display on maps, and this is supposedly metadata for that.
    /// I have not had the time to analyze the details of what is useful within
    /// however: we could be missing important data to make this work. Expect
    /// this property to be expanded on later if possible.
    pub fn display_objectives(
        &self,
    ) -> Option<&Vec<DestinyActivityGraphDisplayObjectiveDefinition>> {
        self.display_objectives.as_ref()
    }

    /// Progressions can also display on maps, but similarly to displayObjectives
    /// we appear to lack some required information and context right now.
    /// We will have to look into it later and add more data if possible.
    pub fn display_progressions(
        &self,
    ) -> Option<&Vec<DestinyActivityGraphDisplayProgressionDefinition>> {
        self.display_progressions.as_ref()
    }

    /// The unique identifier for this entity. Guaranteed to be unique for the
    /// type of entity, but not globally. When entities refer to each other in
    /// Destiny content, it is this hash that they are referring to.
    pub fn hash(&self) -> Option<u32> {
        self.hash
    }

    /// The index of the entity as it was found in the investment tables.
    pub fn index(&self) -> Option<i32> {
        self.index
    }

    /// Represents links between this Activity Graph and other ones.
    pub fn linked_graphs(&self) -> Option<&Vec<DestinyLinkedGraphDefinition>> {
        self.linked_graphs.as_ref()
    }

    /// These represent the visual "nodes" on the map's view. These are the
    /// activities you can click on in the map.
    pub fn nodes(&self) -> Option<&Vec<DestinyActivityGraphNodeDefinition>> {
        self.nodes.as_ref()
    }

    /// If this is true, then there is an entity with this identifier/type
    /// combination, but BNet is not yet allowed to show it. Sorry!
    pub fn redacted(&self) -> Option<bool> {
        self.redacted
    }
}

/// When a Graph needs to show active Objectives, this defines those objectives
/// as well as an identifier.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphDisplayObjectiveDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphDisplayObjectiveDefinition {
    id: Option<u32>,
    objective_hash: Option<u32>,
}

impl DestinyActivityGraphDisplayObjectiveDefinition {
    /// This field is apparently something that CUI uses to manually wire up
    /// objectives to display info. I am unsure how it works
    pub fn id(&self) -> Option<u32> {
        self.id
    }

    /// The objective being shown on the map.
    pub fn objective_hash(&self) -> Option<u32> {
        self.objective_hash
    }
}

/// When a Graph needs to show active Progressions, this defines those
/// objectives as well as an identifier.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphDisplayProgressionDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphDisplayProgressionDefinition {
    id: Option<u32>,
    progression_hash: Option<u32>,
}

impl DestinyActivityGraphDisplayProgressionDefinition {
    pub fn id(&self) -> Option<u32> {
        self.id
    }

    pub fn progression_hash(&self) -> Option<u32> {
        self.progression_hash
    }
}

/// The actual activity to be redirected to when you click on the node.
///
/// Note that a node can have many Activities attached to it: but only one will
/// be active at any given time. The list of Node Activities will be traversed,
/// and the first one found to be active will be displayed. This way, a node
/// can layer multiple variants of an activity on top of each other. For
/// instance, one node can control the weekly Crucible Playlist. There are
/// multiple possible playlists, but only one is active for the week.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphNodeActivityDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphNodeActivityDefinition {
    activity_hash: Option<u32>,
    node_activity_id: Option<u32>,
}

impl DestinyActivityGraphNodeActivityDefinition {
    /// The activity that will be activated if the user clicks on this node.
    /// Controls all activity-related information displayed on the node if it
    /// is active (the text shown in the tooltip etc)
    pub fn activity_hash(&self) -> Option<u32> {
        self.activity_hash
    }

    /// An identifier for this node activity. It is only guaranteed to be
    /// unique within the Activity Graph.
    pub fn node_activity_id(&self) -> Option<u32> {
        self.node_activity_id
    }
}

/// This is the position and other data related to nodes in the activity graph
/// that you can click to launch activities. An Activity Graph node will only
/// have one active Activity at a time, which will determine the activity to be
/// launched (and, unless overrideDisplay information is provided, will also
/// determine the tooltip and other UI related to the node)
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphNodeDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphNodeDefinition {
    activities: Option<Vec<DestinyActivityGraphNodeActivityDefinition>>,
    featuring_states: Option<Vec<DestinyActivityGraphNodeFeaturingStateDefinition>>,
    node_id: Option<u32>,
    override_display: Option<DestinyDisplayPropertiesDefinition>,
    position: Option<DestinyPositionDefinition>,
    states: Option<Vec<DestinyActivityGraphNodeStateEntry>>,
}

impl DestinyActivityGraphNodeDefinition {
    /// The node may have various possible activities that could be active for
    /// it, however only one may be active at a time. See the
    /// DestinyActivityGraphNodeActivityDefinition for details.
    pub fn activities(&self) -> Option<&Vec<DestinyActivityGraphNodeActivityDefinition>> {
        self.activities.as_ref()
    }

    /// The node may have various visual accents placed on it, or styles
    /// applied. These are the list of possible styles that the Node can have.
    /// The game iterates through each, looking for the first one that passes
    /// a check of the required game/character/account state in order to show
    /// that style, and then renders the node in that style.
    pub fn featuring_states(
        &self,
    ) -> Option<&Vec<DestinyActivityGraphNodeFeaturingStateDefinition>> {
        self.featuring_states.as_ref()
    }

    /// An identifier for the Activity Graph Node, only guaranteed to be unique
    /// within its parent Activity Graph.
    pub fn node_id(&self) -> Option<u32> {
        self.node_id
    }

    /// The node *may* have display properties that override the active
    /// Activity's display properties.
    pub fn override_display(&self) -> Option<&DestinyDisplayPropertiesDefinition> {
        self.override_display.as_ref()
    }

    /// The position on the map for this node.
    pub fn position(&self) -> Option<&DestinyPositionDefinition> {
        self.position.as_ref()
    }

    /// Represents possible states that the graph node can be in. These are
    /// combined with some checking that happens in the game client and server
    /// to determine which state is actually active at any given time.
    pub fn states(&self) -> Option<&Vec<DestinyActivityGraphNodeStateEntry>> {
        self.states.as_ref()
    }
}

/// Nodes can have different visual states. This object represents a single
/// visual state ("highlight type") that a node can be in, and the unlock
/// expression condition to determine whether it should be set.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphNodeFeaturingStateDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphNodeFeaturingStateDefinition {
    highlight_type: Option<i32>,
}

impl DestinyActivityGraphNodeFeaturingStateDefinition {
    /// The node can be highlighted in a variety of ways - the game iterates
    /// through these and finds the first FeaturingState that is valid at the
    /// present moment given the Game, Account, and Character state, and
    /// renders the node in that state. See the ActivityGraphNodeHighlightType
    /// enum for possible values.
    pub fn highlight_type(&self) -> Option<i32> {
        self.highlight_type
    }
}

/// Represents a single state that a graph node might end up in. Depending on
/// what's going on in the game, graph nodes could be shown in different ways
/// or even excluded from view entirely.
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyActivityGraphNodeStateEntry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityGraphNodeStateEntry {
    state: Option<i32>,
}

impl DestinyActivityGraphNodeStateEntry {
    pub fn state(&self) -> Option<i32> {
        self.state
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyLinkedGraphDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyLinkedGraphDefinition {
    description: Option<String>,
    linked_graph_id: Option<u32>,
    linked_graphs: Option<Vec<DestinyLinkedGraphEntryDefinition>>,
    name: Option<String>,
    overview: Option<String>,
    unlock_expression: Option<DestinyUnlockExpressionDefinition>,
}

impl DestinyLinkedGraphDefinition {
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn linked_graph_id(&self) -> Option<u32> {
        self.linked_graph_id
    }

    pub fn linked_graphs(&self) -> Option<&Vec<DestinyLinkedGraphEntryDefinition>> {
        self.linked_graphs.as_ref()
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn overview(&self) -> Option<&String> {
        self.overview.as_ref()
    }

    pub fn unlock_expression(&self) -> Option<&DestinyUnlockExpressionDefinition> {
        self.unlock_expression.as_ref()
    }
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Director.DestinyLinkedGraphEntryDefinition
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyLinkedGraphEntryDefinition {
    activity_graph_hash: Option<u32>,
}

impl DestinyLinkedGraphEntryDefinition {
    pub fn activity_graph_hash(&self) -> Option<u32> {
        self.activity_graph_hash
    }
}
