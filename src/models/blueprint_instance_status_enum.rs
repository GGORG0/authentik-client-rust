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
pub enum BlueprintInstanceStatusEnum {
    #[serde(rename = "successful")]
    Successful,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "orphaned")]
    Orphaned,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for BlueprintInstanceStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Successful => write!(f, "successful"),
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
            Self::Orphaned => write!(f, "orphaned"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

impl Default for BlueprintInstanceStatusEnum {
    fn default() -> BlueprintInstanceStatusEnum {
        Self::Successful
    }
}
