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
pub enum CompatibilityModeEnum {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "slack")]
    Slack,
}

impl std::fmt::Display for CompatibilityModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::Aws => write!(f, "aws"),
            Self::Slack => write!(f, "slack"),
        }
    }
}

impl Default for CompatibilityModeEnum {
    fn default() -> CompatibilityModeEnum {
        Self::Default
    }
}
