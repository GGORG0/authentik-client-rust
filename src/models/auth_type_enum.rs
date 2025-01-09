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
pub enum AuthTypeEnum {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "bearer")]
    Bearer,
}

impl std::fmt::Display for AuthTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Basic => write!(f, "basic"),
            Self::Bearer => write!(f, "bearer"),
        }
    }
}

impl Default for AuthTypeEnum {
    fn default() -> AuthTypeEnum {
        Self::Basic
    }
}
