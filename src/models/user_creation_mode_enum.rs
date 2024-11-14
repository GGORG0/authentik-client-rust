/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserCreationModeEnum {
    #[serde(rename = "never_create")]
    NeverCreate,
    #[serde(rename = "create_when_required")]
    CreateWhenRequired,
    #[serde(rename = "always_create")]
    AlwaysCreate,
}

impl std::fmt::Display for UserCreationModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NeverCreate => write!(f, "never_create"),
            Self::CreateWhenRequired => write!(f, "create_when_required"),
            Self::AlwaysCreate => write!(f, "always_create"),
        }
    }
}

impl Default for UserCreationModeEnum {
    fn default() -> UserCreationModeEnum {
        Self::NeverCreate
    }
}
