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
pub enum AuthenticatorAttachmentEnum {
    #[serde(rename = "platform")]
    Platform,
    #[serde(rename = "cross-platform")]
    CrossPlatform,
}

impl std::fmt::Display for AuthenticatorAttachmentEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Platform => write!(f, "platform"),
            Self::CrossPlatform => write!(f, "cross-platform"),
        }
    }
}

impl Default for AuthenticatorAttachmentEnum {
    fn default() -> AuthenticatorAttachmentEnum {
        Self::Platform
    }
}
