/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IssuerModeEnum {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "per_provider")]
    PerProvider,
}

impl std::fmt::Display for IssuerModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Global => write!(f, "global"),
            Self::PerProvider => write!(f, "per_provider"),
        }
    }
}

impl Default for IssuerModeEnum {
    fn default() -> IssuerModeEnum {
        Self::Global
    }
}
