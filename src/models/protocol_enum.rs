/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProtocolEnum {
    #[serde(rename = "rdp")]
    Rdp,
    #[serde(rename = "vnc")]
    Vnc,
    #[serde(rename = "ssh")]
    Ssh,
}

impl std::fmt::Display for ProtocolEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Rdp => write!(f, "rdp"),
            Self::Vnc => write!(f, "vnc"),
            Self::Ssh => write!(f, "ssh"),
        }
    }
}

impl Default for ProtocolEnum {
    fn default() -> ProtocolEnum {
        Self::Rdp
    }
}
