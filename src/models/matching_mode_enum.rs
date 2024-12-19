/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MatchingModeEnum {
    #[serde(rename = "strict")]
    Strict,
    #[serde(rename = "regex")]
    Regex,
}

impl std::fmt::Display for MatchingModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Strict => write!(f, "strict"),
            Self::Regex => write!(f, "regex"),
        }
    }
}

impl Default for MatchingModeEnum {
    fn default() -> MatchingModeEnum {
        Self::Strict
    }
}
