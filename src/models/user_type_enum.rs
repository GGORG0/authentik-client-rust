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
pub enum UserTypeEnum {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "external")]
    External,
    #[serde(rename = "service_account")]
    ServiceAccount,
    #[serde(rename = "internal_service_account")]
    InternalServiceAccount,
}

impl std::fmt::Display for UserTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Internal => write!(f, "internal"),
            Self::External => write!(f, "external"),
            Self::ServiceAccount => write!(f, "service_account"),
            Self::InternalServiceAccount => write!(f, "internal_service_account"),
        }
    }
}

impl Default for UserTypeEnum {
    fn default() -> UserTypeEnum {
        Self::Internal
    }
}
