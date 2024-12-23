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
pub enum SyncObjectModelEnum {
    #[serde(rename = "authentik.core.models.User")]
    User,
    #[serde(rename = "authentik.core.models.Group")]
    Group,
}

impl std::fmt::Display for SyncObjectModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::User => write!(f, "authentik.core.models.User"),
            Self::Group => write!(f, "authentik.core.models.Group"),
        }
    }
}

impl Default for SyncObjectModelEnum {
    fn default() -> SyncObjectModelEnum {
        Self::User
    }
}
