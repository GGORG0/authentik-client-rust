/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GroupMatchingModeEnum {
    #[serde(rename = "identifier")]
    Identifier,
    #[serde(rename = "name_link")]
    NameLink,
    #[serde(rename = "name_deny")]
    NameDeny,
}

impl std::fmt::Display for GroupMatchingModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Identifier => write!(f, "identifier"),
            Self::NameLink => write!(f, "name_link"),
            Self::NameDeny => write!(f, "name_deny"),
        }
    }
}

impl Default for GroupMatchingModeEnum {
    fn default() -> GroupMatchingModeEnum {
        Self::Identifier
    }
}
