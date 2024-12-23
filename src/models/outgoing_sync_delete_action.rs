/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OutgoingSyncDeleteAction {
    #[serde(rename = "do_nothing")]
    DoNothing,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "suspend")]
    Suspend,
}

impl std::fmt::Display for OutgoingSyncDeleteAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DoNothing => write!(f, "do_nothing"),
            Self::Delete => write!(f, "delete"),
            Self::Suspend => write!(f, "suspend"),
        }
    }
}

impl Default for OutgoingSyncDeleteAction {
    fn default() -> OutgoingSyncDeleteAction {
        Self::DoNothing
    }
}
