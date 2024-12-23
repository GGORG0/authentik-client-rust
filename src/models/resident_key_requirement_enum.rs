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
pub enum ResidentKeyRequirementEnum {
    #[serde(rename = "discouraged")]
    Discouraged,
    #[serde(rename = "preferred")]
    Preferred,
    #[serde(rename = "required")]
    Required,
}

impl std::fmt::Display for ResidentKeyRequirementEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Discouraged => write!(f, "discouraged"),
            Self::Preferred => write!(f, "preferred"),
            Self::Required => write!(f, "required"),
        }
    }
}

impl Default for ResidentKeyRequirementEnum {
    fn default() -> ResidentKeyRequirementEnum {
        Self::Discouraged
    }
}
