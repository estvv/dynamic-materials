//! Material behavior definitions.
//!
//! This module defines the different behavioral types that materials can exhibit
//! in the simulation, such as static, granular, or liquid.

use serde::Deserialize;

/// Defines how a material behaves in the simulation.
///
/// Each behavior type determines how particles of that material move and interact
/// with their surroundings during each update cycle.
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Behavior {
    Static,
    Granular,
    Liquid,
    Burning,
    Solid,
    Heavy,
    Acid,
}

impl std::fmt::Display for Behavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
