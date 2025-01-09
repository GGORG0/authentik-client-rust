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
pub enum RedirectStageModeEnum {
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "flow")]
    Flow,
}

impl std::fmt::Display for RedirectStageModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Static => write!(f, "static"),
            Self::Flow => write!(f, "flow"),
        }
    }
}

impl Default for RedirectStageModeEnum {
    fn default() -> RedirectStageModeEnum {
        Self::Static
    }
}
