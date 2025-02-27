/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PolicyEngineMode {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "any")]
    Any,
}

impl std::fmt::Display for PolicyEngineMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Any => write!(f, "any"),
        }
    }
}

impl Default for PolicyEngineMode {
    fn default() -> PolicyEngineMode {
        Self::All
    }
}
